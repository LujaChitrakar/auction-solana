use crate::states::Auction;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateBid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        mut,
        seeds=[b"auction", auction.seller.key().as_ref()],
        bump)]
    pub auction: Account<'info, Auction>,

    ///CHECK: refund back to previous bidder
    #[account(mut)]
    pub previous_bidder:UncheckedAccount<'info>,

    ///CHECK: escrowaccount that only holds sol
    #[account(mut)]
    pub auction_escrow:UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
