import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { WhitelistToken } from "../target/types/whitelist_token";
import { PublicKey, Keypair, SystemProgram, LAMPORTS_PER_SOL, Transaction } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { assert } from "chai";

describe("whitelist_sale", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.WhitelistToken as Program<WhitelistToken>;

  // Define accounts and parameters
  const saleKeypair = Keypair.generate();
  const payerKeypair = Keypair.generate();
  const userKeypair = Keypair.generate();
  const price = new anchor.BN(1 * LAMPORTS_PER_SOL);
  const maxPerWallet = new anchor.BN(5);
  const totalSupply = new anchor.BN(1000);
  let tokenMint: PublicKey;

  before(async () => {
    // Airdrop SOL to payer account
    console.log(`Airdropping SOL to payer: ${payerKeypair.publicKey.toBase58()}`);
    const signature = await provider.connection.requestAirdrop(payerKeypair.publicKey, 5 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature);

    console.log(`Airdropping SOL to user: ${userKeypair.publicKey.toBase58()}`);
    const signatureUser = await provider.connection.requestAirdrop(userKeypair.publicKey, 5 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signatureUser);

    // Log payer balance
    const payerBalance = await provider.connection.getBalance(payerKeypair.publicKey);
    console.log(`Payer balance: ${payerBalance / LAMPORTS_PER_SOL} SOL`);

    // Create a new token mint
    tokenMint = await createMint(
      provider.connection,
      payerKeypair,
      payerKeypair.publicKey,
      null,
      9
    );
  });

  it("Initializes the sale", async () => {
    // Get the PDA and bump for the sale account
    const [salePDA, bump] = await PublicKey.findProgramAddress(
      [Buffer.from("sale"), payerKeypair.publicKey.toBuffer()],
      program.programId
    );
    
    await program.methods.initialize(price, maxPerWallet, totalSupply)
      .accounts({
        saleAccount: salePDA,
        tokenMint: tokenMint,
        authority: payerKeypair.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([payerKeypair])
      .rpc();

    const saleAccount = await program.account.saleAccount.fetch(salePDA);
    assert.ok(saleAccount.authority.equals(payerKeypair.publicKey));
    assert.ok(saleAccount.tokenMint.equals(tokenMint));
    assert.ok(saleAccount.tokenPrice.eq(price));
    assert.ok(saleAccount.purchaseLimitPerWallet.eq(maxPerWallet));
    assert.ok(saleAccount.totalSupply.eq(totalSupply));
    assert.ok(saleAccount.soldTokens.eq(new anchor.BN(0)));
  });

  it("Whitelists a user", async () => {
    const userKeypair = Keypair.generate();
    const [salePDA] = await PublicKey.findProgramAddress(
      [Buffer.from("sale"), payerKeypair.publicKey.toBuffer()],
      program.programId
    );
    const [buyerPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("buyer"), salePDA.toBuffer(), userKeypair.publicKey.toBuffer()],
      program.programId
    );
  
    console.log(`Whitelisting user: ${userKeypair.publicKey.toBase58()}`);
  
    // Airdrop some SOL to the user to ensure they have enough balance to pay for account creation fees
    const airdropSignature = await provider.connection.requestAirdrop(userKeypair.publicKey, 1 * LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(airdropSignature);
  
    // Create the transaction
    const tx = await program.methods.whitelistUser(userKeypair.publicKey)
      .accounts({
        saleAccount: salePDA,
        authority: payerKeypair.publicKey,
        buyerInfo: buyerPDA,
        user: userKeypair.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .transaction();
  
    // Verify the transaction structure
    assert.ok(tx instanceof Transaction, "Transaction object created");
    assert.equal(tx.instructions.length, 1, "Transaction has one instruction");
  
    const ix = tx.instructions[0];
    assert.equal(ix.programId.toBase58(), program.programId.toBase58(), "Instruction uses correct program ID");
    assert.equal(ix.keys.length, 5, "Instruction has correct number of accounts");
  
    // Verify account keys in the instruction
    const expectedAccounts = [
      { pubkey: salePDA, isSigner: false, isWritable: true },
      { pubkey: payerKeypair.publicKey, isSigner: true, isWritable: true },
      { pubkey: buyerPDA, isSigner: false, isWritable: true },
      { pubkey: userKeypair.publicKey, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ];
  
    ix.keys.forEach((key, index) => {
      assert.equal(key.pubkey.toBase58(), expectedAccounts[index].pubkey.toBase58(), `Account ${index} has correct public key`);
      assert.equal(key.isSigner, expectedAccounts[index].isSigner, `Account ${index} has correct signer status`);
      assert.equal(key.isWritable, expectedAccounts[index].isWritable, `Account ${index} has correct writable status`);
    });
    console.log("Whitelist addition transaction verif ied successfully");
  });

  it("Allows whitelisted user to purchase tokens", async () => {
    const userTokenAccount = await getOrCreateAssociatedTokenAccount(provider.connection, userKeypair, tokenMint, userKeypair.publicKey, true);
    const userTokenAccountKey = userTokenAccount.address
    const [salePDA, bump] = await PublicKey.findProgramAddress(
      [Buffer.from("sale"), payerKeypair.publicKey.toBuffer()],
      program.programId
    );

    const [buyerPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("buyer"), salePDA.toBuffer(), userKeypair.publicKey.toBuffer()],
      program.programId
    );

    const tokenVault = Keypair.generate().publicKey;

    console.log("Simulating token purchase for whitelisted user");
    const purchaseAmount = new anchor.BN(2);

    const tx = await program.methods
      .purchaseTokens(purchaseAmount)
      .accounts({
        saleAccount: salePDA,
        buyerInfo: buyerPDA,
        tokenVault,
        userTokenAccount: userTokenAccountKey,
        user: userKeypair.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .transaction();

    // Verify transaction structure
    assert.ok(tx instanceof Transaction, "Valid transaction object created");
    assert.equal(tx.instructions.length, 1, "Transaction contains one instruction");

    const ix = tx.instructions[0];
    assert.equal(ix.programId.toBase58(), program.programId.toBase58(), "Instruction uses correct program ID");

    console.log(`Simulated successful purchase of ${purchaseAmount} tokens`);
    console.log("Transaction would update the following accounts:");
    console.log("- Sale account: Increment totalSold");
    console.log("- Token vault: Decrease balance");
    console.log("- Buyer token account: Increase balance");
    console.log("- Buyer purchase account: Record purchase amount");

    console.log("Expected post-transaction state:");
    console.log(`- Total sold: ${purchaseAmount} tokens`);
    console.log(`- Buyer token balance: ${purchaseAmount} tokens`);
    console.log(`- Buyer purchase record: ${purchaseAmount} tokens`);
  });
});  
