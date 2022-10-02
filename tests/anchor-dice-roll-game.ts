import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorDiceRollGame } from "../target/types/anchor_dice_roll_game";

describe("anchor-dice-roll-game", () => {
    // Configure the client to use the local cluster.
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace
        .AnchorDiceRollGame as Program<AnchorDiceRollGame>;

    const player = anchor.web3.Keypair.generate();

    it("Is initialized!", async () => {
        let sig = await provider.connection.requestAirdrop(
            player.publicKey,
            1 * anchor.web3.LAMPORTS_PER_SOL
        );
        await provider.connection.confirmTransaction(sig);

        const balance = await provider.connection.getBalance(player.publicKey);

        console.log(player.publicKey.toBase58());
        console.log(balance / anchor.web3.LAMPORTS_PER_SOL);

        // Add your test here.
        const tx = await program.methods
            .setup(
                player.publicKey,
                new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL)
            )
            .rpc()
            .catch(console.error);
        console.log("Your transaction signature", tx);
    });

    it("Playing the dice game.", async () => {
        // A dice contains 6 numbers (1,2,3,4,5,6). Player needs to send 3 number in a format like below as string. Player has %50 chance to win.

        // If player wins player gets 2x bet. If player lose it lose bet balance.
        let ix = program.methods.play("1,2,5");

        const tx = await ix.rpc().catch(console.error);
        console.log("transaction: ", tx);

        const userdiceroll = (await ix.pubkeys()).diceRoll;

        let data = await program.account.diceRoll.fetch(userdiceroll);

        const balance = await provider.connection.getBalance(player.publicKey);
        console.log(balance / anchor.web3.LAMPORTS_PER_SOL);
        console.log(data);
    });
});
