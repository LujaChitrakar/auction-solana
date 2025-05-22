use crate::states::Auction;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer=owner,
        space=Auction::INIT_SPACE,
        seeds=[b"auction",owner.key().as_ref()],
        bump,
    )]
    pub auction: Account<'info, Auction>,

    ///CHECK: ONLY HOLDS SOL
    #[account(seeds=[b"auction_escrow",auction.key().as_ref()],bump)]
    pub auction_escrow: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
