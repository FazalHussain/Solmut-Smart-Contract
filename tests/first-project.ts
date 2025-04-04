import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FirstProject } from "../target/types/first_project";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from "@solana/spl-token";


describe("first-project", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.firstProject as Program<FirstProject>;

  it("Is initialized!", async () => {
    // Add your test here.
    //const tx = await program.methods.initialize(new anchor.BN(100)).rpc();

    const buyer = provider.wallet.publicKey;
    const newAccount = anchor.web3.Keypair.generate();
  
    const presale = new anchor.web3.PublicKey("CE6d9AfPE5pY1og8WpyiErbrB45aU7W73XNPMrRfWGwz");
    const mint = new anchor.web3.PublicKey("smtfTXAcTZAMpTpNLg76MBQJ1pTm5YA7mUP3xYFrtk9");
    const tokenProgram = new anchor.web3.PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"); // TOKEN_2022_PROGRAM_ID
  
    const [vaultAuthority] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault")],
      program.programId
    );
    
    const vaultTokenAccount = getAssociatedTokenAddressSync(
      mint,
      vaultAuthority,
      true,
      tokenProgram,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const userTokenAccount = getAssociatedTokenAddressSync(
      mint,          // SLMT mint
      buyer,
      false,         // allowOwnerOffCurve = false
      tokenProgram
    );

    // Get the current balance of the presale account
    const presaleLamportsBefore = await provider.connection.getBalance(presale);
  
    const tx = await program.methods
      .buySlmt(new anchor.BN(presaleLamportsBefore), 1) // 1 = decimals
      .accounts({
        buyer,
        vaultAuthority,
        vaultTokenAccount,
        userTokenAccount,
        mint,
        tokenProgram,
        presale,
      })
      .signers([]) // If no additional signers
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
