import { Commitment, Connection, Keypair, PublicKey, Transaction, Signer } from "@solana/web3.js";
import wallet from "./wba-wallet.json";
import { createMetadataAccountV3 } from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";



// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Define our Mint address
const mint = new PublicKey("Gd98uAD23f1wZigYqpKnPcrDxF6EnzraPqys2otcjH4h");

// Add the Token Metadata Program
const token_metadata_program_id = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

// Create PDA for token metadata
const metadata_seeds = [
  Buffer.from("metadata"),
  token_metadata_program_id.toBuffer(),
  mint.toBuffer(),
];
const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(
  metadata_seeds,
  token_metadata_program_id
);

const umi = createUmi(connection);
const umiKeypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signerKeypair = createSignerFromKeypair(umi, umiKeypair);
umi.use(signerIdentity(signerKeypair));

(async () => {
  try {
    // Start here
    const metadataTx = createMetadataAccountV3(umi, {
      mint: publicKey(mint),
      mintAuthority: signerKeypair,
      updateAuthority: publicKey(keypair.publicKey),
      data: {
        name: "WBA",
        symbol: "WBA",
        uri: "https://arweave.net/euAlBrhc3NQJ5Q-oJnP10vsQFjTV7E9CgHZcVm8cogo",
        sellerFeeBasisPoints: 0,
        creators: null,
        collection: null,
        uses: null,
      },
      isMutable: true,
      collectionDetails: null,
    });

    const tx = await metadataTx.buildAndSign(umi);
    const signature = await umi.rpc.sendTransaction(tx);
    console.log(signature);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();