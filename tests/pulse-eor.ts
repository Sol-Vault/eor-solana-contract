import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PulseEor } from "../target/types/pulse_eor";
import {
    PublicKey,
    SystemProgram,
    Connection,
    Commitment,
    TransactionMessage,
    VersionedTransaction,
    Keypair
} from "@solana/web3.js";

import {
    createMint,
    createAssociatedTokenAccount,
    mintTo,
    TOKEN_PROGRAM_ID,
    createAssociatedTokenAccountInstruction,
    getAssociatedTokenAddressSync,
    createAssociatedTokenAccountIdempotent,
    createAssociatedTokenAccountIdempotentInstruction,
    getAssociatedTokenAddress,
} from "@solana/spl-token";
import { BN } from "bn.js";

describe("pulse-eor", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.PulseEor as Program<PulseEor>;

    const commitment: Commitment = "finalized";
    const connection = new Connection("http://127.0.0.1:8899", {
        commitment,
        wsEndpoint: "ws://localhost:8900/",
    });

    const confirmSignature = async (signature: string, commitment: Commitment) => {
        const blockhash = await connection.getLatestBlockhash();
        const status = await connection.confirmTransaction({
            signature,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }, commitment);
        console.log("Transaction status", JSON.stringify(status));
    }

    const organisationSeed = "organisation";
    const organisationId = "1";
    const employeeId = "1";
    const streamingWalletSeed = "streaming-wallet";
    const employeeContractSeed = "employee-contract";
    const employeeRate = 1

    const organisationWallet = Keypair.generate();
    const employeeWallet = Keypair.generate();
    const employeeContractWallet = Keypair.generate();
    const organisationAccount = PublicKey.findProgramAddressSync(
        [Buffer.from(organisationSeed), Buffer.from(organisationId)],
        program.programId
    );

    const streamingWalletAccount = PublicKey.findProgramAddressSync(
        [Buffer.from(streamingWalletSeed), Buffer.from(organisationId)],
        program.programId
    );

    const employeeContractAccount = PublicKey.findProgramAddressSync(
        [Buffer.from(employeeContractSeed), Buffer.from(organisationId), Buffer.from(employeeId)],
        program.programId
    );

    const adminWallet = Keypair.generate();
    const streamAuthority = Keypair.generate();

    const mint = Keypair.generate();
    let token: PublicKey = new PublicKey("8wTZ9FYDSMnUvbZ2iTVr9ww8k1tBfqcnkZGYGuuYUjWe");

    before(async () => {
        console.log("Airdropping PDA");

        const signature = await connection.requestAirdrop(adminWallet.publicKey, 100000000000)
        const blockhash = await connection.getLatestBlockhash();
        console.log("Airdrop signatureee", signature);
        const status = await connection.confirmTransaction({
            signature,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }, commitment);
        console.log("Airdrop status", status);

        token = await createMint(
            connection,
            adminWallet,
            mint.publicKey,
            mint.publicKey,
            10,
        )
        console.log("Token", token.toBase58());
        
    });

    before(async () => {
        console.log("Airdropping PDA");
        const signature = await connection.requestAirdrop(streamAuthority.publicKey, 100000000000)
        const blockhash = await connection.getLatestBlockhash();
        console.log("Airdrop signatureee", signature);
        const status = await connection.confirmTransaction({
            signature,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }, commitment);
        console.log("Airdrop status", status);
    });

    it("Creates an organisation", async () => {
        try {
            const tx = await program.methods.setupOrganisation(
                organisationId,
            ).accounts({
                organisation: organisationAccount[0],
                streamingWallet: streamingWalletAccount[0],
                admin: adminWallet.publicKey,
                streamAuthority: streamAuthority.publicKey,
                systemProgram: SystemProgram.programId,
            }).signers([
                adminWallet,
                streamAuthority,
            ]).rpc()

            await confirmSignature(tx, commitment);

            console.log("creating employee contract");
            
            const txContract = await program.methods.setupEmployeeContract(
                organisationId,
                employeeId,
                new BN(employeeRate * 10 ** 10),
            ).accounts({
                employeeContract: employeeContractAccount[0],
                organisation: organisationAccount[0],
                payee: employeeWallet.publicKey,
                payer: adminWallet.publicKey,
                systemProgram: SystemProgram.programId,
            }).signers([
                adminWallet,
            ]).rpc()

            await confirmSignature(txContract, commitment);
        } catch (e) {
            console.log(e);
        }
    });

    it("deposit", async () => {
        const streamingWalletATA = getAssociatedTokenAddressSync(
            token,
            streamingWalletAccount[0],
            true
        )

        const createStreamingWalletATA = createAssociatedTokenAccountIdempotentInstruction(
            adminWallet.publicKey,
            streamingWalletATA,
            streamingWalletAccount[0],
            token,
        )

        const blockhash = await connection.getLatestBlockhash();
        const transactionMessage = new TransactionMessage({
            instructions: [createStreamingWalletATA],
            recentBlockhash: blockhash.blockhash,
            payerKey: adminWallet.publicKey,
        }).compileToV0Message();

        const transaction = new VersionedTransaction(transactionMessage)
        transaction.sign([adminWallet]);
        const signature = await connection.sendRawTransaction(transaction.serialize(), {
            skipPreflight: true,
        });

        await confirmSignature(signature, commitment);

        console.log("Minting to Org");
        const mintToOrgSignature = await mintTo(
            connection,
            adminWallet,
            token,
            streamingWalletATA,
            mint,
            10 * 10 ** 10,
        )

        await confirmSignature(mintToOrgSignature, commitment);

        const streamingBalance = await connection.getTokenAccountBalance(
            streamingWalletATA
        );
        console.log("Organisation token account after", streamingBalance.value.uiAmount);
        console.log("Creating holding wallet associated token account");
    })

    it ("streaming", async () => {
        console.log("Streaming");
        
        const streamingWalletTokenAccount = getAssociatedTokenAddressSync(
            token,
            streamingWalletAccount[0],
            true
        )
        const employeeWalletTokenAccount = await getAssociatedTokenAddress(
            token,
            employeeWallet.publicKey
        )

        const createATAInstruction = createAssociatedTokenAccountInstruction(
            adminWallet.publicKey,
            employeeWalletTokenAccount,
            employeeWallet.publicKey,
            token,
        )

        const blockhash1 = await connection.getLatestBlockhash();
        const transactionMessage1 = new TransactionMessage({
            instructions: [createATAInstruction],
            recentBlockhash: blockhash1.blockhash,

            payerKey: adminWallet.publicKey,
        }).compileToV0Message();

        const transaction1 = new VersionedTransaction(transactionMessage1)
        transaction1.sign([adminWallet]);
        const signature1 = await connection.sendRawTransaction(transaction1.serialize(), {
            skipPreflight: true,
        });

        await confirmSignature(signature1, commitment);

        console.log("Signature created employee token account ", signature1);

        const instruction = await program.methods.payContract(
            organisationId,
            employeeId,
            new BN(employeeRate * 10 ** 10),
        ).accounts({
            employeeContract: employeeContractAccount[0],
            organisation: organisationAccount[0],
            streamingWallet: streamingWalletAccount[0],
            streamingWalletTokenAccount: streamingWalletTokenAccount,
            employeeTokenAccount: employeeWalletTokenAccount,
            payer: streamAuthority.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
        }).instruction()

        const blockhash = await connection.getLatestBlockhash();
        const transactionMessage = new TransactionMessage({
            instructions: [instruction],
            recentBlockhash: blockhash.blockhash,
            payerKey: streamAuthority.publicKey,
        }).compileToV0Message();

        const transaction = new VersionedTransaction(transactionMessage)
        transaction.sign([streamAuthority]);
        const sig = await connection.sendRawTransaction(transaction.serialize(), {
            skipPreflight: true,
        });
        await confirmSignature(sig, commitment);

        const transactionDetails = await connection.getTransaction(sig, {
            commitment,
            maxSupportedTransactionVersion: 1,
        })
        console.log("Transaction details", JSON.stringify(transactionDetails));
        const employeeBalance = await connection.getTokenAccountBalance(
            employeeWalletTokenAccount
        );
        console.log("Employee token account after", employeeBalance.value.uiAmount);

        const adminTokenAccount = await getAssociatedTokenAddress(
            token,
            adminWallet.publicKey
        )

        const adminTokenAccountInstrucrtion = createAssociatedTokenAccountIdempotentInstruction(
            adminWallet.publicKey,
            adminTokenAccount,
            adminWallet.publicKey,
            token,
        )

        const blockhash2 = await connection.getLatestBlockhash();
        const transactionMessage2 = new TransactionMessage({
            instructions: [adminTokenAccountInstrucrtion],
            recentBlockhash: blockhash2.blockhash,
            payerKey: adminWallet.publicKey,
        }).compileToV0Message();


        const transaction2 = new VersionedTransaction(transactionMessage2)
        transaction2.sign([adminWallet]);
        const signature2 = await connection.sendRawTransaction(transaction2.serialize(), {
            skipPreflight: true,
        });

        await confirmSignature(signature2, commitment);

        console.log("Withdrawing");
        const withdrawInstruction = await program.methods.withdrawFromStreamWallet(
            organisationId,
            new BN(2 * 10 ** 10),
        ).accounts({
            organisation: organisationAccount[0],
            streamingWallet: streamingWalletAccount[0],
            streamingWalletTokenAccount: streamingWalletTokenAccount,
            payer: adminWallet.publicKey,
            withdraweeTokenAccount: adminTokenAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
        }).instruction()

        const blockhash3 = await connection.getLatestBlockhash();
        const transactionMessage3 = new TransactionMessage({
            instructions: [withdrawInstruction],
            recentBlockhash: blockhash3.blockhash,
            payerKey: adminWallet.publicKey,
        }).compileToV0Message();

        const transaction3 = new VersionedTransaction(transactionMessage3)
        transaction3.sign([adminWallet]);
        const signature3 = await connection.sendRawTransaction(transaction3.serialize(), {
            skipPreflight: true,
        });

        await confirmSignature(signature3, commitment);

        const details = await connection.getTransaction(signature3, {
            commitment,
            maxSupportedTransactionVersion: 1,
        })

        console.log("Transaction details", JSON.stringify(details));

        const adminBalance = await connection.getTokenAccountBalance(
            adminTokenAccount
        );

        console.log("Admin token account after", adminBalance.value.uiAmount);
        
    })

});