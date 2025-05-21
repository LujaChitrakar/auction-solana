use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The bid is less than highest bid")]
    BidNotHighestBid,
    #[msg("The auction has ended")]
    AuctionTImeHasPassed,
}
