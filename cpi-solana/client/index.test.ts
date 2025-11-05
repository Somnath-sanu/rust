import { test, expect } from "bun:test";
import { LiteSVM } from "litesvm";
import {
  PublicKey,
  Transaction,
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  TransactionInstruction,
} from "@solana/web3.js";
import * as borsh from "borsh";
import { deserialize } from "borsh";

// Define the same schema as in the on-chain program
class CounterState {
  count: number;

  // parameterized constructor
  constructor(count: number) {
    this.count = count;
  }

  static schema: borsh.Schema = {
    struct: {
      count: "u32",
    },
  };
}

export const schema: borsh.Schema = {
  struct: {
    count: "u32",
  },
};

const PROGRAM_ID = new PublicKey("JzwvuJouP8XziFsbKZsCygwBY1smKbfqfCpd7TxrU6A");

test("one transfer", () => {
  const svm = new LiteSVM(); // Initialize a new in-memory Solana VM

  const contractPubKeyPair = Keypair.generate();

  svm.addProgramFromFile(PROGRAM_ID, "./cpi-solana.so"); // Load the compiled program

  const payer = new Keypair(); // Create a new keypair to act as the payer

  svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL)); // Airdrop 1 SOL to the payer

  const dataAccount = Keypair.generate(); // Create a unique public key for the data account

  const blockhash = svm.latestBlockhash(); // Get the latest blockhash

  const transferLamports = 1_000_000n; // Amount to transfer (0.001 SOL); n denotes BigInt

  const ixs = [
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: dataAccount.publicKey,
      lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))), // lamports for rent exemption for 4 bytes not bits
      space: 4, // Space for a u32
      programId: PROGRAM_ID,
    }),
  ];

  const tx = new Transaction();

  tx.recentBlockhash = blockhash; // Set the recent blockhash

  tx.add(...ixs);
  tx.feePayer = payer.publicKey; // Set the fee payer
  tx.sign(payer, dataAccount); // Sign the transaction with the payer and data account

  svm.sendTransaction(tx);

  const balanceAfter = svm.getBalance(dataAccount.publicKey);

  expect(balanceAfter).toBe(svm.minimumBalanceForRentExemption(BigInt(4)));

  function doubleCounter() {
    const blockhash = svm.latestBlockhash(); // Get the latest blockhash
    const tx = new Transaction();

    const instruction = new TransactionInstruction({
      programId: PROGRAM_ID,
      keys: [
        {
          pubkey: dataAccount.publicKey,
          isSigner: false,
          isWritable: true,
        },
      ],
      data: Buffer.from([]), // No data needed for this instruction
    });

    tx.recentBlockhash = blockhash; // Set the recent blockhash

    tx.feePayer = payer.publicKey; // Set the fee payer
    tx.add(instruction);
    tx.sign(payer); // Sign the transaction with the payer
    // since isSigner is false for dataAccount, we don't need to sign with it
    svm.sendTransaction(tx);
    svm.expireBlockhash();
  }

  doubleCounter(); // 1
  doubleCounter(); // 2
  doubleCounter(); // 4
  doubleCounter(); // 8

  const updatedAccountData = svm.getAccount(dataAccount.publicKey); // Get the account data as a Uint8Array

  if (!updatedAccountData) {
    throw new Error("Account not found");
  }

  const updatedCounter = deserialize(schema, updatedAccountData.data);

  if (!updatedCounter) {
    throw new Error("Counter not found");
  }

  console.log("Final counter value:", updatedCounter);
});
