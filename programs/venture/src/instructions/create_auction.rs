use anchor_lang::prelude::*;
use crate::constants::{ANCHOR_DISCRIMINATOR_SIZE, AUCTION_SEED, PROPERTY_SEED, MAX_AUCTION_DURATION};
use crate::errors::ErrorCode;
use crate::states::{Auction, Property, MarketplaceState};

pub fn create_auction(
    ctx: Context<CreateAuctionCtx>,
    property_id: u64,
    starting_price: u64,
    reserve_price: u64,
    duration: u64,
) -> Result<()> {
    let property = &ctx.accounts.property;
    let auction = &mut ctx.accounts.auction;
    let state = &mut ctx.accounts.marketplace_state;
    let clock = Clock::get()?;

    // Validations
    if property.property_id != property_id {
        return Err(ErrorCode::PropertyNotFound.into());
    }
    if property.owner != ctx.accounts.seller.key() {
        return Err(ErrorCode::NotPropertyOwner.into());
    }
    if !property.is_listed || property.listing_type != 1 {
        return Err(ErrorCode::PropertyNotListed.into());
    }
    if duration > MAX_AUCTION_DURATION {
        return Err(ErrorCode::InvalidAuctionDuration.into());
    }
    if starting_price == 0 || reserve_price < starting_price {
        return Err(ErrorCode::InvalidPropertyPrice.into());
    }

    // Update state
    state.total_auctions += 1;

    // Initialize auction
    auction.auction_id = state.total_auctions;
    auction.property_id = property_id;
    auction.seller = ctx.accounts.seller.key();
    auction.starting_price = starting_price;
    auction.reserve_price = reserve_price;
    auction.current_bid = 0;
    auction.highest_bidder = None;
    auction.bid_count = 0;
    auction.start_time = clock.unix_timestamp as u64;
    auction.end_time = clock.unix_timestamp as u64 + duration;
    auction.is_ended = false;
    auction.winner = None;

    Ok(())
}

#[derive(Accounts)]
#[instruction(property_id: u64)]
pub struct CreateAuctionCtx<'info> {
    #[account(mut)]
    pub marketplace_state: Account<'info, MarketplaceState>,
    
    #[account(
        init,
        payer = seller,
        space = ANCHOR_DISCRIMINATOR_SIZE + Auction::INIT_SPACE,
        seeds = [
            AUCTION_SEED,
            (marketplace_state.total_auctions + 1).to_le_bytes().as_ref()
        ],
        bump
    )]
    pub auction: Account<'info, Auction>,
    
    #[account(
        seeds = [
            PROPERTY_SEED,
            property_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub property: Account<'info, Property>,
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}