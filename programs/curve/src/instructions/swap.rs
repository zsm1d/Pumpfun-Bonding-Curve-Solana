use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token;
use anchor_spl::token::Transfer;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{
    errors::ProtocolError, admin::*, pool::PoolData, events::*
};



#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BuyParams {
    pub accurate_in_sol: u64,
    pub min_out_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SellParams {
    pub accurate_in_token: u64,
    pub min_out_amount: u64,
}


#[event_cpi]
#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Fee receiver account, validated by program
    #[account(mut)]
    pub fee_receiver: AccountInfo<'info>,

    #[account(mut)]
    pub mint_account: Box<Account<'info, Mint>>,

    #[account(
        seeds = [b"tick_token_six_nine_admin_config"],
        bump,
    )]
    pub admin: Box<Account<'info, ProgramAdmin>>,

    #[account(
        mut,
        seeds = [b"tick_token_virtual_pool", mint_account.key().as_ref()],
        constraint = pool.complete == false @ ProtocolError::PoolComplete,
        bump,
    )]
    pub pool: Box<Account<'info, PoolData>>,

    #[account(mut,
        associated_token::mint = mint_account,
        associated_token::authority = pool,
    )]
    pub pool_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_account,
        associated_token::authority = user,
    )]
    pub user_ata: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}



