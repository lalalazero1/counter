// const wallet = pg.wallet;
// const program = pg.program;
// const counterSeed = Buffer.from("counter");

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";

process.env.ANCHOR_WALLET = '/home/round00/.config/solana/id.json'
// Configure the client to use the local cluster.
const provider = anchor.AnchorProvider.local()
anchor.setProvider(provider);

const program = anchor.workspace.Counter as Program<Counter>;
const counter = anchor.web3.Keypair.generate();


async function initialize() {
  try {

    // TODO initialize 报错了
    const tx = await program.methods.initialize(counter.publicKey, 11).accounts({
      counter: provider.wallet.publicKey,
      authority: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,

    }).signers([provider.wallet]).rpc();
    console.log("Your transaction signature", tx);

  } catch (error) {
    console.log(error)
  }

}

initialize()




