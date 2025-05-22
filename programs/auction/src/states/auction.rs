use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Auction {
    pub seller: Pubkey,
    pub item_mint: Pubkey,
    pub starting_price: u64,
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub is_open: bool,
    pub bump: u8,
}
