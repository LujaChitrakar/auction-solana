use anchor_lang::prelude::*;

use crate::states::Auction;

#[derive(Accounts)]
pub struct CreateAuction<'info>{
    #[account(mut)]
    pub owner:Signer<'info>,

    #[account(
        init,
        payer=owner,
        space=Auction::INIT_SPACE,
        seeds=[b"auction",owner.key().as_ref()],
        bump,
    )]
    pub auction:Account<'info,Auction>,

    pub system_program:Program<'info,System>
}