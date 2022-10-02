import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorDiceRollGame } from "../target/types/anchor_dice_roll_game";

describe("anchor-dice-roll-game", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorDiceRollGame as Program<AnchorDiceRollGame>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
