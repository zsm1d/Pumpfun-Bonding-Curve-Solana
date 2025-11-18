// /**
//  * 测试辅助函数文件（Anchor + UMI 混合方式）
//  * 
//  * 提供所有测试需要的辅助函数，包括：
//  * - UMI实例创建
//  * - PDA派生（使用UMI）
//  * - 文件上传（UMI独有）
//  * - 账户查询
//  * - 数学计算
//  * 
//  * 这些函数设计为可复用，之后可以直接复制到SDK中
//  */

// import * as anchor from "@coral-xyz/anchor";
// import { BN, Program } from "@coral-xyz/anchor";
// import {
//   PublicKey,
//   Keypair,
//   SystemProgram,
//   LAMPORTS_PER_SOL,
//   Connection,
// } from "@solana/web3.js";
// import { Curve } from "../target/types/curve";
// import {
//   ADMIN_SEED,
//   POOL_SEED,
//   EVENT_AUTHORITY_SEED,
//   CURVE_PROGRAM_ID,
//   METADATA_PROGRAM_ID,
//   FEE_BASIS_POINTS,
//   BASIS_POINTS_DIVISOR,
//   DEFAULT_SLIPPAGE_BPS,
// } from "./constants";

// // UMI imports
// import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
// import { mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
// import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
// import {
//   findMetadataPda,
//   findAssociatedTokenPda,
// } from "@metaplex-foundation/mpl-token-metadata";
// import {
//   fromWeb3JsPublicKey,
//   toWeb3JsPublicKey,
//   fromWeb3JsKeypair,
// } from "@metaplex-foundation/umi-web3js-adapters";
// import {
//   generateSigner,
//   signerIdentity,
//   createGenericFile,
//   percentAmount,
//   Umi,
// } from "@metaplex-foundation/umi";

// // 新的Pinata版本
// import pinataSDK from '@pinata/sdk';
// import fs from 'fs';
// import path from 'path';

// // ============================================================================
// // UMI 实例管理
// // ============================================================================

// /**
//  * 创建UMI实例
//  * @param connection Solana连接
//  * @returns UMI实例
//  */
// export function createUmiInstance(connection: Connection): Umi {
//   const umi = createUmi(connection.rpcEndpoint)
//     .use(mplTokenMetadata())
//     .use(irysUploader());
  
//   return umi;
// }

// /**
//  * 设置UMI使用指定的keypair
//  * @param umi UMI实例
//  * @param keypair Web3.js Keypair
//  */
// export function setUmiSigner(umi: Umi, keypair: Keypair): void {
//   const umiSigner = fromWeb3JsKeypair(keypair);
//   umi.use(signerIdentity(umiSigner));
// }

// // ============================================================================
// // PDA 派生函数（使用UMI）
// // ============================================================================

// /**
//  * 派生Admin账户PDA
//  * @param programId 程序ID
//  * @returns [PDA公钥, bump]
//  */
// export function getAdminPDA(programId: PublicKey = CURVE_PROGRAM_ID): [PublicKey, number] {
//   return PublicKey.findProgramAddressSync(
//     [Buffer.from(ADMIN_SEED)],
//     programId
//   );
// }

// /**
//  * 派生Pool账户PDA
//  * @param mint Token Mint地址
//  * @param programId 程序ID
//  * @returns [PDA公钥, bump]
//  */
// export function getPoolPDA(
//   mint: PublicKey,
//   programId: PublicKey = CURVE_PROGRAM_ID
// ): [PublicKey, number] {
//   return PublicKey.findProgramAddressSync(
//     [Buffer.from(POOL_SEED), mint.toBuffer()],
//     programId
//   );
// }

// /**
//  * 派生Metadata账户PDA（使用UMI）
//  * @param umi UMI实例
//  * @param mint Token Mint地址
//  * @returns Metadata PDA公钥
//  */
// export function getMetadataPDA(umi: Umi, mint: PublicKey): PublicKey {
//   const umiMint = fromWeb3JsPublicKey(mint);
//   const [metadataPda] = findMetadataPda(umi, { mint: umiMint });
//   return toWeb3JsPublicKey(metadataPda);
// }

// /**
//  * 派生Event Authority PDA
//  * @param programId 程序ID
//  * @returns [PDA公钥, bump]
//  */
// export function getEventAuthorityPDA(
//   programId: PublicKey = CURVE_PROGRAM_ID
// ): [PublicKey, number] {
//   return PublicKey.findProgramAddressSync(
//     [Buffer.from(EVENT_AUTHORITY_SEED)],
//     programId
//   );
// }

