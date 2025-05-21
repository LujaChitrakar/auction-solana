use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod states;
use error::ErrorCode;
use instructions::*;
declare_id!("E6nE6seBRzfDrk1m96fKXKWxYe7JYWSpkFVMj5CLGeP6");

#[program]
pub mod auction {
    use anchor_lang::{ system_program::{ transfer, Transfer}};
    use states::auction;

    use super::*;

    pub fn create_auction(
        ctx: Context<CreateAuction>,
        starting_price: u64,
        end_time: i64,
        item_mint: Pubkey,
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;

        auction.seller = ctx.accounts.owner.key();
        auction.item_mint = item_mint;
        auction.starting_price = starting_price;
        auction.highest_bid = starting_price;
        auction.highest_bidder = Pubkey::default();
        auction.end_time = end_time;
        auction.is_open = true;

        Ok(())
    }

    pub fn create_bid(ctx: Context<CreateBid>, bid_amount: u64) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let highest_bid = auction.highest_bid;
        let auction_key=auction.key();
        
        let auction_end_time=auction.end_time;
        let current_time=Clock::get()?.unix_timestamp;
        
        require!(highest_bid < bid_amount, ErrorCode::BidNotHighestBid);
        require!(auction_end_time > current_time, ErrorCode::AuctionTImeHasPassed);
        
        if auction.highest_bidder !=Pubkey::default(){
            let refund_amount=auction.highest_bid;
            **ctx.accounts.auction_escrow.to_account_info().try_borrow_mut_lamports()?-=refund_amount;
            **ctx.accounts.previous_bidder.to_account_info().try_borrow_mut_lamports()?+=refund_amount;
        }
        // need to change put signer.

        let cpi_ctx=CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer{from:ctx.accounts.bidder.to_account_info(),to:ctx.accounts.auction_escrow.to_account_info()});

        transfer(cpi_ctx, bid_amount)?;

        auction.highest_bid = bid_amount;
        auction.highest_bidder = ctx.accounts.bidder.key();

        Ok(())
    }
}
