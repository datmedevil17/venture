use anchor_lang::prelude::*;
use crate::constants::{ANCHOR_DISCRIMINATOR_SIZE, ESCROW_SEED, PROPERTY_SEED};
use crate::errors::ErrorCode;
use crate::states::{Escrow, Property, MarketplaceState};

pub fn create_escrow(
    ctx: Context<CreateEscrowCtx>,
    property_id: u64,
    buyer: Pubkey,
    amount: u64,
    conditions: String,
) -> Result<()> {
    let property = &ctx.accounts.property;
    let escrow = &mut ctx.accounts.escrow;
    let state = &mut ctx.accounts.marketplace_state;
    let clock = Clock::get()?;

    // Validations
    if property.property_id != property_id {
        return Err(ErrorCode::PropertyNotFound.into());
    }
    if property.owner != ctx.accounts.seller.key() {
        return Err(ErrorCode::NotPropertyOwner.into());
    }
    if !property.is_listed {
        return Err(ErrorCode::PropertyNotListed.into());
    }
    if conditions.len() > 256 {
        return Err(ErrorCode::ConditionsTooLong.into());
    }
    if amount == 0 {
        return Err(ErrorCode::InvalidEscrowAmount.into());
    }

    // Update state
    state.total_escrows += 1;

    // Initialize escrow
    escrow.escrow_id = state.total_escrows;
    escrow.property_id = property_id;
    escrow.seller = ctx.accounts.seller.key();
    escrow.buyer = buyer;
    escrow.amount = amount;
    escrow.deposited_amount = 0;
    escrow.conditions = conditions;
    escrow.created_at = clock.unix_timestamp as u64;
    escrow.is_completed = false;
    escrow.released_to_seller = false;

    Ok(())
}

#[derive(Accounts)]
#[instruction(property_id: u64)]
pub struct CreateEscrowCtx<'info> {
    #[account(mut)]
    pub marketplace_state: Account<'info, MarketplaceState>,
    
    #[account(
        init,
        payer = seller,
        space = ANCHOR_DISCRIMINATOR_SIZE + Escrow::INIT_SPACE,
        seeds = [
            ESCROW_SEED,
            (marketplace_state.total_escrows + 1).to_le_bytes().as_ref()
        ],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    
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