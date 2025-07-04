import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://cool-virulent-needle.solana-devnet.quiknode.pro/89e3ee41b126264357803999a0327ca14e33f106/');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));


(async () => {
    try {
        //1. Load image
        const imageFile = await readFile('/home/div/Downloads/generug.png');
        //2. Convert image to generic file.
        const genericFile = createGenericFile(imageFile,"generug.png",{displayName:"Random NFT" , contentType:"img/png"});
        //3. Upload image
        // const image = ???

        const [myUri] = await umi.uploader.upload([genericFile]);
        const txId = myUri.split('/').pop();
        const irysDevnetUri = `https://devnet.irys.xyz/${txId}`;
        console.log("Irys Gateway URI:", irysDevnetUri);
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
