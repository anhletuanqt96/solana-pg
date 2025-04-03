import * as anchor from "@coral-xyz/anchor";
import {Program, AnchorProvider, BN} from "@coral-xyz/anchor";
import {Pg} from "../target/types/pg";
import {Keypair, SystemProgram, PublicKey} from "@solana/web3.js";
import {
  TOKEN_2022_PROGRAM_ID,
  getMintLen,
  createMint,
  mintTo,
  createAssociatedTokenAccount,
} from "@solana/spl-token";
import {expect} from "chai";

describe("Test PG", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Pg as Program<Pg>;

  // Keypairs and accounts
  let tokenMint: PublicKey;
  let vaultPda: PublicKey;
  let bump: number;
  const signer = Keypair.generate();
  const addr1 = Keypair.generate();

  // beforeEach(async () => {
  before(async () => {
    await provider.connection.requestAirdrop(signer.publicKey, 20e9); // 20 SOL
    await provider.connection.requestAirdrop(addr1.publicKey, 20e9); // 20 SOL
    await new Promise(resolve => setTimeout(resolve, 1000)); // Chờ airdrop xác nhận
    // Generate keypairs for vault and token mint
    tokenMint = await createMint(
      provider.connection,
      signer, // Payer of the transaction
      signer.publicKey, // Mint authority
      null, // Freeze authority (null for no freeze authority)
      9, // Decimals
      Keypair.generate(), // Mint keypair
      {commitment: "confirmed"},
      TOKEN_2022_PROGRAM_ID // Use Token-2022 program
    );
    // Derive the vault PDA
    [vaultPda, bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    );
  });

  it("init vault", async () => {
    // Call the init_vault instruction
    await program.methods
      .initVault()
      .accounts({
        signer: signer.publicKey,
        vault: vaultPda,
        tokenMint: tokenMint,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc();

    const vaultAccount = await program.account.vault.fetch(vaultPda);
    expect(vaultAccount.owner.toBase58()).to.equal(
      signer.publicKey.toBase58(),
      "Vault owner should match the signer"
    );
    expect(vaultAccount.bump).to.equal(
      bump,
      "Vault bump should match the derived bump"
    );
  });

  it("init owner", async () => {
    const [ownerPda, bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("owner")],
      program.programId
    );
    await program.methods
      .initOwner(signer.publicKey)
      .accounts({
        signer: signer.publicKey,
        owner: ownerPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc();
    const ownerAccount = await program.account.owner.fetch(ownerPda);
    expect(ownerAccount.addr.toBase58()).to.equal(
      signer.publicKey.toBase58(),
      "owner addr should match the signer"
    );
  });

  it("transfer owner", async () => {
    const [ownerPda, bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("owner")],
      program.programId
    );
    await program.methods
      .transferOwner(addr1.publicKey)
      .accounts({
        signer: signer.publicKey,
        owner: ownerPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc();
    const ownerAccount = await program.account.owner.fetch(ownerPda);
    console.log(ownerAccount, addr1);

    expect(ownerAccount.addr.toBase58()).to.equal(
      addr1.publicKey.toBase58(),
      "owner addr should match the signer"
    );
  });

  it.only("add whitelist", async () => {
    const [ownerPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("owner")],
      program.programId
    );
    const ownerAccount = await program.account.owner.fetch(ownerPda);
    console.log(
      ownerAccount.addr.toString(),
      addr1.publicKey.toString(),
      signer.publicKey.toString()
    );
    const [whitelistPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("whitelist"), addr1.publicKey.toBuffer()],
      program.programId
    );
    await program.methods
      .addWhitelist(addr1.publicKey)
      .accounts({
        signer: addr1.publicKey,
        owner: ownerPda,
        whitelist: whitelistPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([addr1])
      .rpc();
    const account = await program.account.whitelist.fetch(whitelistPda);

    expect(account.addr.toBase58()).to.equal(
      addr1.publicKey.toBase58(),
      "addr should match the signer"
    );
  });
});
