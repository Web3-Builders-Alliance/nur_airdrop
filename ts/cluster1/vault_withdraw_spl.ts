import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Commitment,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
  BN,
} from "@coral-xyz/anchor";
import { WbaVault, IDL } from "../programs/wba_vault";
import wallet from "./wba-wallet.json";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Commitment
const commitment: Commitment = "finalized";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment,
});

// Create our program
const program = new Program<WbaVault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);

// Create a random keypair
const vaultState = new PublicKey("Gd98uAD23f1wZigYqpKnPcrDxF6EnzraPqys2otcjH4h");

// Create the PDA for our enrollment account

const vaultAuthKeys = [Buffer.from("auth"), vaultState.toBuffer()];
const [vaultAuth, _bump] =
  PublicKey.findProgramAddressSync(
    vaultAuthKeys, program.programId);


// Create the vault key

const vault = [Buffer.from("vault"), vaultAuth.toBuffer()];
const [vaultKey, _bump2] =
  PublicKey.findProgramAddressSync(
    vaultAuthKeys, program.programId);

const token_decimals = 100000000

// Mint address
const mint = new PublicKey("6hnA1FUrbYuadJc4amyxoNXQ9A3rYQLmfPwpFd5ruJ5k");

// Execute our enrollment transaction
(async () => {
  try {
    //Get the token account of the fromWallet address, and if it does not exist, create it
    const ownerAta = await getOrCreateAssociatedTokenAccount
      (connection,
        keypair,
        mint,
        keypair.publicKey
        )

    // Get the token account of the fromWallet address, and if it does not exist, create it
     
    const vaultAta = await getOrCreateAssociatedTokenAccount(
        connection,
        keypair,
        mint,
        vaultAuth,
        true,
        commitment
      );
    
      const signature = await program.methods
    .withdrawSpl(new BN(100* LAMPORTS_PER_SOL))
    .accounts({
      owner:keypair.publicKey,
        
        vaultState,
        vaultAuth,
        systemProgram: SystemProgram.programId,
        ownerAta: ownerAta.address,
        vaultAta: vaultAta.address,
        tokenMint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([
        keypair
    ]).rpc();
    console.log(
      `Withdraw success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