// /**
//  * 获取Associated Token Account地址（使用UMI）
//  * @param umi UMI实例
//  * @param owner 所有者地址
//  * @param mint Token Mint地址
//  * @returns ATA地址
//  */
// export function getATA(umi: Umi, owner: PublicKey, mint: PublicKey): PublicKey {
//   const umiOwner = fromWeb3JsPublicKey(owner);
//   const umiMint = fromWeb3JsPublicKey(mint);
//   const [ata] = findAssociatedTokenPda(umi, { mint: umiMint, owner: umiOwner });
//   return toWeb3JsPublicKey(ata);
// }

// // ============================================================================
// // 文件上传函数（UMI独有功能）
// // ============================================================================

// // /**
// //  * 上传图片到Arweave
// //  * @param umi UMI实例
// //  * @param imagePath 图片文件路径
// //  * @returns 图片URI
// //  */
// // export async function uploadImage(
// //   umi: Umi,
// //   imagePath: string
// // ): Promise<string> {
// //   console.log(`📤 上传图片: ${imagePath}`);
  
// //   const imageFile = fs.readFileSync(imagePath);
// //   const extension = imagePath.split('.').pop()?.toLowerCase() || 'png';
// //   const mimeType = extension === 'jpg' || extension === 'jpeg' 
// //     ? 'image/jpeg' 
// //     : `image/${extension}`;
  
// //   const umiImageFile = createGenericFile(imageFile, `image.${extension}`, {
// //     tags: [{ name: "Content-Type", value: mimeType }],
// //   });
  
// //   const [imageUri] = await umi.uploader.upload([umiImageFile]);
// //   console.log(`✅ 图片已上传: ${imageUri}`);
  
// //   return imageUri;
// // }

// /**
//  * 使用Pinata上传图片到IPFS
//  * @param imagePath 图片文件路径
//  * @returns IPFS URI
//  */
// export async function uploadImage(
//   imagePath: string
// ): Promise<string> {
//   console.log(`📤 上传图片到Pinata: ${imagePath}`);
  
//   // 从环境变量获取Pinata凭证
//   const pinataApiKey = process.env.PINATA_API_KEY;
//   const pinataSecretApiKey = process.env.PINATA_SECRET_API_KEY;
  
//   if (!pinataApiKey || !pinataSecretApiKey) {
//     console.log("⚠️  未配置Pinata凭证，使用占位符");
//     return "https://gateway.pinata.cloud/ipfs/QmPlaceholderImage";
//   }
  
//   const pinata = new pinataSDK(pinataApiKey, pinataSecretApiKey);
  
//   // 测试连接
//   try {
//     await pinata.testAuthentication();
//   } catch (error) {
//     console.log("❌ Pinata认证失败，使用占位符");
//     return "https://gateway.pinata.cloud/ipfs/QmPlaceholderImage";
//   }
  
//   // 上传文件
//   const readableStream = fs.createReadStream(imagePath);
//   const options = {
//     pinataMetadata: {
//       name: path.basename(imagePath),
//     },
//   };
  
//   const result = await pinata.pinFileToIPFS(readableStream, options);
//   const imageUri = `https://gateway.pinata.cloud/ipfs/${result.IpfsHash}`;
  
//   console.log(`✅ 图片已上传到Pinata: ${imageUri}`);
//   return imageUri;
// }


// // /**
// //  * 上传metadata JSON到Arweave
// //  * @param umi UMI实例
// //  * @param metadata Metadata对象
// //  * @returns Metadata URI
// //  */
// // export async function uploadMetadata(
// //   umi: Umi,
// //   metadata: {
// //     name: string;
// //     symbol: string;
// //     description: string;
// //     image: string;
// //   }
// // ): Promise<string> {
// //   console.log(`📤 上传metadata: ${metadata.name}`);
  
// //   const metadataUri = await umi.uploader.uploadJson(metadata);
// //   console.log(`✅ Metadata已上传: ${metadataUri}`);
  
// //   return metadataUri;
// // }


// // 新的Pinata版本
// /**
//  * 使用Pinata上传metadata JSON到IPFS
//  * @param metadata Metadata对象
//  * @returns Metadata URI
//  */
// export async function uploadMetadata(
//   metadata: {
//     name: string;
//     symbol: string;
//     description: string;
//     image: string;
//   }
// ): Promise<string> {
//   console.log(`📤 上传metadata到Pinata: ${metadata.name}`);
  
//   const pinataApiKey = process.env.PINATA_API_KEY;
//   const pinataSecretApiKey = process.env.PINATA_SECRET_API_KEY;
  
//   if (!pinataApiKey || !pinataSecretApiKey) {
//     console.log("⚠️  未配置Pinata凭证，使用占位符");
//     return "https://gateway.pinata.cloud/ipfs/QmPlaceholderMetadata";
//   }
  
