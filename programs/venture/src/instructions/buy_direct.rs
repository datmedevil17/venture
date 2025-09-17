use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use crate::constants::PROPERTY_SEED;
use crate::errors::ErrorCode;
use crate::states::{Property, MarketplaceState};

pub fn buy_direct(
    ctx: Context<BuyDirectCtx>,
    property_id: u64,
) -> Result<()> {
    let property = &mut ctx.accounts.property;
    let state = &ctx.accounts.marketplace_state;
    let buyer = &ctx.accounts.buyer;

    // Validations
    if property.property_id != property_id {
        return Err(ErrorCode::PropertyNotFound.into());
    }
    if !property.is_listed || property.listing_type != 0 {
        return Err(ErrorCode::PropertyNotListed.into());
    }
    if property.owner == buyer.key() {
        return Err(ErrorCode::NotPropertyOwner.into());
    }

    let sale_price = property.list_price;
    let platform_fee = (sale_price * state.platform_fee) / 10000;
    let seller_amount = sale_price - platform_fee;

    // Transfer payment
    let transfer_to_seller = system_instruction::transfer(
        &buyer.key(),
        &ctx.accounts.seller.key(),
        seller_amount,
    );
    let transfer_to_platform = system_instruction::transfer(
        &buyer.key(),
        &state.platform_treasury,
        platform_fee,
    );

    invoke(
        &transfer_to_seller,
        &[
            buyer.to_account_info(),
            ctx.accounts.seller.to_account_info(),
        ],
    )?;

    invoke(
        &transfer_to_platform,
        &[
            buyer.to_account_info(),
            ctx.accounts.platform_treasury.to_account_info(),
        ],
    )?;

    // Transfer NFT to buyer
    let cpi_accounts = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, 1)?;

    // Update property ownership
    property.owner = buyer.key();
    property.is_listed = false;
    property.list_price = 0;

    msg!("Property {} sold directly to {} for {} lamports", property_id, buyer.key(), sale_price);

    Ok(())
}

#[derive(Accounts)]
#[instruction(property_id: u64)]
pub struct BuyDirectCtx<'info> {
    pub marketplace_state: Account<'info, MarketplaceState>,
    
    #[account(
        mut,
        seeds = [
            PROPERTY_SEED,
            property_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub property: Account<'info, Property>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK: Seller account
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    
    /// CHECK: This is a token account owned by the seller containing the NFT
    #[account(mut)]
    pub seller_token_account: UncheckedAccount<'info>,
    
    /// CHECK: This is a token account owned by the buyer to receive the NFT
    #[account(mut)]
    pub buyer_token_account: UncheckedAccount<'info>,
    
    /// CHECK: Platform treasury
    #[account(mut)]
    pub platform_treasury: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}