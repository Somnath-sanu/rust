import { test, describe, expect } from "bun:test";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { mySecretKeyAddress } from "./secrets";

const PROGRAM_ID = new PublicKey(
  "5bMFPBNv6mqUW5p2D7WYBfHNFR5ndPKGrP98UWi2D8As"
);

let PDA_ACCOUNT: null | PublicKey = null;

describe("pda", async () => {
  test("cpi on pda test", async () => {
    const connection = new Connection(
      `https://devnet.helius-rpc.com/?api-key=${process.env.HELIUS_API_KEY}`
    );

    const userAcc = Keypair.fromSecretKey(mySecretKeyAddress);

    const [pda, bump] = PublicKey.findProgramAddressSync(
      [userAcc.publicKey.toBuffer(), Buffer.from("user")],
      PROGRAM_ID
    );

    console.log("PDA account -> ", pda.toBase58());
    PDA_ACCOUNT = pda;

    const ix = new TransactionInstruction({
      keys: [
        {
          pubkey: pda,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: userAcc.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId: PROGRAM_ID,
      data: Buffer.from(""),
    });

    const tx = new Transaction();

    tx.add(ix);

    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    tx.feePayer = userAcc.publicKey;

    const signature = await connection.sendTransaction(tx, [userAcc]);

    console.log("SIGNATURE :", signature);
  });

  // delay a bit to get the balance

  test("check", async () => {
    const connection = new Connection(
      `https://devnet.helius-rpc.com/?api-key=${process.env.HELIUS_API_KEY}`
    );
    if (!PDA_ACCOUNT) {
      throw new Error("PDA_ACCOUNT NOT FOUND");
    }
    const balance = await connection.getBalance(PDA_ACCOUNT);
    console.log("BALANCE : ", balance);

    expect(balance).toBeGreaterThan(0);
  });
});
