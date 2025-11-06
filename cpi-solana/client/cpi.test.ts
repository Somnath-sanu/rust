import { test, expect } from "bun:test";
import { LiteSVM } from "litesvm";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  Connection,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { mySecretKeyAddress } from "./secrets";
import { COUNTER_SIZE, schema } from "./types";
import * as borsh from "borsh";

test("CPI works", async () => {
  // let svm = new LiteSVM();
  const connection = new Connection(
    `https://devnet.helius-rpc.com/?api-key=${process.env.HELIUS_API_KEY}`
  );

  // let doubleContract = PublicKey.unique();
  //let cpiContract = PublicKey.unique(); // like middleman works

  let doubleContract = new PublicKey(
    "JzwvuJouP8XziFsbKZsCygwBY1smKbfqfCpd7TxrU6A"
  );
  let cpiContract = new PublicKey(
    "4fqp9qZGWeSP9xgf6crEx2EAUvd4h5vY1gARjbnnpEtY"
  );

  // svm.addProgramFromFile(doubleContract, "./double.so");
  // svm.addProgramFromFile(cpiContract, "./cpi.so");

  let userAcc = Keypair.fromSecretKey(mySecretKeyAddress);
  let dataAcc = new Keypair();

  // svm.airdrop(userAcc.publicKey, 1_000_000_000n); // 1 SOL // OR BigInt(1000000000);

  // await connection.requestAirdrop(userAcc.publicKey, LAMPORTS_PER_SOL * 1); // 1 SOL

  await createDataAccOnChain(connection, dataAcc, userAcc, doubleContract);

  console.log("public key of data account : ", dataAcc.publicKey.toBase58());

  const dataAccInfo = await connection.getAccountInfo(
    dataAcc.publicKey,
    "confirmed"
  );

  // if (!dataAccInfo?.data) {
  //   throw new Error("data Account Info not found!!!");
  // }

  // console.log("data account owner: " , dataAccInfo.owner.toBase58())
  // const counter = borsh.deserialize(schema, dataAccInfo?.data);

  // console.log("Counter value: ", counter);

  let ix = new TransactionInstruction({
    keys: [
      {
        pubkey: dataAcc.publicKey,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: doubleContract,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: cpiContract,
    data: Buffer.alloc(0), // No data needed for this instruction
  });

  const blockhash = await connection.getLatestBlockhash();

  let tx = new Transaction();
  tx.add(ix);
  tx.feePayer = userAcc.publicKey;

  // tx.recentBlockhash = svm.latestBlockhash();
  tx.recentBlockhash = blockhash.blockhash;
  // tx.sign(userAcc, dataAcc);

  // svm.sendTransaction(tx);

  // const dataAccData = svm.getAccount(dataAcc.publicKey);

  const signature = await connection.sendTransaction(tx, [userAcc]);

  // const sendTransaction = await connection.confirmTransaction({
  //   ...blockhash,
  //   signature,
  // });


  const dataAccountInfo = await connection.getAccountInfo(
    dataAcc.publicKey,
    "confirmed"
  );

  // if (!dataAccountInfo) {
  //   throw new Error("Data Account not found!");
  // }

  console.log("dataAccData : ", dataAccountInfo);

  // expect(dataAccData.data[0]).toBe(1);
});

async function createDataAccOnChain(
  connection: Connection,
  dataAcc: Keypair,
  payer: Keypair,
  doubleContract: PublicKey // only this program has rights to change data in the data account not system program
) {
  // const blockhash = svm.latestBlockhash();
  const blockhash = await connection.getLatestBlockhash();
  const lamports = await connection.getMinimumBalanceForRentExemption(
    COUNTER_SIZE
  );

  const ixs = [
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: dataAcc.publicKey,

      // lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))), // lamports for rent exemption for 4 bytes not bits

      lamports,
      space: COUNTER_SIZE, // Space for a u32
      programId: doubleContract,
    }),
  ];

  const tx = new Transaction();

  // tx.recentBlockhash = blockhash.blockhash;
  // tx.feePayer = payer.publicKey;
  tx.add(...ixs);
  // tx.sign(payer, dataAcc);
  tx.recentBlockhash = blockhash.blockhash;

  // svm.sendTransaction(tx);
  const signature = await connection.sendTransaction(tx, [payer, dataAcc]);

  // await connection.confirmTransaction({
  //   signature,
  //   ...blockhash,
  // });

  console.log(
    "Data Account created successfully \n",
    dataAcc.publicKey.toBase58()
  );
}
