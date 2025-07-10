import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;
  const vault_state = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("state") , provider.wallet.publicKey.toBytes()] , program.programId)[0];
  const vault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault") , vault_state.toBytes()] , program.programId)[0];
  console.log("vault state public key: " , vault_state);
  console.log("vault public key: " , vault);
  console.log("user public key " , provider.wallet.publicKey);
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .initialize()
    .accountsPartial({
      user:provider.wallet.publicKey,
      vault,
      vaultState:vault_state,
      systemProgram:anchor.web3.SystemProgram.programId
    })
    .rpc();
    console.log("vault state public key: " , vault_state);
    console.log("vault public key: " , vault);
  });


  it("deposit", async () => {
    const amount = 2 * anchor.web3.LAMPORTS_PER_SOL;
    const tx = await program.methods
    .deposit(new anchor.BN(amount))
    .accountsPartial({
      user:provider.wallet.publicKey,
      vault,
      vaultState:vault_state,
      systemProgram:anchor.web3.SystemProgram.programId
    })
    .rpc();
    console.log("after deposit");
    console.log("Your transaction signature" , tx);
    console.log("your vault balance: " , (await provider.connection.getBalance(vault)).toString());

  });

  it("withdraw", async () => {
    const amount = 2 * anchor.web3.LAMPORTS_PER_SOL;
    const tx = await program.methods
    .withdraw(new anchor.BN(amount))
    .accountsPartial({
      user:provider.wallet.publicKey,
      vault,
      vaultState:vault_state,
      systemProgram:anchor.web3.SystemProgram.programId
    })
    .rpc();
    console.log("after withdraw");
    console.log("Your transaction signature" , tx);
    console.log("your vault balance: " , (await provider.connection.getBalance(vault)).toString());
  });

  it("close", async () => {
    const tx = await program.methods
    .close()
    .accountsPartial({
      user:provider.wallet.publicKey,
      vault,
      vaultState:vault_state,
      systemProgram:anchor.web3.SystemProgram.programId
    })
    .rpc();
    console.log("after close");
    console.log("Your transaction signature" , tx);
    console.log("your vault balance: " , (await provider.connection.getBalance(vault)).toString());
  });

});
