import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  getAssociatedTokenAddress,
  createMint,
  mintTo,
  createAssociatedTokenAccount,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";

import { PublicKey, SystemProgram } from "@solana/web3.js";

import { Auction } from "../target/types/auction";
import { expect } from "chai";

describe("auction", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.auction as Program<Auction>;

  let owner = provider.wallet.publicKey;
  let nftMint: PublicKey;
  let ownerNftAta: PublicKey;
  let auctionPda: PublicKey;
  let auctionEscrowPda: PublicKey;
  // let escrowAta: PublicKey;
  let escrowNftAta: PublicKey;
  let auctionBump: number;
  let auctionEscrowBump: number;
  let escrowNftBump: number;

  it("Initialize NFT mint and ATA", async () => {
    nftMint = await createMint(
      provider.connection,
      provider.wallet.payer,
      owner,
      null,
      0 //decimal
    );

    ownerNftAta = await getAssociatedTokenAddress(nftMint, owner);

    await createAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      nftMint,
      owner
    );

    await mintTo(
      provider.connection,
      provider.wallet.payer,
      nftMint,
      ownerNftAta,
      owner,
      1
    );
    console.log("NFT minted ata:", ownerNftAta.toBase58());
  });

  it("Creates an auction", async () => {
    // derive auction pda
    [auctionPda, auctionBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("auction"), owner.toBuffer()],
      program.programId
    );

    // derive auction escrow pda
    [auctionEscrowPda, auctionEscrowBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("auction_escrow"), auctionPda.toBuffer()],
        program.programId
      );

    // derive auction escrow nft pda
    [escrowNftAta, escrowNftBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("escrow_nft"), auctionPda.toBuffer()],
        program.programId
      );

    await program.methods
      .createAuction(new anchor.BN(10), new anchor.BN(20), nftMint)
      .accounts({
        owner: owner,
        auction: auctionPda,
        auctionEscrow: auctionEscrowPda,
        ownerNftAccount: ownerNftAta,
        escrowNftTokenAccount: escrowNftAta,
        nftMint,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        system_program: SystemProgram.programId,
      })
      .signers([provider.wallet.payer])
      .rpc();

    const auction = await program.account.auction.fetch(auctionPda);
    console.log("THIS IS AUCTION", owner.toBase58());
    console.log("NFT MINT", nftMint.toBase58());
  });
});
