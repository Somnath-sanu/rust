// Bun has in-built support for tests unlike npm, so we can use its native test runner.

import * as borsh from "borsh";
import { expect, test } from "bun:test";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { COUNTER_SIZE, schema } from "./types";
import { mySecretKeyAddress } from "./secrets";

let adminAccount: Keypair;
let dataAccount: Keypair;

// have some lamports in myPubKey
const myPubKey = new PublicKey("4RGzWazZN3USpyuPZTGCCupUhJQejSPrdQbB6t59yP3d");

// my deployed program id
const PROGRAM_ID = new PublicKey(
  "8ez7ZSpzt35jDk7hL8uv596qrchnX6z1QD5p944Fim56"
);

// my keypair from secret key
const myPubKeyPair = Keypair.fromSecretKey(mySecretKeyAddress);

// test("Account is initialized", async () => {
//   // adminAccount = Keypair.generate();
//   dataAccount = Keypair.generate();

//   const connection = new Connection(
//     "https://devnet.helius-rpc.com/?api-key=d59db555-106e-4713-8e4c-f55e5773da9d"
//   );

//   // const txt = await connection.requestAirdrop(
//   //   adminAccount.publicKey,
//   //   1 * LAMPORTS_PER_SOL
//   // );

//   // console.log("Airdrop transaction signature", txt);

//   const latestBlockhash = await connection.getLatestBlockhash();

//   // await connection.confirmTransaction({
//   //   blockhash: latestBlockhash.blockhash,
//   //   lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
//   //   signature: txt,
//   // });

//   const dataAccountInfo = await connection.getAccountInfo(myPubKey);
//   console.log("Account Info: ", dataAccountInfo);

//   // airdroppedBalance done

//   const lamports = await connection.getMinimumBalanceForRentExemption(
//     COUNTER_SIZE
//   );

//   const instruction = SystemProgram.createAccount({
//     fromPubkey: myPubKey,
//     newAccountPubkey: dataAccount.publicKey,
//     lamports,
//     space: COUNTER_SIZE,
//     programId: PROGRAM_ID,
//   });

//   const createDataAccountTransaction = new Transaction();

//   createDataAccountTransaction.add(instruction);

//   const signature = await connection.sendTransaction(
//     createDataAccountTransaction,
//     [myPubKeyPair, dataAccount]
//   );

//   await connection.confirmTransaction({
//     ...latestBlockhash,
//     signature: signature,
//   });

//   console.log("Data account ", dataAccount.publicKey.toBase58());
// });

//* Now get the counter value from the account *//

const dataAccountPubkey = new PublicKey(
  "DJzdtZFCiMGMzt3gw7Xpv5GokfopqkpqAnZoTR9i8Znm"
);

test("Get the counter value from the account", async () => {
  const connection = new Connection(
    "https://devnet.helius-rpc.com/?api-key=d59db555-106e-4713-8e4c-f55e5773da9d"
  );

  const dataAccountInfo = await connection.getAccountInfo(dataAccountPubkey);

  if (dataAccountInfo === null) {
    throw new Error("Error: cannot find the data account");
  }
  const counter = borsh.deserialize(schema, dataAccountInfo.data);


  console.log("Counter value: ", counter);
});
