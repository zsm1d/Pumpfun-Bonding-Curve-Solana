use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
use instructions::*;
use state::*;




declare_id!("CgxWvAx7BgukMMijhmXKur6LhsAMu1CBkYNmi9wTa4wR");

#[program]
pub mod curve {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: AdminDataInput) -> Result<()> {
        Initialize::initialize(ctx, params)
    }

    pub fn change_admin_params(ctx: Context<UpdateAdminParams>, params: AdminDataInput) -> Result<()> {
        UpdateAdminParams::change_data(ctx, params)
    }

    pub fn create_pool(ctx: Context<CreatePool>, params: CreatePoolParams) -> Result<()> {
        CreatePool::create_pool(ctx, params)
    }

    pub fn buy(ctx: Context<Swap>, params: BuyParams) -> Result<()> {
        Swap::buy(ctx, params)
    }

    pub fn sell(ctx: Context<Swap>, params: SellParams) -> Result<()> {
        Swap::sell(ctx, params)
    }
}
