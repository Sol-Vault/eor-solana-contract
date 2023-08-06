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

describe("pulse-eor", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.PulseEor as Program<PulseEor>;

    const commitment: Commitment = "finalized";
    const connection = new Connection("http://127.0.0.1:8899", {
        commitment,
        wsEndpoint: "ws://localhost:8900/",
    });
    const holdingWalletSeed = "holding-wallet";
    const holdingWalletStateSeed = "holding-state";

    const employee = Keypair.generate();
    const organisationId = "abc";

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

    it("Is initialized!", async () => {
        let signature = await connection.requestAirdrop(employee.publicKey, 100000000000)
        let blockhash = await connection.getLatestBlockhash();
        console.log("Airdrop signature", signature);
        let status = await connection.confirmTransaction({
            signature,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }, commitment);
        console.log("Airdrop status", status);

        signature = await connection.requestAirdrop(holdingWalletPda, 100000000000)
        blockhash = await connection.getLatestBlockhash();
        console.log("Airdrop signature", signature);
        status = await connection.confirmTransaction({
            signature,
            blockhash: blockhash.blockhash,
            lastValidBlockHeight: blockhash.lastValidBlockHeight,
        }, commitment);
        console.log("Airdrop status", status);
        
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
});
