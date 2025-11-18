use anchor_lang::prelude::*;
use crate::state::{
    admin::ProgramAdmin,
    errors::ProtocolError,
};


#[derive(Debug, Clone)]
pub struct BuyResult {
    pub token_amount: u64,
    pub sol_amount: u64,
}

#[derive(Debug, Clone)]
pub struct SellResult {
    pub token_amount: u64,
    pub sol_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreatePoolParams {
    pub token_name: String,
    pub token_symbol: String,
    pub token_uri: String,
}


#[account]
#[derive(InitSpace, Debug)]
pub struct PoolData {
    pub virtual_liquidity_sol: u64,
    pub virtual_liquidity_token: u64,
    pub liquidity_sol: u64,
    pub liquidity_token: u64,
    pub token_supply: u64,
    pub decimals_sol: u8,
    pub decimals_token: u8,
    pub k_scale: u8,
    pub complete: bool,
    pub creator: Pubkey,
    pub bump: u8,
}


impl PoolData {
    #[inline(always)]
    pub fn get_signer<'a>(key: &'a Pubkey, bump: &'a u8) -> [&'a [u8]; 3] {
        [
            b"tick_token_virtual_pool",
            key.as_ref(),
            std::slice::from_ref(bump),
        ]
    }


    pub fn init_data(&mut self, admin_cfg: &ProgramAdmin, creator: Pubkey, bump: u8) {
        self.virtual_liquidity_sol = admin_cfg.initial_virtual_sol;
        self.virtual_liquidity_token = admin_cfg.initial_virtual_token;
        self.liquidity_sol = 0;
        self.liquidity_token = admin_cfg.initial_real_token;
        self.token_supply = admin_cfg.token_supply;
        self.decimals_sol = 9;
        self.decimals_token = admin_cfg.mint_decimals;
        self.k_scale = admin_cfg.compute_scale;
        self.complete = false;
        self.creator = creator;
        self.bump = bump;
    }


    pub fn calculate_fee(admin_cfg: &ProgramAdmin, amount: u64) -> Result<u64> {
        let bps = admin_cfg.fee_bps;
        let points_divisor = admin_cfg.fee_divisor;

        let sol_fee = (amount as u128).checked_mul(bps as u128)
            .ok_or(ProtocolError::FeeCalculationFailed)?
            .checked_div(points_divisor as u128)
            .ok_or(ProtocolError::FeeCalculationFailed)?
            .try_into()
            .map_err(|_| ProtocolError::FeeCalculationFailed)?;
        
        Ok(sol_fee)
    }


    pub fn compute_top_virtual_sol(&self, admin: &ProgramAdmin) -> Result<u64> {
        const SOL_DECIMALS: u32 = 9;
        let token_decimals = self.decimals_token;
        let k_scale = self.k_scale;

        let final_token = admin.initial_virtual_token
            .checked_sub(admin.initial_real_token)
            .ok_or(ProtocolError::Overflow)?;
        
        let final_token_scaled = (final_token as u128)
            .checked_mul(10u128.pow(k_scale as u32 - token_decimals as u32))
            .ok_or(ProtocolError::Overflow)?;

        let initial_v_sol_scaled = (admin.initial_virtual_sol as u128)
            .checked_mul(10u128.pow(k_scale as u32 - SOL_DECIMALS))
            .ok_or(ProtocolError::Overflow)?;

        let initial_v_token_scaled = (admin.initial_virtual_token as u128)
            .checked_mul(10u128.pow(k_scale as u32 - token_decimals as u32))
            .ok_or(ProtocolError::Overflow)?;

        let product_k = initial_v_sol_scaled
            .checked_mul(initial_v_token_scaled)
            .ok_or(ProtocolError::Overflow)?;

        let virtual_sol_final_scaled = product_k.checked_div(final_token_scaled)
            .ok_or(ProtocolError::Overflow)?;

        let virtual_sol_final = virtual_sol_final_scaled
            .checked_div(10u128.pow(k_scale as u32 - SOL_DECIMALS))
            .ok_or(ProtocolError::Overflow)?
            .try_into()
            .map_err(|_| ProtocolError::Overflow)?;
        Ok(virtual_sol_final)
    }


