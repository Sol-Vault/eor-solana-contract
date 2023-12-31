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
} from "@solana/spl-token";
import { BN } from "bn.js";


describe("pulse-eor", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.PulseEor as Program<PulseEor>;

    const commitment: Commitment = "finalized";
    const connection = new Connection("https://api.devnet.solana.com", {
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
        console.log("Transaction status", status);
    }

    const holdingWalletSeed = "holding-wallet";
    const holdingWalletStateSeed = "holding-state";

    const employee = Keypair.generate();
    const organisationId = "abc";
    const employeeId = "123";
    const organisation = Keypair.generate();

    const holdingWallet = Keypair.generate();
    const holdingWalletAccount = Keypair.generate();

    const holdingWalletPda = PublicKey.findProgramAddressSync(
        [
            Buffer.from(holdingWalletSeed),
            employee.publicKey.toBuffer(),
        ],
        program.programId
    )[0]


    const holdingWalletStatePda = PublicKey.findProgramAddressSync(
        [
            Buffer.from(holdingWalletStateSeed),
            employee.publicKey.toBuffer(),
            anchor.utils.bytes.utf8.encode(organisationId),
        ],
        program.programId
    )[0]

    before(async () => {
        console.log("Airdropping PDA");

        const signature = await connection.requestAirdrop(holdingWalletPda, 100000000000)
        const blockhash = await connection.getLatestBlockhash();
        console.log("Airdrop signature", signature);
        const status = await connection.confirmTransaction({
            signature,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }, commitment);
        console.log("Airdrop status", status);
    });

    before(async () => {
        console.log("Airdropping Employee");

        let signature = await connection.requestAirdrop(employee.publicKey, 100000000000)
        let blockhash = await connection.getLatestBlockhash();
        console.log("Airdrop signature", signature);
        let status = await connection.confirmTransaction({
            signature,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }, commitment);
        console.log("Airdrop status", status);
    })

    before(async () => {
        const orgSignatureAirDrop = await connection.requestAirdrop(organisation.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL)
        await confirmSignature(orgSignatureAirDrop, commitment);
        const mint = Keypair.generate();
        const token = await createMint(
            connection,
            organisation,
            mint.publicKey,
            mint.publicKey,
            10,
        )
    })

    it("Wallet PDA and Wallet State PDA initialized!", async () => {
        const balance = await connection.getBalance(employee.publicKey);
        console.log("Balance", balance);

        try {
            const tx = await program.methods.setupHoldingWallet(
                organisationId
            ).accounts({
                holdingWallet: holdingWalletPda,
                holdingWalletState: holdingWalletStatePda,
                employee: employee.publicKey,
                systemProgram: SystemProgram.programId,
            }).signers([employee]).rpc();
            console.log("Your transaction signature", tx);
        } catch (err) {
            console.log("Transaction error", err);
        }
    });

    it("Transfer 10 Token to Holding Wallet", async () => {
        try {
            // Create mint and transfer mint to organisation

            console.log("Creating mint");
            const mint = Keypair.generate();
            const token = await createMint(
                connection,
                organisation,
                mint.publicKey,
                mint.publicKey,
                10,
            )

            console.log("Creating Org associated token account");
            const organisationATA = await createAssociatedTokenAccount(
                connection,
                organisation,
                token,
                organisation.publicKey,
            )

            console.log("Minting to Org");
            const mintToOrgSignature = await mintTo(
                connection,
                organisation,
                token,
                organisationATA,
                mint,
                10 * 10 ** 10,
            )

            const tokenAccounts3 = await connection.getTokenAccountBalance(
                organisationATA
            );

            console.log("Organisation token account initial", tokenAccounts3.value.uiAmount);

            await confirmSignature(mintToOrgSignature, commitment);

            console.log("Creating holding wallet associated token account");
            // const holdingWalletATAPda = PublicKey.findProgramAddressSync(
            //     [
            //         Buffer.from("holding-wallet-token-account"),
            //         employee.publicKey.toBuffer(),
            //         token.toBuffer(),
            //     ],
            //     program.programId
            // )[0]

            const holdingWalletATAPda = getAssociatedTokenAddressSync(
                token,
                holdingWalletPda,
                true
            )

            const holdingWalletATAInstruction = createAssociatedTokenAccountInstruction(
                organisation.publicKey,
                holdingWalletATAPda,
                holdingWalletPda,
                token,
            )

            console.log("Creating holding wallet associated token account");
            const blockhash = await connection.getLatestBlockhash();
            const transactionMessage = new TransactionMessage(
                {
                    instructions: [holdingWalletATAInstruction],
                    payerKey: organisation.publicKey,
                    recentBlockhash: blockhash.blockhash,
                }
            ).compileToV0Message();

            const transaction = new VersionedTransaction(transactionMessage);

            transaction.sign([organisation]);

            const holdingWalletATASig = await connection.sendRawTransaction(transaction.serialize(), {
                skipPreflight: true,
                preflightCommitment: commitment,
            });

            await confirmSignature(holdingWalletATASig, commitment);

            // await transfer(
            //     connection,
            //     organisation,
            //     organisationATA,
            //     holdingWalletATAPda,
            //     organisation,
            //     1*10**10,
            // )


            try {
                console.log("Transfering to holding wallet");
                const orgtransfer = await program.methods.
                    payOrganisationEmployee(organisationId, employeeId, new BN(3)).
                    accounts({
                        holdingWallet: holdingWalletPda,
                        holdingWalletState: holdingWalletStatePda,
                        employee: employee.publicKey,
                        tokenMint: token,
                        holdingWalletTokenAccount: holdingWalletATAPda,
                        payer: organisation.publicKey,
                        payerTokenAccount: organisationATA,
                        tokenProgram: TOKEN_PROGRAM_ID,
                    }).signers([organisation]).rpc();
                console.log("Your transaction signature", orgtransfer);

                await confirmSignature(orgtransfer, commitment);
                const tokenAccounts2 = await connection.getTokenAccountBalance(
                    organisationATA
                );
                console.log("Organisation token account Post", tokenAccounts2.value.amount);

                const tokenAccoutsHoldingWallets = await connection.getTokenAccountsByOwner(holdingWalletPda, {
                    programId: TOKEN_PROGRAM_ID
                });

                for (let i = 0; i < tokenAccoutsHoldingWallets.value.length; i++) {
                    const tokenBalance = await connection.getTokenAccountBalance(
                        tokenAccoutsHoldingWallets.value[i].pubkey
                    );

                    console.log("Holding wallet token account balance", tokenBalance.value.amount);
                }

                try {

                    const employeeATA = await createAssociatedTokenAccount(
                        connection,
                        employee,
                        token,
                        employee.publicKey,
                    )

                    const withdrawSignature = await program.methods.employeeWithdraw(
                        organisationId,
                        new BN(1),
                    ).accounts({
                        holdingWalletState: holdingWalletStatePda,
                        holdingWallet: holdingWalletPda,
                        holdingWalletTokenAccount: holdingWalletATAPda,
                        withdrawer: employee.publicKey,
                        withdrawerTokenAccount: employeeATA,
                        tokenMint: token,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        systemProgram: SystemProgram.programId,
                    }).signers([employee]).rpc();

                    await confirmSignature(withdrawSignature, commitment);

                    const tokenBalance = await connection.getTokenAccountBalance(
                        employeeATA
                    );
                    console.log("Employee token account balance", tokenBalance.value.amount);
                    
                    const tokenAccoutsHoldingWallets = await connection.getTokenAccountsByOwner(holdingWalletPda, {
                        programId: TOKEN_PROGRAM_ID
                    });
    
                    for (let i = 0; i < tokenAccoutsHoldingWallets.value.length; i++) {
                        const tokenBalance = await connection.getTokenAccountBalance(
                            tokenAccoutsHoldingWallets.value[i].pubkey
                        );
    
                        console.log("Holding wallet token account balance", tokenBalance.value.amount);
                    }
                    
                } catch (err) {
                    console.log("[withdraw from holding] ", err);
                }

            } catch (err) {
                console.log("[payOrganisationEmployee] ", err);
            }
        } catch (err) {
            console.log("[Create mint and transfer to organisation] Transaction error", err);
        }
    })

});
