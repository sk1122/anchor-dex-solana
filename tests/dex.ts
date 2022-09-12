import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TypeDef } from "@project-serum/anchor/dist/cjs/program/namespace/types";
import { Dex } from "../target/types/dex";

describe("dex", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Dex as Program<Dex>;

  let keypair = anchor.web3.Keypair.generate()

  it("Is initialized!", async () => {
    // Add your test here.
    const fees: TypeDef<
      {
        name: "fees",
        type: {
          kind: "struct",
          fields: [
            {
              "name": "percentage",
              "type": "u64"
            }
          ]
        }
      },
      Record<string, number>
    > =  {
      percentage: new anchor.BN(1)
    }

    const tx = await program.methods.createLiquidityPools({fees}).accounts(
      {
        authority: keypair.publicKey,
        liquidityPool: keypair.publicKey,
        tokenA: keypair.publicKey,
        tokenB: keypair.publicKey,
        feeAccount: keypair.publicKey,
        tokenProgram: keypair.publicKey
      }
    ).signers([keypair]).rpc()

    console.log(tx)
  });
});
