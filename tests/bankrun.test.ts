import { describe, it } from "node:test";
import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { startAnchor } from "solana-bankrun";
import BN from "bn.js";
import IDL from "../target/idl/curve.json" with { type: "json" };
import type { Curve } from "../target/types/curve";
import * as fs from "fs";
import * as path from "path";

const PROGRAM_ID = new PublicKey(IDL.address);
const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

describe("bankrun test", async () => {

    const context = await startAnchor(
        "",
        [   
            { name: "curve", programId: PROGRAM_ID },
            { name: "token_metadata", programId: METADATA_PROGRAM_ID },
        ],
        [],
    );

    const provider = new BankrunProvider(context);
    const program = new anchor.Program<Curve>(IDL, provider);
    const payer = provider.wallet.payer;
    const mintKeypair = new Keypair();

    const metadata_tank = {
        name: "Tank No Mirror",
        symbol: "WuJing",
        uri: "https://coral-actual-harrier-694.mypinata.cloud/ipfs/bafkreiapee2wldablwgwystkwk4uxvc263hjpvsrztccbmxvtiox5du4wm",
    };

    it("initialize", async () => {
        const txSignature = await program.methods.initialize({
            status: null,
            feeReceiver: payer.publicKey,
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
            authority: payer.publicKey,
        })
        .signers([payer])
        .rpc();

        console.log(`Initialize Success! txSignature:\n${txSignature}`);
    });

    it("Change Admin Params", async () => {
        const txSignature = await program.methods.changeAdminParams({
            status: null,
            feeReceiver: null,
            programAuthority: null,
            initialVirtualSol: new BN(30_000_000_000),
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
            authority: payer.publicKey,
            newAuthority: null,
        })
        .signers([payer])
        .rpc();

        console.log(`Change Admin Params Success! txSignature:\n${txSignature}`);
    });

    it("Create Pool", async () => {
        const [metadataPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("metadata"), METADATA_PROGRAM_ID.toBuffer(), mintKeypair.publicKey.toBuffer()],
            METADATA_PROGRAM_ID
        );
        const txSignature = await program.methods.createPool({
            tokenName: metadata_tank.name,
            tokenSymbol: metadata_tank.symbol,
            tokenUri: metadata_tank.uri,
        })
        .accounts({
            creator: payer.publicKey,
            mintAccount: mintKeypair.publicKey,
            metadataAccount: metadataPDA,
        })
        .signers([payer, mintKeypair])
        .rpc();

        console.log(`Create Pool Success! txSignature:\n${txSignature}`);
    });

    it("Buy", async () => {
        const txSignature = await program.methods.buy({
            accurateInSol: new BN(1_000_000_000),
            minOutAmount: new BN(900_000_000),
        })
        .accounts({
            user: payer.publicKey,
            feeReceiver: payer.publicKey,
            mintAccount: mintKeypair.publicKey,
        })
        .signers([payer])
        .rpc();

        console.log(`Buy Success! txSignature:\n${txSignature}`);
    });

    it("Sell", async () => {
        const txSignature = await program.methods.sell({
            accurateInToken: new BN(2_800_000_000_000),
            minOutAmount: new BN(70_000_000),
        })
        .accounts({
            user: payer.publicKey,
            feeReceiver: payer.publicKey,
            mintAccount: mintKeypair.publicKey,
        })
        .signers([payer])
        .rpc();

        console.log(`Sell Success! txSignature:\n${txSignature}`);
    });
    

})