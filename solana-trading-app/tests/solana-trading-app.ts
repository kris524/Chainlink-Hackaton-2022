// import * as anchor from "@project-serum/anchor";
// import { Program } from "@project-serum/anchor";
// import { SolanaTradingApp } from "../target/types/solana_trading_app";

// describe("solana-trading-app", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.SolanaTradingApp as Program<SolanaTradingApp>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });


const anchor = require('@project-serum/anchor');

describe('solana-trading-app', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it('Is initialized!', async () => {
    const program = anchor.workspace.SolanaTradingApp;
    const tx = await program.rpc.initialize();
    console.log("Your transaction signature", tx);
  });
});