import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { WhitelistToken } from "../target/types/whitelist_token";

describe("whitelist_token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.WhitelistToken as Program<WhitelistToken>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
