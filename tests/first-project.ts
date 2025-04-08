import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FirstProject } from "../target/types/first_project";
import bs58 from "bs58";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

describe("first-project", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.firstProject as Program<FirstProject>;

  const sender = anchor.web3.Keypair.fromSecretKey(bs58.decode("M77sg6csmRvzsT9UDcjYiXvazAtWLdHgoGmsMGQdnRgd6M1yWjgujer6SteA3t4wUJ5s9kJjFLwkkKW58Mg1mg4")); // Phantom wallet holding SLMT
  let buyer: anchor.web3.Keypair;

  let mint: anchor.web3.PublicKey;
  let senderTokenAccount: anchor.web3.PublicKey;
  let userTokenAccount: anchor.web3.PublicKey;

  let presale: anchor.web3.Keypair;

  it("Buys SLMT!", async () => {
    const buyer = provider.wallet.payer;
    

    const mint = new anchor.web3.PublicKey("smtfTXAcTZAMpTpNLg76MBQJ1pTm5YA7mUP3xYFrtk9");
    senderTokenAccount = await getAssociatedTokenAddress(
      mint,
      sender.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    );

    userTokenAccount = await getAssociatedTokenAddress(
      mint,
      buyer.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    );

    // // Airdrop SOL to buyer
    // const sig = await provider.connection.requestAirdrop(
    //   buyer.publicKey,
    //   1 * anchor.web3.LAMPORTS_PER_SOL
    // );
    // await provider.connection.confirmTransaction(sig);

    // Create temp presale account to receive SOL
    const presale = new anchor.web3.PublicKey("DrPNz9nuifyiFBFaE3kxsmF7Vt9125LFNMT3bGzPaWje")

    const solToSend = new anchor.BN(0.01 * anchor.web3.LAMPORTS_PER_SOL); // 0.01 SOL

  const transferIx = anchor.web3.SystemProgram.transfer({
    fromPubkey: buyer.publicKey,
    toPubkey: presale,
    lamports: solToSend.toNumber(),
  });

  await program.methods
    .buySlmt(solToSend)
    .accounts({
      sender: sender.publicKey,
      presale: presale,
      senderTokenAccount,
      userTokenAccount,
      buyer: buyer.publicKey,
      mint,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .preInstructions([transferIx]) // ← send SOL as part of the transaction
    .signers([buyer, sender])
    .rpc();

    console.log("SLMT transferred to buyer:", buyer.publicKey.toBase58());
    
  });
});