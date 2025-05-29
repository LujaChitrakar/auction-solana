use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The bid is less than highest bid")]
    BidNotHighestBid,
    #[msg("The auction has ended")]
    AuctionTImeHasPassed,
    #[msg("The previous bidder and highest bidder mismatch")]
    PreviousBidderMismatch,
    #[msg("The auction is closed")]
    AuctionClosed,
    #[msg("The auction end time is greater than current time")]
    AuctionEndTimeNotReached,
    #[msg("TOnly owner is authorized for this action.")]
    NotOwner,
}
