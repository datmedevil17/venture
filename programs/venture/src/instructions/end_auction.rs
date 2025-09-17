use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};
use crate::constants::{AUCTION_SEED, PROPERTY_SEED};
use crate::errors::ErrorCode;
use crate::states::{Auction, Property, MarketplaceState};

pub fn end_auction(
    ctx: Context<EndAuctionCtx>,
    auction_id: u64,
) -> Result<()> {
    let auction = &mut ctx.accounts.auction;
    let property = &mut ctx.accounts.property;
    let state = &ctx.accounts.marketplace_state;
    let clock = Clock::get()?;

    // Validations
    if auction.auction_id != auction_id {
        return Err(ErrorCode::AuctionNotFound.into());
    }
    if auction.is_ended {
        return Err(ErrorCode::AuctionAlreadyEnded.into());
    }
    if (clock.unix_timestamp as u64) < auction.end_time {
        return Err(ErrorCode::AuctionNotEnded.into());
    }

    auction.is_ended = true;

    // Check if reserve price was met
    if auction.current_bid >= auction.reserve_price {
        if let Some(winner) = auction.highest_bidder {
            auction.winner = Some(winner);

            // Calculate platform fee
            let platform_fee = (auction.current_bid * state.platform_fee) / 10000;
            let seller_amount = auction.current_bid - platform_fee;

            // Transfer payment to seller (simplified)
            **auction.to_account_info().try_borrow_mut_lamports()? -= auction.current_bid;
            **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += seller_amount;
            **ctx.accounts.platform_treasury.to_account_info().try_borrow_mut_lamports()? += platform_fee;

            // Transfer NFT to winner
            let cpi_accounts = Transfer {
                from: ctx.accounts.seller_token_account.to_account_info(),
                to: ctx.accounts.winner_token_account.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, 1)?;

            // Update property ownership
            property.owner = winner;
            property.is_listed = false;
            msg!("Auction ended successfully. Winner: {}", winner);
        }
    } else {
        // Reserve price not met, refund highest bidder
        if auction.current_bid > 0 {
            if let Some(_highest_bidder) = auction.highest_bidder {
                **auction.to_account_info().try_borrow_mut_lamports()? -= auction.current_bid;
                **ctx.accounts.highest_bidder_account.to_account_info().try_borrow_mut_lamports()? += auction.current_bid;
            }
        }
        msg!("Auction ended without meeting reserve price");
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(auction_id: u64)]
pub struct EndAuctionCtx<'info> {
    pub marketplace_state: Account<'info, MarketplaceState>,
    
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
        mut,
        seeds = [
            PROPERTY_SEED,
            auction.property_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub property: Account<'info, Property>,
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    /// CHECK: This is a token account owned by the seller containing the NFT
    #[account(mut)]
    pub seller_token_account: UncheckedAccount<'info>,
    
    /// CHECK: This is a token account owned by the winner to receive the NFT
    #[account(mut)]
    pub winner_token_account: UncheckedAccount<'info>,
    
    /// CHECK: Platform treasury account
    #[account(mut)]
    pub platform_treasury: UncheckedAccount<'info>,
    
    /// CHECK: Highest bidder account for refund if needed
    #[account(mut)]
    pub highest_bidder_account: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
}