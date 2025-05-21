use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("E6nE6seBRzfDrk1m96fKXKWxYe7JYWSpkFVMj5CLGeP6");

#[program]
pub mod auction {

    use super::*;

    pub fn create_auction(ctx: Context<CreateAuction>,starting_price:u64,end_time:i64,item_mint:Pubkey) -> Result<()> {
        let auction=&mut ctx.accounts.auction;

        auction.seller=ctx.accounts.owner.key();
        auction.item_mint=item_mint;
        auction.starting_price=starting_price;
        auction.highest_bid=starting_price;
        auction.highest_bidder=Pubkey::default();
        auction.end_time=end_time;
        auction.is_open=true;

        Ok(())
    }
}
