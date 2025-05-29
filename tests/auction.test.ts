import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  getAssociatedTokenAddress,
  createMint,
  mintTo,
  createAssociatedTokenAccount,
} from "@solana/spl-token";

import { PublicKey, SystemProgram } from "@solana/web3.js";

import { Auction } from "../target/types/auction";

describe("auction", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.auction as Program<Auction>;

  let owner = provider.wallet.publicKey;
  let nftMint: PublicKey;
  let ownerNftAta: PublicKey;
  let auctionPda: PublicKey;
  let escrowAta: PublicKey;
  let auctionBump: number;
  let escrowBump: number;

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
});
