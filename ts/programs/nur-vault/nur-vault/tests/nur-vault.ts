import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NurVault } from "../target/types/nur_vault";


describe("nur-vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NurVault as Program<NurVault>;
  
  const vault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("nur-vault")], program.programId);

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        signer: anchor.getProvider().publicKey,
        vault: vault[0], // Pass the first element of the vault array
      })
      .rpc();
    console.log("Your transaction signature", tx);
    console.log("Vault balance", (await anchor.getProvider().connection.getBalance(vault[0])).toString());
  });
});
