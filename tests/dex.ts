import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TypeDef } from "@project-serum/anchor/dist/cjs/program/namespace/types";
import { TOKEN_PROGRAM_ID, createAccount, createAssociatedTokenAccount, createMint } from "@solana/spl-token";
import { Dex } from "../target/types/dex";
import time from "timers"
import { Connection, PublicKey } from "@solana/web3.js";
import idl from '../target/idl/dex.json'

describe("dex", () => {
  // Configure the client to use the local cluster.
  const provider = new Connection("https://api.devnet.solana.com")
  anchor.setProvider(anchor.AnchorProvider.env());

  const programID = new PublicKey(idl.metadata.address)

  const program = anchor.workspace.Dex as Program<Dex>;

  const authority = anchor.web3.Keypair.generate()
  const payer = anchor.web3.Keypair.generate()
  const ammAccount = anchor.web3.Keypair.generate()

  it("Is initialized!", async () => {
    const sig = await provider.requestAirdrop(
      ammAccount.publicKey,
      10000000000000
    )
    await provider.confirmTransaction(sig, "confirmed")
    console.log(sig)

    const tx = await program
      .methods
      .vote(true)
      .accounts(
        {
          vote: ammAccount.publicKey
        }
      )
      .signers([ammAccount])
      .rpc()

    console.log(tx)

    const tx1 = await program
      .methods
      .vote(false)
      .accounts(
        {
          vote: ammAccount.publicKey
        }
      )
      .signers([ammAccount])
      .rpc()

    console.log(tx1)

    const voteYes = await (await program.account.voteAccount.fetch(ammAccount.publicKey)).yes
    const voteNo = await (await program.account.voteAccount.fetch(ammAccount.publicKey)).no

    console.log(voteNo.toString(), voteYes.toString(), await (await program.account.voteAccount.fetch(payer.publicKey)).no)
  });
});
