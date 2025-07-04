import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = 'https://devnet.irys.xyz/Hjh724LDcSemAuwn1EHRrDoKRRVa8vGgShDqPw3Zm1bf';
        const metadata = {
            name: "Random NFT",
            symbol: "RN",
            description: "This is Random NFT that is for educational purpose",
            image: image,
            attributes: [
                { trait_type: 'DUMB', value: '100' }
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: []
        };
        // Call upon Umi's `uploadJson()` function to upload our metadata to Arweave via Irys.
        const metadataUri = await umi.uploader.uploadJson(metadata).catch((err) => {
            throw new Error(err)
        })
        const txId = metadataUri.split('/').pop();
        const irysDevnetUri = `https://devnet.irys.xyz/${txId}`;
        console.log("Irys Gateway URI:", irysDevnetUri);
        console.log("Your metadata URI: ", metadataUri);
    }
    catch (error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