impl<'info> Swap<'info> {
    pub fn buy(ctx: Context<Swap>, params: BuyParams) -> Result<()> {

        require_eq!(ctx.accounts.admin.status, true, ProtocolError::InvalidProgramStatus);
        require!(ctx.accounts.fee_receiver.key() == ctx.accounts.admin.fee_receiver, ProtocolError::InvalidFeeReceiver);

        let BuyParams { accurate_in_sol, min_out_amount } = params;
        require_gt!(accurate_in_sol, 0, ProtocolError::InvalidAmount);

        if ctx.accounts.pool.complete {
            return err!(ProtocolError::PoolComplete);
        }
        require!(ctx.accounts.user.get_lamports() >= accurate_in_sol, ProtocolError::UserSolInsufficient);

        let fee_lamports = PoolData::calculate_fee(&ctx.accounts.admin, accurate_in_sol)?;
        let sol_amount_minus_fee = accurate_in_sol.checked_sub(fee_lamports)
            .ok_or(ProtocolError::FeeCalculationFailed)?;

        let buy_result = ctx.accounts.pool
            .execute_buy(sol_amount_minus_fee, &ctx.accounts.admin)?;

        require!(buy_result.token_amount >= min_out_amount, ProtocolError::SlippageFailed);


        let cpi_context_to_pool = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.pool.to_account_info(),
            },
        );
        system_program::transfer(cpi_context_to_pool, buy_result.sol_amount)?;
        msg!("Sol to pool transfer complete");

        let cpi_context_to_fee = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.fee_receiver.to_account_info(),
            },
        );
        system_program::transfer(cpi_context_to_fee, fee_lamports)?;
        msg!("Fee transfer complete");

        let mint_key = ctx.accounts.mint_account.key();
        let bump = ctx.bumps.pool;
        let signer = PoolData::get_signer(&mint_key, &bump);
        let signer_seeds = &[&signer[..]];
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.pool_ata.to_account_info(),
                    to: ctx.accounts.user_ata.to_account_info(),
                    authority: ctx.accounts.pool.to_account_info(),
                }, 
            signer_seeds),
            buy_result.token_amount,
        )?;
        msg!("Token to user transfer complete");

        Self::invariant_check(&mut ctx.accounts.pool, &mut ctx.accounts.pool_ata)?;

        
        emit_cpi!(SwapEvent {
            is_buy: true,
            user: ctx.accounts.user.key(),
            user_ata: ctx.accounts.user_ata.key(),
            pool: ctx.accounts.pool.key(),
            pool_ata: ctx.accounts.pool_ata.key(),
            sol_amount: buy_result.sol_amount,
            token_amount: buy_result.token_amount,
            virtual_liquidity_sol: ctx.accounts.pool.virtual_liquidity_sol,
            virtual_liquidity_token: ctx.accounts.pool.virtual_liquidity_token,
            liquidity_sol: ctx.accounts.pool.liquidity_sol,
            liquidity_token: ctx.accounts.pool.liquidity_token,
        });

        if ctx.accounts.pool.complete {
            emit_cpi!(CompleteEvent {
                pool: ctx.accounts.pool.key(),
                pool_ata: ctx.accounts.pool_ata.key(),
                complete: ctx.accounts.pool.complete,
                virtual_liquidity_sol: ctx.accounts.pool.virtual_liquidity_sol,
                virtual_liquidity_token: ctx.accounts.pool.virtual_liquidity_token,
                liquidity_sol: ctx.accounts.pool.liquidity_sol,
                liquidity_token: ctx.accounts.pool.liquidity_token,
            });
        }

        Ok(())
    }


    pub fn sell(ctx: Context<Swap>, params: SellParams) -> Result<()> {

        require_eq!(ctx.accounts.admin.status, true, ProtocolError::InvalidProgramStatus);
        require!(ctx.accounts.fee_receiver.key() == ctx.accounts.admin.fee_receiver, ProtocolError::InvalidFeeReceiver);

        let SellParams { accurate_in_token, min_out_amount } = params;
        require_gt!(accurate_in_token, 0, ProtocolError::InvalidAmount);
        
        if ctx.accounts.pool.complete {
            return err!(ProtocolError::PoolComplete);
        }
        require!(ctx.accounts.user_ata.amount >= accurate_in_token, ProtocolError::UserTokenInsufficient);

        let sell_result = ctx.accounts.pool.execute_sell(accurate_in_token)?;

        require!(sell_result.sol_amount >= min_out_amount, ProtocolError::SlippageFailed);

        let fee_lamports = PoolData::calculate_fee(&ctx.accounts.admin, sell_result.sol_amount)?;
        let sol_received = sell_result.sol_amount.checked_sub(fee_lamports)
            .ok_or(ProtocolError::FeeCalculationFailed)?;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_ata.to_account_info(),
                    to: ctx.accounts.pool_ata.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            sell_result.token_amount,
        )?;
        msg!("Token to pool transfer complete");

        ctx.accounts.pool.sub_lamports(sol_received)?;
        ctx.accounts.user.add_lamports(sol_received)?;
        msg!("Sol to user transfer complete");

        ctx.accounts.pool.sub_lamports(fee_lamports)?;
        ctx.accounts.fee_receiver.add_lamports(fee_lamports)?;
        msg!("Fee transfer complete");

        Self::invariant_check(&mut ctx.accounts.pool, &mut ctx.accounts.pool_ata)?;

        emit_cpi!(SwapEvent {
            is_buy: false,
            user: ctx.accounts.user.key(),
            user_ata: ctx.accounts.user_ata.key(),
            pool: ctx.accounts.pool.key(),
            pool_ata: ctx.accounts.pool_ata.key(),
            sol_amount: sell_result.sol_amount,
            token_amount: sell_result.token_amount,
            virtual_liquidity_sol: ctx.accounts.pool.virtual_liquidity_sol,
            virtual_liquidity_token: ctx.accounts.pool.virtual_liquidity_token,
            liquidity_sol: ctx.accounts.pool.liquidity_sol,
            liquidity_token: ctx.accounts.pool.liquidity_token,
        });

        if ctx.accounts.pool.complete {
            emit_cpi!(CompleteEvent {
                pool: ctx.accounts.pool.key(),
                pool_ata: ctx.accounts.pool_ata.key(),
                complete: ctx.accounts.pool.complete,
                virtual_liquidity_sol: ctx.accounts.pool.virtual_liquidity_sol,
                virtual_liquidity_token: ctx.accounts.pool.virtual_liquidity_token,
                liquidity_sol: ctx.accounts.pool.liquidity_sol,
                liquidity_token: ctx.accounts.pool.liquidity_token,
            });
        }

        Ok(())
    }


    pub fn invariant_check(pool: &mut Account<PoolData>, pool_ata: &mut Account<TokenAccount>) -> Result<()> {

        if pool_ata.owner != pool.key() {
            msg!("Check Failed: Invalid Pool ATA");
            return err!(ProtocolError::SwapCheckFailed);
        }
        pool_ata.reload()?;


        let pool_rent: u64 = Rent::get()?.minimum_balance(8 + PoolData::INIT_SPACE as usize)
            .try_into().map_err(|_| ProtocolError::SwapCheckFailed)?;
        let pool_real_sol = pool.get_lamports() - pool_rent;
        require_gte!(pool_real_sol, pool.liquidity_sol, ProtocolError::SwapCheckFailed);

        let pool_ata_amount = pool_ata.amount;
        require_gte!(pool_ata_amount, pool.liquidity_token, ProtocolError::SwapCheckFailed);

        Ok(())
    }

    pub fn calculate_fee(&self, amount: u64) -> Result<u64> {
        let bps = self.admin.fee_bps;
        let points_diviso = self.admin.fee_divisor;

        let sol_fee = (amount as u128).checked_mul(bps as u128)
            .ok_or(ProtocolError::FeeCalculationFailed)?
            .checked_div(points_diviso as u128)
            .ok_or(ProtocolError::FeeCalculationFailed)?
            .try_into()
            .map_err(|_| ProtocolError::FeeCalculationFailed)?;
        
        Ok(sol_fee)
    }
}