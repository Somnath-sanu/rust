import { test, expect } from "bun:test";
import { LiteSVM } from "litesvm";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

test("CPI works", async () => {
  let svm = new LiteSVM();

  let doubleContract = PublicKey.unique();
  let cpiContract = PublicKey.unique();

  svm.addProgramFromFile(doubleContract, "./double.so");
  svm.addProgramFromFile(cpiContract, "./cpi.so");

  let userAcc = new Keypair();
  let dataAcc = new Keypair();

  svm.airdrop(userAcc.publicKey, 1_000_000_000n); // 1 SOL // OR BigInt(1000000000)

  createDataAccOnChain(svm, dataAcc, userAcc, doubleContract);

  let ix = new TransactionInstruction({
    keys: [
      {
        pubkey: dataAcc.publicKey,
        isSigner: true,
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

  let tx = new Transaction();
  tx.add(ix);
  tx.feePayer = userAcc.publicKey;
  tx.recentBlockhash = svm.latestBlockhash();
  tx.sign(userAcc, dataAcc);

  svm.sendTransaction(tx);

  const dataAccData = svm.getAccount(dataAcc.publicKey);

});

function createDataAccOnChain(
  svm: LiteSVM,
  dataAcc: Keypair,
  payer: Keypair,
  contractPubKey: PublicKey
) {
  const blockhash = svm.latestBlockhash();

  const ixs = [
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: dataAcc.publicKey,
      lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))), // lamports for rent exemption for 4 bytes not bits
      space: 4, // Space for a u32
      programId: contractPubKey,
    }),
  ];

  const tx = new Transaction();

  tx.recentBlockhash = blockhash;
  tx.feePayer = payer.publicKey;
  tx.add(...ixs);
  tx.sign(payer, dataAcc);

  svm.sendTransaction(tx);
}
