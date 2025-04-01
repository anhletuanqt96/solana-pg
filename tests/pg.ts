import * as anchor from "@coral-xyz/anchor";
import {Program, AnchorProvider, BN} from "@coral-xyz/anchor";
import {Pg} from "../target/types/pg";
import {Keypair, SystemProgram, PublicKey} from "@solana/web3.js";
import {TOKEN_2022_PROGRAM_ID, getMintLen, createMint} from "@solana/spl-token";
import {expect} from "chai";

describe("init_vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Pg as Program<Pg>;
  const wallet = provider.wallet as anchor.Wallet;

  // Keypairs and accounts
  let tokenMint: PublicKey;
  let vaultPda: PublicKey;
  let bump: number;

  beforeEach(async () => {
    // Generate keypairs for vault and token mint
    tokenMint = await createMint(
      provider.connection,
      wallet.payer, // Payer of the transaction
      wallet.publicKey, // Mint authority
      null, // Freeze authority (null for no freeze authority)
      9, // Decimals
      Keypair.generate(), // Mint keypair
      {commitment: "confirmed"},
      TOKEN_2022_PROGRAM_ID // Use Token-2022 program
    );
    console.log({tokenMint});
    // Derive the vault PDA
    [vaultPda, bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    );
  });

  it("Successfully initializes the vault", async () => {
    // Call the init_vault instruction
    await program.methods
      .initVault()
      .accounts({
        signer: wallet.publicKey,
        vault: vaultPda,
        tokenMint: tokenMint,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const vaultAccount = await program.account.vault.fetch(vaultPda);
    expect(vaultAccount.owner.toBase58()).to.equal(
      wallet.publicKey.toBase58(),
      "Vault owner should match the signer"
    );
    expect(vaultAccount.bump).to.equal(
      bump,
      "Vault bump should match the derived bump"
    );
  });
});
