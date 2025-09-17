use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use crate::constants::{ANCHOR_DISCRIMINATOR_SIZE, AUCTION_SEED, BID_SEED, MIN_BID_INCREMENT};
use crate::errors::ErrorCode;
use crate::states::{Auction, Bid};

pub fn place_bid(
    ctx: Context<PlaceBidCtx>,
    auction_id: u64,
    bid_amount: u64,
) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let bid = &mut ctx.accounts.bid;
    let bidder = &ctx.accounts.bidder;
    let clock = Clock::get()?;

    // Validations
    if auction.auction_id != auction_id {
        return Err(ErrorCode::AuctionNotFound.into());
    }
    if auction.is_ended || clock.unix_timestamp as u64 > auction.end_time {
        return Err(ErrorCode::AuctionAlreadyEnded.into());
    }
    if auction.seller == bidder.key() {
        return Err(ErrorCode::CannotBidOnOwnAuction.into());
    }

    let min_bid = if auction.current_bid == 0 {
        auction.starting_price
    } else {
        auction.current_bid + MIN_BID_INCREMENT
    };

    if bid_amount < min_bid {
        return Err(ErrorCode::BidAmountTooLow.into());
    }

    // If there's a previous highest bidder, mark their bid as not winning
    if let Some(prev_bidder) = auction.highest_bidder {
        // In a full implementation, you'd want to refund the previous bidder
        // This would require storing bid accounts and handling refunds
        msg!("Previous highest bidder: {}", prev_bidder);
    }

    // Transfer bid amount to auction escrow (simplified)
    let transfer_instruction = system_instruction::transfer(
        &bidder.key(),
        &auction.key(),
        bid_amount,
    );

    invoke(
        &transfer_instruction,
        &[
            bidder.to_account_info(),
            auction.to_account_info(),
        ],
    )?;

    // Update auction state
    auction.current_bid = bid_amount;
    auction.highest_bidder = Some(bidder.key());
    auction.bid_count += 1;

    // Initialize bid record
    bid.auction_id = auction_id;
    bid.bidder = bidder.key();
    bid.amount = bid_amount;
    bid.timestamp = clock.unix_timestamp as u64;
    bid.is_winning = true;

    Ok(())
}

#[derive(Accounts)]
#[instruction(auction_id: u64)]
pub struct PlaceBidCtx<'info> {
    #[account(
        mut,
        seeds = [
            AUCTION_SEED,
            auction_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub auction: Account<'info, Auction>,
    
    #[account(
        init,
        payer = bidder,
        space = ANCHOR_DISCRIMINATOR_SIZE + Bid::INIT_SPACE,
        seeds = [
            BID_SEED,
            bidder.key().as_ref(),
            auction_id.to_le_bytes().as_ref(),
            (auction.bid_count + 1).to_le_bytes().as_ref()
        ],
        bump
    )]
    pub bid: Account<'info, Bid>,
    
    #[account(mut)]
    pub bidder: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}