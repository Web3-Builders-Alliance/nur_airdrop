import {
    Transaction, SystemProgram, Connection, Keypair,
    LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey
} from
    "@solana/web3.js"

import wallet from "./dev-wallet.json";


const from = Keypair.fromSecretKey(new Uint8Array(wallet));
// Define our WBA public key
const to = new
    PublicKey("GLtaTaYiTQrgz411iPJD79rsoee59HhEy18rtRdrhEUJ");

//Create a Solana devnet connection
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        // Get balance of dev wallet
        const balance = await connection.getBalance(from.publicKey);

        console.log(`Balance: ${balance}`);
       

    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();