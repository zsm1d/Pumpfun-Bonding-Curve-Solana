# Pump.fun Protocol Replica

### This is a pumpfun smart contract replica, which uses bonding curve to offer token fair launch on Solana.



The program has been published on Solana, the address:
```
CgxWvAx7BgukMMijhmXKur6LhsAMu1CBkYNmi9wTa4wR
```

For publisher:
```rust
pub fn initialize(ctx: Context<Initialize>, params: AdminDataInput) -> Result<()> {
    Initialize::initialize(ctx, params)
}
```
Call `initialize` entry function to initalize the program.


```rust
pub fn change_admin_params(ctx: Context<UpdateAdminParams>, params: AdminDataInput) -> Result<()> {
    UpdateAdminParams::change_data(ctx, params)
}

```
Call `fn change_admin_params` entry function to change admin config.


For devs:
```rust
pub fn create_pool(ctx: Context<CreatePool>, params: CreatePoolParams) -> Result<()> {
    CreatePool::create_pool(ctx, params)
}

```
Call `create_pool` entry function to create a token with a bonding curve pool.


For traders:
```rust
pub fn buy(ctx: Context<Swap>, params: BuyParams) -> Result<()> {
    Swap::buy(ctx, params)
}

pub fn sell(ctx: Context<Swap>, params: SellParams) -> Result<()> {
    Swap::sell(ctx, params)
}

```
Call `buy` & `sell` entry functions to trade tokens.
