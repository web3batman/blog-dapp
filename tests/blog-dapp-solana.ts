import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BlogDappSolana } from "../target/types/blog_dapp_solana";

describe("blog-dapp-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.BlogDappSolana as Program<BlogDappSolana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
