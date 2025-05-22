use anchor_lang::prelude::*;

use crate::states::Auction;

#[derive(Accounts)]
pub struct EndAuction<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds=[b"auction",owner.key().as_ref()],
        bump
    )]
    pub auction: Account<'info, Auction>,

    ///CHECK: THIS IS the previous bidder
    #[account(mut)]
    pub previous_bidder: UncheckedAccount<'info>,

    ///CHECK: THis account only holds sol
    #[account(mut)]
    pub escrow_auction: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
