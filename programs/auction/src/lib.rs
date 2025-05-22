use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;
pub mod error;
pub mod event;
use instructions::*;
use error::ErrorCode;
use event::*;
declare_id!("E6nE6seBRzfDrk1m96fKXKWxYe7JYWSpkFVMj5CLGeP6");

#[program]
pub mod auction {
    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;

    pub fn create_auction(
        ctx: Context<CreateAuction>,
        starting_price: u64,
        end_time: i64,
        item_mint: Pubkey,
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let current_time=Clock::get()?.unix_timestamp;

        auction.seller = ctx.accounts.owner.key();
        auction.item_mint = item_mint;
        auction.starting_price = starting_price;
        auction.highest_bid = starting_price;
        auction.highest_bidder = Pubkey::default();
        auction.start_time=current_time;
        auction.end_time = end_time;
        auction.is_open = true;
        auction.bump = ctx.bumps.auction_escrow;

        emit!(AuctionStarted{seller:ctx.accounts.owner.key(),item_mint:item_mint,starting_price:starting_price,strating_time:current_time,end_time:end_time});
        Ok(())
    }

    pub fn create_bid(ctx: Context<CreateBid>, bid_amount: u64) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let highest_bid = auction.highest_bid;
        let auction_key = auction.key();

        let auction_end_time = auction.end_time;
        let current_time = Clock::get()?.unix_timestamp;

        require!(highest_bid < bid_amount, ErrorCode::BidNotHighestBid);
        require!(
            auction_end_time > current_time,
            ErrorCode::AuctionTImeHasPassed
        );
        require!(auction.is_open==true,ErrorCode::AuctionClosed);

        require_keys_eq!(
            ctx.accounts.previous_bidder.key(),
            auction.highest_bidder,
            ErrorCode::PreviousBidderMismatch
        );

        if auction.highest_bidder != Pubkey::default() {
            let seeds: &[&[&[u8]]] = &[&[b"auction_escrow", auction_key.as_ref(), &[auction.bump]]];
            let refund_amount = auction.highest_bid;

            let cpi_ctx_new = CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.auction_escrow.to_account_info(),
                    to: ctx.accounts.previous_bidder.to_account_info(),
                },
            )
            .with_signer(seeds);
            transfer(cpi_ctx_new, refund_amount)?;
        }

        let cpi_ctx_old = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.bidder.to_account_info(),
                to: ctx.accounts.auction_escrow.to_account_info(),
            },
        );

        transfer(cpi_ctx_old, bid_amount)?;

        auction.highest_bid = bid_amount;
        auction.highest_bidder = ctx.accounts.bidder.key();

        emit!(BidPlaced{bidder:ctx.accounts.auction.highest_bidder,bid_amount:bid_amount,bid_time:current_time});
        Ok(())
    }

    pub fn end_auction(ctx: Context<EndAuction>)->Result<()>{
        let auction=&mut ctx.accounts.auction;
        let clock=Clock::get()?.unix_timestamp;
        let auction_key=auction.key();

        require!(auction.end_time<=clock,ErrorCode::AuctionEndTimeNotReached);
        require_keys_eq!(ctx.accounts.previous_bidder.key(),auction.highest_bidder,ErrorCode::PreviousBidderMismatch);

        let signer_seeds: &[&[&[u8]]]=&[&[b"auction_escrow",auction_key.as_ref(),&[auction.bump]]];

        let cpi_ctx=CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info(), Transfer{from:ctx.accounts.escrow_auction.to_account_info(),to:ctx.accounts.owner.to_account_info()}, signer_seeds);

        transfer(cpi_ctx, auction.highest_bid)?;

        auction.is_open=false;

        emit!(AuctionEnded{highest_bidder:auction.highest_bidder,highest_bid:auction.highest_bid});
        Ok(())
    }
}