//   const pinata = new pinataSDK(pinataApiKey, pinataSecretApiKey);
  
//   // 上传JSON
//   const options = {
//     pinataMetadata: {
//       name: `${metadata.symbol}-metadata.json`,
//     },
//   };
  
//   const result = await pinata.pinJSONToIPFS(metadata, options);
//   const metadataUri = `https://gateway.pinata.cloud/ipfs/${result.IpfsHash}`;
  
//   console.log(`✅ Metadata已上传到Pinata: ${metadataUri}`);
//   return metadataUri;
// }


// /**
//  * 完整的Token metadata准备流程（使用Pinata上传）
//  * @param tokenName Token名称
//  * @param tokenSymbol Token符号
//  * @param imagePath 图片路径（可选）
//  * @returns Metadata URI
//  */
// export async function prepareTokenMetadata(
//   tokenName: string,
//   tokenSymbol: string,
//   imagePath?: string
// ): Promise<string> {
//   let imageUri: string;

//   if (imagePath && fs.existsSync(imagePath)) {
//     // 上传真实图片到Pinata
//     imageUri = await uploadImage(imagePath);
//   } else {
//     // 使用占位符
//     console.log("⚠️  未提供图片，使用占位符");
//     imageUri = "https://gateway.pinata.cloud/ipfs/QmPlaceholderImage";
//   }

//   // 上传metadata到Pinata
//   const metadata = {
//     name: tokenName,
//     symbol: tokenSymbol,
//     description: `${tokenName} is a meme token on Solana`,
//     image: imageUri,
//   };

//   const metadataUri = await uploadMetadata(metadata);
//   return metadataUri;
// }

// // ============================================================================
// // 账户查询函数
// // ============================================================================

// /**
//  * 获取Admin账户数据
//  * @param program Anchor程序实例
//  * @returns Admin账户数据
//  */
// export async function getAdminAccount(program: Program<Curve>) {
//   const [adminPDA] = getAdminPDA(program.programId);
//   return await program.account.programAdmin.fetch(adminPDA);
// }

// /**
//  * 获取Pool账户数据
//  * @param program Anchor程序实例
//  * @param mint Token Mint地址
//  * @returns Pool账户数据
//  */
// export async function getPoolAccount(program: Program<Curve>, mint: PublicKey) {
//   const [poolPDA] = getPoolPDA(mint, program.programId);
//   return await program.account.poolData.fetch(poolPDA);
// }

// /**
//  * 检查账户是否存在
//  * @param connection Solana连接
//  * @param address 账户地址
//  * @returns 是否存在
//  */
// export async function accountExists(
//   connection: Connection,
//   address: PublicKey
// ): Promise<boolean> {
//   const accountInfo = await connection.getAccountInfo(address);
//   return accountInfo !== null;
// }

// /**
//  * 获取SOL余额
//  * @param connection Solana连接
//  * @param address 账户地址
//  * @returns SOL余额（lamports）
//  */
// export async function getSOLBalance(
//   connection: Connection,
//   address: PublicKey
// ): Promise<number> {
//   return await connection.getBalance(address);
// }

// /**
//  * 获取Token余额
//  * @param connection Solana连接
//  * @param ata ATA地址
//  * @returns Token余额
//  */
// export async function getTokenBalance(
//   connection: Connection,
//   ata: PublicKey
// ): Promise<BN> {
//   try {
//     const accountInfo = await connection.getTokenAccountBalance(ata);
//     return new BN(accountInfo.value.amount);
//   } catch (error) {
//     // ATA不存在，返回0
//     return new BN(0);
//   }
// }

// // ============================================================================
// // 数学计算函数（Bonding Curve）
// // ============================================================================

// /**
//  * 计算买入时获得的Token数量（不含手续费）
//  * 使用恒定乘积公式：x * y = k
//  */
// export function calculateBuyTokenAmount(
//   solIn: BN,
//   virtualSolReserve: BN,
//   virtualTokenReserve: BN
// ): BN {
//   const k = virtualSolReserve.mul(virtualTokenReserve);
//   const newSolReserve = virtualSolReserve.add(solIn);
//   const newTokenReserve = k.div(newSolReserve);
//   const tokenOut = virtualTokenReserve.sub(newTokenReserve);
//   return tokenOut;
// }

// /**
//  * 计算卖出时获得的SOL数量（不含手续费）
//  */
// export function calculateSellSOLAmount(
//   tokenIn: BN,
//   virtualSolReserve: BN,
//   virtualTokenReserve: BN
// ): BN {
//   const k = virtualSolReserve.mul(virtualTokenReserve);
//   const newTokenReserve = virtualTokenReserve.add(tokenIn);
//   const newSolReserve = k.div(newTokenReserve);
//   const solOut = virtualSolReserve.sub(newSolReserve);
//   return solOut;
// }

