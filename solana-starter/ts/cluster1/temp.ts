import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import wallet from "../wba-wallet.json"
const connection = new Connection("https://api.devnet.solana.com");
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

(async () => {
    await connection.requestAirdrop(keypair.publicKey,4 * LAMPORTS_PER_SOL);
    console.log("public key is : ",keypair.publicKey);
})();