use anchor_lang::prelude::*;
use crate::state::{
    admin::*,
    errors::ProtocolError,
    events::AdminParamsUpdated,
};


pub const AUTHORIZED_ADMIN: &str = "Eqj7erVLFTz5CLFDS1jYhSYkNb73eFHf21Vd57Etiwu9";


#[event_cpi]
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        mut,
        constraint = authority.key().to_string() == AUTHORIZED_ADMIN @ ProtocolError::InvalidAdminAuthority,
    )]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        constraint = admin.initialized == false @ ProtocolError::AlreadyInitialized,
        space = 8 + ProgramAdmin::INIT_SPACE,
        seeds = [b"tick_token_six_nine_admin_config"],
        bump,
    )]
    pub admin: Box<Account<'info, ProgramAdmin>>,

    pub system_program: Program<'info, System>,
}



impl<'info> Initialize<'info> {
    pub fn initialize(ctx: Context<Initialize>, params: AdminDataInput) -> Result<()> {
        let admin = &mut ctx.accounts.admin;
        
        admin.update_admin_params(params.clone());
        admin.status = true;
        admin.initialized = true;
        admin.program_authority = ctx.accounts.authority.key();

        emit_cpi!(AdminParamsUpdated {
            status: Some(admin.status),
            fee_receiver: Some(admin.fee_receiver),
            program_authority: Some(admin.program_authority),
            initial_virtual_sol: Some(admin.initial_virtual_sol),
            initial_virtual_token: Some(admin.initial_virtual_token),
            initial_real_token: Some(admin.initial_real_token),
            token_supply: Some(admin.token_supply),
            mint_decimals: Some(admin.mint_decimals),
            compute_scale: Some(admin.compute_scale),
            fee_bps: Some(admin.fee_bps),
            fee_divisor: Some(admin.fee_divisor),
            migaration_fee: Some(admin.migaration_fee),
            complete_reward: Some(admin.complete_reward),
            raydium_cfg: Some(admin.raydium_cfg),
            meteora_cfg: Some(admin.meteora_cfg),
        });
        Ok(())
    }
}