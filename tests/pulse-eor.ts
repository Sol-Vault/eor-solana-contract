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
        console.log("Transaction status", status);
    }

    const holdingWalletSeed = "holding-wallet";
    const holdingWalletStateSeed = "holding-state";

    const employee = Keypair.generate();
    const organisationId = "abc";
    const employeeId = "123";

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

    it("Wallet PDA and Wallet State PDA initialized!", async () => {
        const balance = await connection.getBalance(employee.publicKey);
        console.log("Balance", balance);

        try {
            const tx = await program.methods.setupHoldingWallet(
                organisationId
            ).accounts({
                holdingWallet: holdingWalletPda,
                holdingWalletAccount: holdingWalletStatePda,
                employee: employee.publicKey,
                systemProgram: SystemProgram.programId,
            }).signers([employee]).rpc();
            console.log("Your transaction signature", tx);
        } catch (err) {
            console.log("Transaction error", err);
        }
    });

    it("Transfer 10 Token to Holding Wallet", async () => {
        // Set up holding wallet and state
        // try {
        //     const tx = await program.methods.setupHoldingWallet(
        //         organisationId
        //     ).accounts({
        //         holdingWallet: holdingWalletPda,
        //         holdingWalletAccount: holdingWalletStatePda,
        //         employee: employee.publicKey,
        //         systemProgram: SystemProgram.programId,
        //     }).signers([employee]).rpc();
        // } catch (err) {
        //     console.log("[Set up holding wallet] Transaction error", err);
        // }

        try {
            // Create mint and transfer mint to organisation
            const organisation = Keypair.generate();
            const orgSignatureAirDrop = await connection.requestAirdrop(organisation.publicKey, 100000000000)
            await confirmSignature(orgSignatureAirDrop, commitment);

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

            await confirmSignature(mintToOrgSignature, commitment);

            console.log("Creating holding wallet associated token account");
            const holdingWalletATAPda = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("holding-wallet-token-account"),
                    employee.publicKey.toBuffer(),
                    token.toBuffer(),
                ],
                program.programId
            )[0]

            try {
                console.log("Transfering to holding wallet");
                const orgtransfer = await program.methods.
                payOrganisationEmployee(organisationId, employeeId, new BN(10 * 10 ** 10)).
                accounts({
                    holdingWallet: holdingWalletPda,
                    holdingWalletState: holdingWalletStatePda,
                    employee: employee.publicKey,
                    tokenMint: token,
                    holdingWalletTokenAccount: holdingWalletATAPda,
                    payer: organisation.publicKey,
                    payerTokenAccount: organisationATA,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                }).signers([organisation]).rpc();

                console.log("Your transaction signature", orgtransfer);
            await confirmSignature(orgtransfer, commitment);
            } catch (err) {
                console.log("[payOrganisationEmployee] ", err);
            }
        } catch (err) {
            console.log("[Create mint and transfer to organisation] Transaction error", err);
        }
    })
});
