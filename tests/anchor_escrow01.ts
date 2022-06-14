import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorEscrow01 } from "../target/types/anchor_escrow01";

describe("anchor_escrow01", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorEscrow01 as Program<AnchorEscrow01>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
