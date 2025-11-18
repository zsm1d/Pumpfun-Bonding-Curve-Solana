use anchor_lang::error_code;


#[error_code]
pub enum ProtocolError {
    #[msg("Invalid Admin Authority")]
    InvalidAdminAuthority,

    #[msg("Invalid Program Status")]
    InvalidProgramStatus,

    #[msg("Already Initialized")]
    AlreadyInitialized,

    #[msg("Not Initialized")]
    NotInitialized,

    #[msg("Invalid Mint Decimals")]
    InvalidMintDecimals,

    #[msg("Pool is Completed")]
    PoolComplete,

    #[msg("Invalid Fee Receiver")]
    InvalidFeeReceiver,

    #[msg("Swap Failed: Computation Overflow")]
    Overflow,

    #[msg("Swap Failed: Invalid Amount")]
    InvalidAmount,

    #[msg("Pool Invariant Check is Failed")]
    SwapCheckFailed,

    #[msg("Fee Calculation is Failed")]
    FeeCalculationFailed,

    #[msg("User SOL is Insufficient")]
    UserSolInsufficient,

    #[msg("User Token is Insufficient")]
    UserTokenInsufficient,

    #[msg("Slippage Failed")]
    SlippageFailed,
}