use anchor_lang::prelude::*;
// use crate::state::events::{ToEvent, AdminParamsUpdated};

// #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone, Copy, Debug, PartialEq, Eq)]
// pub enum ProgramStatus {
//     Running,
//     Swap,
//     Paused,
// }

#[account]
#[derive(InitSpace, Debug)]
pub struct ProgramAdmin {
    pub status: bool,
    pub initialized: bool,
    pub fee_receiver: Pubkey,
    pub program_authority: Pubkey,
    pub initial_virtual_sol: u64,
    pub initial_virtual_token: u64,
    pub initial_real_token: u64,
    pub token_supply: u64,
    pub mint_decimals: u8,
    pub compute_scale: u8,
    pub fee_bps: u8,
    pub fee_divisor: u16,
    pub migaration_fee: u64,
    pub complete_reward: u64,
    pub raydium_cfg: Pubkey,
    pub meteora_cfg: Pubkey,
}


impl Default for ProgramAdmin {
    fn default () -> Self {
        Self {
            status: false,
            initialized: false,
            program_authority: Pubkey::default(),
            fee_receiver: Pubkey::default(),
            initial_virtual_sol: 30_000_000_000,
            initial_virtual_token: 1_073_716_769_000_000,
            initial_real_token: 796_969_000_000_000,
            token_supply: 1_000_000_000_000_000,
            mint_decimals: 6,
            compute_scale: 12,
            fee_bps: 100,
            fee_divisor: 10000,
            migaration_fee: 15_000_000,
            complete_reward: 1_000_000_000,
            raydium_cfg: Pubkey::default(),
            meteora_cfg: Pubkey::default(),
        }
    }
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AdminDataInput {
    pub status: Option<bool>,
    pub program_authority: Option<Pubkey>,
    pub fee_receiver: Option<Pubkey>,
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



impl ProgramAdmin {
    pub fn update_admin_params(
        &mut self,
        params: AdminDataInput,
    ){
        if let Some(status) = params.status {
            self.status = status;
        }
        if let Some(program_authority) = params.program_authority {
            self.program_authority = program_authority;
        }
        if let Some(fee_receiver) = params.fee_receiver {
            self.fee_receiver = fee_receiver;
        }
        if let Some(initial_virtual_sol) = params.initial_virtual_sol {
            self.initial_virtual_sol = initial_virtual_sol;
        }
        if let Some(initial_virtual_token) = params.initial_virtual_token {
            self.initial_virtual_token = initial_virtual_token;
        }
        if let Some(initial_real_token) = params.initial_real_token {
            self.initial_real_token = initial_real_token;
        }
        if let Some(token_supply) = params.token_supply {
            self.token_supply = token_supply;
        }
        if let Some(mint_decimals) = params.mint_decimals {
            self.mint_decimals = mint_decimals;
        }
        if let Some(compute_scale) = params.compute_scale {
            self.compute_scale = compute_scale;
        }
        if let Some(fee_bps) = params.fee_bps {
            self.fee_bps = fee_bps;
        }
        if let Some(fee_divisor) = params.fee_divisor {
            self.fee_divisor = fee_divisor;
        }
        if let Some(migaration_fee) = params.migaration_fee {
            self.migaration_fee = migaration_fee;
        }
        if let Some(complete_reward) = params.complete_reward {
            self.complete_reward = complete_reward;
        }
        if let Some(raydium_cfg) = params.raydium_cfg {
            self.raydium_cfg = raydium_cfg;
        }
        if let Some(meteora_cfg) = params.meteora_cfg {
            self.meteora_cfg = meteora_cfg;
        }
    }
}



// impl ToEvent<AdminParamsUpdated> for ProgramAdmin {
//     fn to_event(&self) -> AdminParamsUpdated {
//         AdminParamsUpdated {
//             status: Some(self.status),
//             fee_receiver: Some(self.fee_receiver),
//             program_authority: Some(self.program_authority),
//             initial_virtual_sol: Some(self.initial_virtual_sol),
//             initial_virtual_token: Some(self.initial_virtual_token),
//             initial_real_token: Some(self.initial_real_token),
//             token_supply: Some(self.token_supply),
//             mint_decimals: Some(self.mint_decimals),
//             compute_scale: Some(self.compute_scale),
//             fee_bps: Some(self.fee_bps),
//             fee_divisor: Some(self.fee_divisor),
//             migaration_fee: Some(self.migaration_fee),
//             raydium_cfg: Some(self.raydium_cfg),
//             meteora_cfg: Some(self.meteora_cfg),
//         }
//     }
// }
