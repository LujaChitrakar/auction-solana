#  Solana NFT Auction (Anchor)

This is a decentralized NFT auction platform built on [Solana](https://solana.com/) using [Anchor](https://book.anchor-lang.com/).  
NFT owners can list their NFTs for auction, and bidders can participate in a transparent and trustless bidding process.
The auction is properly tested.

---

## 🚀 Features

- ✅ NFT owners can create auctions
- 🧑‍⚖️ Bidders can place bids on active auctions
- 💸 If a higher bid is placed, the previous highest bid is refunded
- 🏁 When the auction ends:
  - The highest bidder receives the NFT
  - The auction creator receives the highest bid in SOL

---

## 🛠 Tech Stack

- [Solana](https://solana.com/)
- [Anchor Framework](https://book.anchor-lang.com/)
- [Metaplex Token Metadata Standard](https://docs.metaplex.com/programs/token-metadata/)
- TypeScript + Rust

---

## 🧪 How It Works

1. Create Auction


   Only the NFT owner can initialize an auction.
  The NFT is held in escrow during the auction.

3. Place Bid


   Bidders can place bids higher than the current highest bid.
Previous bidder's SOL is refunded automatically.

3. End Auction

   
  
   Once the auction duration ends, the owner finalizes it.
NFT is transferred to the highest bidder.
Funds are transferred to the auction creator.

---

## 🧑‍💻 Author


Built by Luja Chitrakar.
