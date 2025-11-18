use anchor_lang::prelude::*;
use crate::state::{
    admin::*,
    errors::ProtocolError,
    events::AdminParamsUpdated,
};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateAdminParams<'info> {

    #[account(
        mut,
        constraint = authority.key() == admin.program_authority.key() @ ProtocolError::InvalidAdminAuthority,
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = admin.initialized == true @ ProtocolError::NotInitialized,
        seeds = [b"tick_token_six_nine_admin_config"],
        bump,
    )]
    pub admin: Box<Account<'info, ProgramAdmin>>,

    /// CHECK: New Authority account, validated by program
    #[account(mut)]
    pub new_authority: Option<UncheckedAccount<'info>>,

    pub system_program: Program<'info, System>,
}



impl<'info> UpdateAdminParams<'info> {
    pub fn change_data(ctx: Context<UpdateAdminParams>, params: AdminDataInput) -> Result<()> {
        let admin = &mut ctx.accounts.admin;
        admin.update_admin_params(params.clone());
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