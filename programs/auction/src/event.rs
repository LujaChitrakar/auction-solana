use anchor_lang::prelude::*;

#[event]
pub struct BidPlaced{
    pub bidder:Pubkey,
    pub bid_amount:u64,
    pub bid_time:i64
}

#[event]
pub struct AuctionStarted{
    pub seller:Pubkey,
    pub item_mint:Pubkey,
    pub starting_price:u64,
    pub strating_time:i64,
    pub end_time:i64,
}

#[event]
pub struct AuctionEnded{
    pub highest_bidder:Pubkey,
    pub highest_bid:u64,
}