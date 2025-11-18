// import { PublicKey } from "@solana/web3.js";
// import { BN } from "@coral-xyz/anchor";


// export const CURVE_PROGRAM_ID = new PublicKey("CgxWvAx7BgukMMijhmXKur6LhsAMu1CBkYNmi9wTa4wR");
// export const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
// export const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
// export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
// export const SYSTEM_PROGRAM_ID = new PublicKey("11111111111111111111111111111111");

// export const ADMIN_SEED = "tick_token_admin_config";
// export const POOL_SEED = "tick_token_virtual_pool";
// export const EVENT_AUTHORITY_SEED = "__event_authority";
// export const ADMIN_AUTHORITY = "Eqj7erVLFTz5CLFDS1jYhSYkNb73eFHf21Vd57Etiwu9";

// export const INITIAL_V_SOL = new BN(30_000_000_000);
// export const INITIAL_V_TOKEN = new BN(1_073_000_000_000_000_000);
// export const INITIAL_R_TOKEN = new BN(793_100_000_000_000);
// export const token_supply = new BN(1_000_000_000_000_000_000);

// export const TOKEN_DECIMALS = 6;
// export const SOL_DECIMALS = 9;

// export const FEE_BASIS_POINTS = 100;
// export const BASIS_POINTS_DIVISOR = 10000;

// export const DEFAULT_MIGRATION_FEE = new BN(1_000_000_000);


// // ============================================================================
// // 测试配置
// // ============================================================================

// /**
//  * 默认滑点容忍度（5% = 500 basis points）
//  */
// export const DEFAULT_SLIPPAGE_BPS = 500;

// /**
//  * 测试用的最小SOL购买量（0.01 SOL）
//  */
// export const MIN_BUY_SOL = new BN(10_000_000);

// /**
//  * 测试用的标准SOL购买量（1 SOL）
//  */
// export const STANDARD_BUY_SOL = new BN(1_000_000_000);

// /**
//  * 测试用的大额SOL购买量（10 SOL）
//  */
// export const LARGE_BUY_SOL = new BN(10_000_000_000);

// // ============================================================================
// // 数学常量
// // ============================================================================

// /**
//  * 零值BN
//  */
// export const ZERO = new BN(0);

// /**
//  * 一值BN
//  */
// export const ONE = new BN(1);

// // ============================================================================
// // 程序状态枚举
// // ============================================================================

// /**
//  * 程序状态枚举（与合约保持一致）
//  */
// export const ProgramStatus = {
//   Running: { running: {} },
//   Swap: { swap: {} },
//   Paused: { paused: {} },
// } as const;