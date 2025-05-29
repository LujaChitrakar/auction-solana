use crate::states::Auction;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

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

    ///CHECK: THis account only holds sol
    #[account(mut)]
    pub auction_escrow: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint=highest_bidder_nft_account.owner==previous_bidder.key(),
        constraint=highest_bidder_nft_account.mint==nft_mint.key()
    )]
    pub highest_bidder_nft_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint=escrow_nft_token_account.owner==auction.key(),
        constraint=escrow_nft_token_account.mint==nft_mint.key(),
    )]
    pub escrow_nft_token_account: Account<'info, TokenAccount>,

    ///CHECK: THIS IS the previous bidder
    #[account(mut)]
    pub previous_bidder: UncheckedAccount<'info>,

    pub nft_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}
