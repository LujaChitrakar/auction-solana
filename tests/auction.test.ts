import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  getAssociatedTokenAddress,
  createMint,
  mintTo,
  createAssociatedTokenAccount,
  createAssociatedTokenAccountInstruction,
  getAccount,
} from "@solana/spl-token";

import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";

import { Auction } from "../target/types/auction";
import { expect } from "chai";

describe("auction", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.auction as Program<Auction>;
  let auction;
  let owner = provider.wallet.publicKey;
  let nftMint: PublicKey;
  let ownerNftAta: PublicKey;
  let highestBidderNftAta: PublicKey;
  let auctionPda: PublicKey;
  let auctionEscrowPda: PublicKey;
  // let escrowAta: PublicKey;
  let escrowNftAta: PublicKey;
  let auctionBump: number;
  let auctionEscrowBump: number;
  let escrowNftBump: number;

  let bidder1 = Keypair.generate();
  let bidder2 = Keypair.generate();

  it("Initialize NFT mint and ATA", async () => {
    nftMint = await createMint(
      provider.connection,
      provider.wallet.payer,
      owner,
      null,
      0 //decimal
    );

    ownerNftAta = await getAssociatedTokenAddress(nftMint, owner);
    highestBidderNftAta = await getAssociatedTokenAddress(
      nftMint,
      bidder2.publicKey
    );

    try {
      await getAccount(provider.connection, highestBidderNftAta);
    } catch (_) {
      const tx = new anchor.web3.Transaction().add(
        createAssociatedTokenAccountInstruction(
          provider.wallet.publicKey,
          highestBidderNftAta,
          bidder2.publicKey,
          nftMint
        )
      );
      await provider.sendAndConfirm(tx);
    }

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
      .createAuction(new anchor.BN(100), new anchor.BN(10000000000000), nftMint)
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

    auction = await program.account.auction.fetch(auctionPda);
    // console.log("THIS IS AUCTION", auction);
    // console.log("NFT MINT", nftMint.toBase58());
  });

  it("Should be able to bid on the auction", async () => {
    // await new Promise((resolve) => setTimeout(resolve, 500)); // 500ms delay

    const dummyPreviousBidder = Keypair.generate();
    await provider.connection.requestAirdrop(
      dummyPreviousBidder.publicKey,
      1e9
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        dummyPreviousBidder.publicKey,
        1e9
      ),
      "confirmed"
    );

    // bidder1 = Keypair.generate(); // Assuming you define bidder1 here
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(bidder1.publicKey, 1e9),
      "confirmed"
    );

    const balance = await provider.connection.getBalance(bidder1.publicKey);
    const balance2 = await provider.connection.getBalance(
      dummyPreviousBidder.publicKey
    );

    // console.log("BALANCES", balance, balance2);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    await program.methods
      .createBid(new anchor.BN(110))
      .accounts({
        bidder: bidder1.publicKey,
        auction: auctionPda,
        previousBidder: dummyPreviousBidder.publicKey,
        auctionEscrow: auctionEscrowPda,
        system_program: SystemProgram.programId,
      })
      .signers([bidder1])
      .rpc();

    auction = await program.account.auction.fetch(auctionPda);
    // console.log(auction);
  });

  it("Should be able to have multiple bids in auction", async () => {
    await provider.connection.requestAirdrop(bidder1.publicKey, 1e9);
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(bidder1.publicKey, 1e9),
      "confirmed"
    );

    await provider.connection.requestAirdrop(bidder2.publicKey, 1e9);
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(bidder2.publicKey, 1e9),
      "confirmed"
    );

    const balance = await provider.connection.getBalance(bidder1.publicKey);
    const balance2 = await provider.connection.getBalance(bidder2.publicKey);

    await new Promise((resolve) => setTimeout(resolve, 1000));
    try {
      await program.methods
        .createBid(new anchor.BN(120))
        .accounts({
          bidder: bidder2.publicKey,
          auction: auctionPda,
          previousBidder: bidder1.publicKey,
          auctionEscrow: auctionEscrowPda,
          system_program: SystemProgram.programId,
        })
        .signers([bidder2])
        .rpc();
      const bid = await program.account.auction.fetch(auctionPda);
      console.log("CREATED", bid);
    } catch (err) {
      console.log(err);
    }
  });

  it("Should be able to end auction and give nft to the highest bidder", async () => {
    console.log("Owner", owner);
    console.log("SEller", auction);
    await program.methods
      .endAuction()
      .accounts({
        owner: owner,
        auction: auctionPda,
        auctionEscrow: auctionEscrowPda,
        highestBidderNftAccount: highestBidderNftAta,
        escrowNftTokenAccount: escrowNftAta,
        previousBidder: bidder2.publicKey,
        nftMint: nftMint,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        system_program: SystemProgram.programId,
      })
      .rpc();

    let auction_ended = await program.account.auction.fetch(auctionPda);
    console.log(auction_ended);
  });
});
