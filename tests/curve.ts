import * as anchor from "@coral-xyz/anchor";
import type { Curve } from "../target/types/curve";
import {
    Keypair, 
    PublicKey,
    LAMPORTS_PER_SOL,
    } from "@solana/web3.js";
import BN from "bn.js";





describe("curve", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Curve as anchor.Program<Curve>;
    const owner = provider.wallet.payer;
    const user_1 = new Keypair();
    const user_2 = new Keypair();
    const mint = new Keypair();

    const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    const metadata_tank = {
        name: "Tank No Mirror",
        symbol: "WuJing",
        uri: "https://coral-actual-harrier-694.mypinata.cloud/ipfs/bafkreiapee2wldablwgwystkwk4uxvc263hjpvsrztccbmxvtiox5du4wm",
    };
    it.skip("Is initialized!", async () => {
        const txSignature = await program.methods.initialize({
            status: null,
            feeReceiver: owner.publicKey,
            programAuthority: null,
            initialVirtualSol: null,
            initialVirtualToken: null,
            initialRealToken: null,
            tokenSupply: null,
            mintDecimals: null,
            computeScale: null,
            feeBps: null,
            feeDivisor: null,
            migarationFee: null,
            completeReward: null,
            raydiumCfg: null,
            meteoraCfg: null,   
        })
        .accounts({
            authority: owner.publicKey,
        })
        .signers([owner])
        .rpc();
        console.log(`Initialize Success! txSignature:\n${txSignature}`);
    });
    it("Change Admin Params", async () => {
        const txSignature = await program.methods.changeAdminParams({
            status: true,
            feeReceiver: owner.publicKey,
            programAuthority: owner.publicKey,
            initialVirtualSol: new BN(30_000_000_000),
            initialVirtualToken: new BN(1_073_716_769_000_000),
            initialRealToken: new BN(796_969_000_000_000),
            tokenSupply: new BN(1_000_000_000_000_000),
            mintDecimals: 6,
            computeScale: 12,
            feeBps: 100,
            feeDivisor: 10000,
            migarationFee: new BN(15_000_000),
            completeReward: new BN(1_000_000_000),
            raydiumCfg: null,
            meteoraCfg: null, 
        })
        .accounts({
            authority: owner.publicKey,
            newAuthority: null,
        })
        .signers([owner])
        .rpc();
        console.log(`Change Admin Params Success! txSignature:\n${txSignature}`);
    });
    it("Create Pool", async () => {
        // const airdropSignature = await provider.connection.requestAirdrop(
        //     user_1.publicKey,
        //     2 * LAMPORTS_PER_SOL
        // );
        // console.log(`Airdrop Signature: ${airdropSignature}`);

        const [metadataPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("metadata"), METADATA_PROGRAM_ID.toBuffer(), mint.publicKey.toBuffer()],
            METADATA_PROGRAM_ID
        );
        const txSignature = await program.methods.createPool({
            tokenName: metadata_tank.name,
            tokenSymbol: metadata_tank.symbol,
            tokenUri: metadata_tank.uri,
        })
        .accounts({
            creator: owner.publicKey,
            mintAccount: mint.publicKey,
            metadataAccount: metadataPDA,
        })
        .signers([owner, mint])
        .rpc();
        console.log(`Create Pool Success! txSignature:\n${txSignature}`);
    });

    it("Check Admin Cfg", async () => {
        const [adminPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("tick_token_six_nine_admin_config")],
            program.programId
        );
        const adminAccountCfg = await program.account.programAdmin.fetch(adminPDA);
        console.log("Admin Account Cfg:");
        console.log(`fee_bps: ${adminAccountCfg.feeBps}`);
        console.log(`fee_divisor: ${adminAccountCfg.feeDivisor}`);
        console.log(`compute_scale: ${adminAccountCfg.computeScale}`);

    })

    it("Buy", async () => {

        const txSignature = await program.methods.buy({
            accurateInSol: new BN(500_000_000),
            minOutAmount: new BN(100_000_000),
        })
        .accounts({
            user: owner.publicKey,
            feeReceiver: owner.publicKey,
            mintAccount: mint.publicKey,
        })
        .signers([owner])
        .rpc();
        console.log(`Buy Success! txSignature:\n${txSignature}`);
    });
    it("Sell", async () => {
        const txSignature = await program.methods.sell({
            accurateInToken: new BN(2_800_000_000_000),
            minOutAmount: new BN(70_000_000),
        })
        .accounts({
            user: owner.publicKey,
            feeReceiver: owner.publicKey,
            mintAccount: mint.publicKey,
        })
        .signers([owner])
        .rpc();
        console.log(`Sell Success! txSignature:\n${txSignature}`);
    });
})