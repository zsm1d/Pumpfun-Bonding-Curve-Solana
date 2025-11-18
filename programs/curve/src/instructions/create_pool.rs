use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
    CreateMetadataAccountsV3, Metadata, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,

    },
    token::{Mint, MintTo, Token, TokenAccount, mint_to},
};
use crate:: state::{
    events::InitData, admin::*, pool::PoolData, pool::CreatePoolParams, errors::ProtocolError,
};


#[event_cpi]
#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,


    #[account(
        seeds = [b"tick_token_six_nine_admin_config"],
        bump,
    )]
    pub admin: Box<Account<'info, ProgramAdmin>>,

    #[account(
        init,
        signer,
        payer = creator,
        mint::decimals = admin.mint_decimals,
        mint::authority = pool,
        mint::freeze_authority = pool,
    )]
    pub mint_account: Box<Account<'info, Mint>>,

    /// CHECK: Metadata account, validated by Metaplex program
    #[account(
        mut
    )]
    pub metadata_account: UncheckedAccount<'info>,


    #[account(
        init,
        payer = creator,
        seeds = [b"tick_token_virtual_pool", mint_account.key().as_ref()],
        bump,
        space = 8 + PoolData::INIT_SPACE,
    )]
    pub pool: Box<Account<'info, PoolData>>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = mint_account,
        associated_token::authority = pool,
    )]
    pub pool_ata: Box<Account<'info, TokenAccount>>,


    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}




impl<'info> CreatePool<'info> {
    pub fn create_mint(&mut self, params: &CreatePoolParams, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        msg!("Creating metadata mint account...");
        
        let cpi_context = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: self.metadata_account.to_account_info(),
                mint: self.mint_account.to_account_info(),
                mint_authority: self.pool.to_account_info(),
                update_authority: self.pool.to_account_info(),
                payer: self.creator.to_account_info(),
                system_program: self.system_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
            signer_seeds,
        );

        let token_data = DataV2 {
            name: params.token_name.clone(),
            symbol: params.token_symbol.clone(),
            uri: params.token_uri.clone(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(cpi_context, token_data, false, false, None)?;

        msg!("Metadata mint account created");
        Ok(())
    }


    pub fn create_pool(ctx: Context<CreatePool>, params: CreatePoolParams) -> Result<()> {

        require_eq!(ctx.accounts.admin.status, true, ProtocolError::InvalidProgramStatus);

        msg!("Creating pool account...");

        let bump = ctx.bumps.pool;
        let creator = ctx.accounts.creator.key();

        ctx.accounts.pool.init_data(&ctx.accounts.admin, creator, bump);
        let mint_key = ctx.accounts.mint_account.key();

        let signer = PoolData::get_signer(&mint_key, &bump);
        let signer_seeds = &[&signer[..]];

        ctx.accounts.create_mint(&params, signer_seeds)?;

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.pool_ata.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            signer_seeds,
        );
        mint_to(cpi_context, ctx.accounts.pool.token_supply)?;

        let pool = &mut ctx.accounts.pool;
        emit_cpi!(InitData {
            name: params.token_name,
            symbol: params.token_symbol,
            uri: params.token_uri,
            mint_account: mint_key,
            creator: pool.creator,
            virtual_liquidity_sol: pool.virtual_liquidity_sol,
            virtual_liquidity_token: pool.virtual_liquidity_token,
            liquidity_sol: pool.liquidity_sol,
            liquidity_token: pool.liquidity_token,
            token_supply: pool.token_supply,
        });

        msg!("Pool account created");
        Ok(())
        
    }

}