    pub fn compute_buy(&mut self, sol_amount: u64) -> Result<u64> {

        require_gt!(sol_amount, 0, ProtocolError::InvalidAmount);

        const SOL_DECIMALS: u32 = 9;
        let token_decimals = self.decimals_token;
        let k_scale = self.k_scale;

        let sol_amount_scaled = (sol_amount as u128)
            .checked_mul(10u128.pow(k_scale as u32 - SOL_DECIMALS))
            .ok_or(ProtocolError::Overflow)?;

        let virtual_sol_scaled = (self.virtual_liquidity_sol as u128)
            .checked_mul(10u128.pow(k_scale as u32 - SOL_DECIMALS))
            .ok_or(ProtocolError::Overflow)?;

        let virtual_token_scaled = (self.virtual_liquidity_token as u128)
            .checked_mul(10u128.pow(k_scale as u32 - token_decimals as u32))
            .ok_or(ProtocolError::Overflow)?;

        let product_k = virtual_sol_scaled
            .checked_mul(virtual_token_scaled)
            .ok_or(ProtocolError::Overflow)?;
        msg!("Compute buy: Product K scaled (12 decimals): {}", product_k);

        let new_virtual_liquidity_sol_scaled = virtual_sol_scaled
            .checked_add(sol_amount_scaled)
            .ok_or(ProtocolError::Overflow)?;
        msg!("Compute buy: New virtual sol reserve scaled (12 decimals): {}", new_virtual_liquidity_sol_scaled);

        let new_virtual_liquidity_token_scaled = product_k
            .checked_div(new_virtual_liquidity_sol_scaled)
            .ok_or(ProtocolError::Overflow)?;
        msg!("Compute buy: New virtual token reserve scaled (12 decimals): {}", new_virtual_liquidity_token_scaled);

        let token_received_scaled = virtual_token_scaled
            .checked_sub(new_virtual_liquidity_token_scaled)
            .ok_or(ProtocolError::Overflow)?;

        let token_received = token_received_scaled
            .checked_div(10u128.pow(k_scale as u32 - token_decimals as u32))
            .ok_or(ProtocolError::Overflow)?;

        let receive = token_received.try_into().map_err(|_| ProtocolError::Overflow)?;
        msg!("Compute buy: Token received: {}", receive);

        Ok(receive)
    }

    pub fn compute_sell(&mut self, token_amount: u64) -> Result<u64> {

        require_gt!(token_amount, 0, ProtocolError::InvalidAmount);

        const SOL_DECIMALS: u32 = 9;
        let token_decimals = self.decimals_token;
        let k_scale = self.k_scale;

        let token_amount_scaled = (token_amount as u128)
            .checked_mul(10u128.pow(k_scale as u32 - token_decimals as u32))
            .ok_or(ProtocolError::Overflow)?;

        let virtual_sol_scaled = (self.virtual_liquidity_sol as u128)
            .checked_mul(10u128.pow(k_scale as u32 - SOL_DECIMALS))
            .ok_or(ProtocolError::Overflow)?;

        let virtual_token_scaled = (self.virtual_liquidity_token as u128)
            .checked_mul(10u128.pow(k_scale as u32 - token_decimals as u32))
            .ok_or(ProtocolError::Overflow)?;

        let product_k = virtual_sol_scaled
            .checked_mul(virtual_token_scaled)
            .ok_or(ProtocolError::Overflow)?;
        msg!("Compute sell: Product K scaled (12 decimals): {}", product_k);

        let new_virtual_liquidity_token_scaled = virtual_token_scaled
            .checked_add(token_amount_scaled)
            .ok_or(ProtocolError::Overflow)?;
        msg!("Compute sell: New virtual token reserve scaled (12 decimals): {}", new_virtual_liquidity_token_scaled);

        let new_virtual_liquidity_sol_scaled = product_k
            .checked_div(new_virtual_liquidity_token_scaled)
            .ok_or(ProtocolError::Overflow)?;
        msg!("Compute sell: New virtual sol reserve scaled (12 decimals): {}", new_virtual_liquidity_sol_scaled);

        let sol_received_scaled = virtual_sol_scaled
            .checked_sub(new_virtual_liquidity_sol_scaled)
            .ok_or(ProtocolError::Overflow)?;

        let sol_received = sol_received_scaled
            .checked_div(10u128.pow(k_scale as u32 - SOL_DECIMALS))
            .ok_or(ProtocolError::Overflow)?;

        let receive = sol_received.try_into().map_err(|_| ProtocolError::Overflow)?;
        msg!("Compute sell: Sol received: {}", receive);

        Ok(receive)
    }


