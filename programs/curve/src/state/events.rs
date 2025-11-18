use anchor_lang::prelude::*;
// use crate::state::admin::ProgramStatus;

#[event]
pub struct AdminParamsUpdated {
    pub status: Option<bool>,
    pub fee_receiver: Option<Pubkey>,
    pub program_authority: Option<Pubkey>,
    pub initial_virtual_sol: Option<u64>,
    pub initial_virtual_token: Option<u64>,
    pub initial_real_token: Option<u64>,
    pub token_supply: Option<u64>,
    pub mint_decimals: Option<u8>,
    pub compute_scale: Option<u8>,
    pub fee_bps: Option<u8>,
    pub fee_divisor: Option<u16>,
    pub migaration_fee: Option<u64>,
    pub complete_reward: Option<u64>,
    pub raydium_cfg: Option<Pubkey>,
    pub meteora_cfg: Option<Pubkey>,
}

#[event]
pub struct InitData {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint_account: Pubkey,
    pub creator: Pubkey,
    pub virtual_liquidity_sol: u64,
    pub virtual_liquidity_token: u64,
    pub liquidity_sol: u64,
    pub liquidity_token: u64,
    pub token_supply: u64,
}

#[event]
pub struct SwapEvent {
    pub is_buy: bool,
    pub user: Pubkey,
    pub user_ata: Pubkey,
    pub pool: Pubkey,
    pub pool_ata: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub virtual_liquidity_sol: u64,
    pub virtual_liquidity_token: u64,
    pub liquidity_sol: u64,
    pub liquidity_token: u64,
}

#[event]
pub struct CompleteEvent {
    pub pool: Pubkey,
    pub pool_ata: Pubkey,
    pub complete: bool,
    pub virtual_liquidity_sol: u64,
    pub virtual_liquidity_token: u64,
    pub liquidity_sol: u64,
    pub liquidity_token: u64,
}



// pub trait ToEvent<T: anchor_lang::Event> {
//     fn to_event(&self) -> T;
// }