// /**
//  * 计算手续费
//  */
// export function calculateFee(
//   amount: BN,
//   feeBasisPoints: number = FEE_BASIS_POINTS
// ): BN {
//   return amount.mul(new BN(feeBasisPoints)).div(new BN(BASIS_POINTS_DIVISOR));
// }

// /**
//  * 计算最小输出（含滑点保护）
//  */
// export function calculateMinOutAmount(
//   expectedOut: BN,
//   slippageBps: number = DEFAULT_SLIPPAGE_BPS
// ): BN {
//   const slippage = expectedOut
//     .mul(new BN(slippageBps))
//     .div(new BN(BASIS_POINTS_DIVISOR));
//   return expectedOut.sub(slippage);
// }

// /**
//  * 计算买入的完整流程（含手续费）
//  */
// export function calculateBuyWithFee(
//   solIn: BN,
//   virtualSolReserve: BN,
//   virtualTokenReserve: BN
// ): { tokenOut: BN; fee: BN; solAfterFee: BN } {
//   const fee = calculateFee(solIn);
//   const solAfterFee = solIn.sub(fee);
//   const tokenOut = calculateBuyTokenAmount(
//     solAfterFee,
//     virtualSolReserve,
//     virtualTokenReserve
//   );
//   return { tokenOut, fee, solAfterFee };
// }

// /**
//  * 计算卖出的完整流程（含手续费）
//  */
// export function calculateSellWithFee(
//   tokenIn: BN,
//   virtualSolReserve: BN,
//   virtualTokenReserve: BN
// ): { solOut: BN; fee: BN; solBeforeFee: BN } {
//   const solBeforeFee = calculateSellSOLAmount(
//     tokenIn,
//     virtualSolReserve,
//     virtualTokenReserve
//   );
//   const fee = calculateFee(solBeforeFee);
//   const solOut = solBeforeFee.sub(fee);
//   return { solOut, fee, solBeforeFee };
// }

// // ============================================================================
// // 辅助工具函数
// // ============================================================================

// /**
//  * 空投SOL到指定地址（仅用于测试）
//  */
// export async function airdrop(
//   connection: Connection,
//   address: PublicKey,
//   amount: number = 10
// ): Promise<void> {
//   const signature = await connection.requestAirdrop(
//     address,
//     amount * LAMPORTS_PER_SOL
//   );
//   await connection.confirmTransaction(signature);
// }

// /**
//  * 等待指定毫秒数
//  */
// export function sleep(ms: number): Promise<void> {
//   return new Promise((resolve) => setTimeout(resolve, ms));
// }

// /**
//  * 格式化SOL数量
//  */
// export function formatSOL(lamports: BN | number): string {
//   const amount = typeof lamports === "number" ? lamports : lamports.toNumber();
//   return (amount / LAMPORTS_PER_SOL).toFixed(9);
// }

// /**
//  * 格式化Token数量
//  */
// export function formatToken(amount: BN | number, decimals: number = 6): string {
//   const amt = typeof amount === "number" ? amount : amount.toNumber();
//   return (amt / Math.pow(10, decimals)).toFixed(decimals);
// }

// /**
//  * 生成随机Token名称
//  */
// export function randomTokenName(): string {
//   const adjectives = ["Super", "Mega", "Ultra", "Hyper", "Epic", "Doge", "Moon"];
//   const nouns = ["Coin", "Token", "Meme", "Pepe", "Cat", "Dog", "Rocket"];
//   const adj = adjectives[Math.floor(Math.random() * adjectives.length)];
//   const noun = nouns[Math.floor(Math.random() * nouns.length)];
//   return `${adj} ${noun}`;
// }

// /**
//  * 生成随机Token符号
//  */
// export function randomTokenSymbol(): string {
//   const chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
//   let symbol = "";
//   for (let i = 0; i < 3 + Math.floor(Math.random() * 2); i++) {
//     symbol += chars.charAt(Math.floor(Math.random() * chars.length));
//   }
//   return symbol;
// }

// /**
//  * 打印分隔线
//  */
// export function printSeparator(title?: string): void {
//   console.log("\n" + "=".repeat(80));
//   if (title) {
//     console.log(`  ${title}`);
//     console.log("=".repeat(80));
//   }
// }

// /**
//  * 打印账户信息
//  */
// export function printAccount(label: string, address: PublicKey): void {
//   console.log(`${label}: ${address.toBase58()}`);
// }