    pub fn execute_buy(&mut self, mut sol_amount: u64, admin: &ProgramAdmin) -> Result<BuyResult> {

        let mut token_amount = self.compute_buy(sol_amount)?;

        if token_amount > self.liquidity_token {

            let token_received = self.liquidity_token;
            let final_vir_liquidity_sol = self.compute_top_virtual_sol(admin)?;
            let cr_virtual_tokan_reserve = self.virtual_liquidity_token;

            let new_liquidity_token = (self.liquidity_token as u128).checked_sub(token_received as u128)
                .ok_or(ProtocolError::Overflow)?;
            msg!("Execute buy: New real token reserve: {}", new_liquidity_token);
            let new_vir_liquidity_token = (cr_virtual_tokan_reserve as u128).checked_sub(token_received as u128)
                .ok_or(ProtocolError::Overflow)?;
            msg!("Execute buy: New virtual token reserve: {}", new_vir_liquidity_token);

            self.virtual_liquidity_token = new_vir_liquidity_token.try_into().map_err(|_| ProtocolError::Overflow)?;
            self.virtual_liquidity_sol = final_vir_liquidity_sol;
            
            let recp_sol_amount = self.compute_sell(token_received)?;
            msg!("Execute buy: Sol amount is overflowed and New sol amount: {}", recp_sol_amount);

            let new_liquidity_sol = (self.liquidity_sol as u128).checked_add(recp_sol_amount as u128)
                .ok_or(ProtocolError::Overflow)?;
            msg!("Execute buy: New real sol reserve: {}", new_liquidity_sol);
            msg!("Execute buy: New virtual sol reserve: {}", final_vir_liquidity_sol);

            self.liquidity_token = new_liquidity_token.try_into().map_err(|_| ProtocolError::Overflow)?;
            self.liquidity_sol = new_liquidity_sol.try_into().map_err(|_| ProtocolError::Overflow)?;

            sol_amount = recp_sol_amount;
            token_amount = token_received;

            self.complete = true;
            msg!("Execute buy: Virtual Pool is completed");
            
        } else {
            let new_liquidity_token = (self.liquidity_token as u128).checked_sub(token_amount as u128)
                .ok_or(ProtocolError::Overflow)?;
            msg!("Execute buy: New real token reserve: {}", new_liquidity_token);

            let new_vir_liquidity_token = (self.virtual_liquidity_token as u128).checked_sub(token_amount as u128)
                .ok_or(ProtocolError::Overflow)?;
            msg!("Execute buy: New virtual token reserve: {}", new_vir_liquidity_token);

            let new_liquidity_sol = (self.liquidity_sol as u128).checked_add(sol_amount as u128)
                .ok_or(ProtocolError::Overflow)?;
            msg!("Execute buy: New real sol reserve: {}", new_liquidity_sol);

            let new_vir_liquidity_sol = (self.virtual_liquidity_sol as u128).checked_add(sol_amount as u128)
                .ok_or(ProtocolError::Overflow)?;
            msg!("Execute buy: New virtual sol reserve: {}", new_vir_liquidity_sol);

            self.liquidity_token = new_liquidity_token.try_into().map_err(|_| ProtocolError::Overflow)?;
            self.virtual_liquidity_token = new_vir_liquidity_token.try_into().map_err(|_| ProtocolError::Overflow)?;
            self.liquidity_sol = new_liquidity_sol.try_into().map_err(|_| ProtocolError::Overflow)?;
            self.virtual_liquidity_sol = new_vir_liquidity_sol.try_into().map_err(|_| ProtocolError::Overflow)?;
            
            if self.liquidity_token == 0 {
                self.complete = true;
                msg!("Execute buy: Virtual Pool is completed");
            }
        }

        Ok(BuyResult {
            token_amount,
            sol_amount,
        })
    }


    pub fn execute_sell(&mut self, token_amount: u64) -> Result<SellResult> {

        let sol_amount = self.compute_sell(token_amount)?;

        let new_liquidity_sol = (self.liquidity_sol as u128).checked_sub(sol_amount as u128)
            .ok_or(ProtocolError::Overflow)?;
        msg!("Execute sell: New real sol reserve: {}", new_liquidity_sol);

        let new_vir_liquidity_sol = (self.virtual_liquidity_sol as u128).checked_sub(sol_amount as u128)
            .ok_or(ProtocolError::Overflow)?;
        msg!("Execute sell: New virtual sol reserve: {}", new_vir_liquidity_sol);

        let new_liquidity_token = (self.liquidity_token as u128).checked_add(token_amount as u128)
            .ok_or(ProtocolError::Overflow)?;
        msg!("Execute sell: New real token reserve: {}", new_liquidity_token);

        let new_vir_liquidity_token = (self.virtual_liquidity_token as u128).checked_add(token_amount as u128)
            .ok_or(ProtocolError::Overflow)?;
        msg!("Execute sell: New virtual token reserve: {}", new_vir_liquidity_token);

        self.liquidity_sol = new_liquidity_sol.try_into().map_err(|_| ProtocolError::Overflow)?;
        self.virtual_liquidity_sol = new_vir_liquidity_sol.try_into().map_err(|_| ProtocolError::Overflow)?;
        self.liquidity_token = new_liquidity_token.try_into().map_err(|_| ProtocolError::Overflow)?;
        self.virtual_liquidity_token = new_vir_liquidity_token.try_into().map_err(|_| ProtocolError::Overflow)?;

        Ok(SellResult {
            token_amount,
            sol_amount,
        })
    }

}