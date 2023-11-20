import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "./dev-wallet.json"

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        // We're going to claim 2 devnet SOL tokens
        const txhash = await
            connection.requestAirdrop(new PublicKey("6K3AAjt4TAfQYAfBjhdKKiztJh275UzPRcSM9aabXpSy"), 2 * LAMPORTS_PER_SOL);
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();