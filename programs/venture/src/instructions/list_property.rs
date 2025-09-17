use anchor_lang::prelude::*;
use crate::constants::{PROPERTY_SEED, MIN_PROPERTY_PRICE};
use crate::errors::ErrorCode;
use crate::states::Property;

pub fn list_property(
    ctx: Context<ListPropertyCtx>,
    property_id: u64,
    price: u64,
    listing_type: u8,
) -> Result<()> {
    let property = &mut ctx.accounts.property;

    // Validations
    if property.property_id != property_id {
        return Err(ErrorCode::PropertyNotFound.into());
    }
    if property.owner != ctx.accounts.owner.key() {
        return Err(ErrorCode::NotPropertyOwner.into());
    }
    if property.is_listed {
        return Err(ErrorCode::PropertyAlreadyListed.into());
    }
    if price < MIN_PROPERTY_PRICE {
        return Err(ErrorCode::InvalidPropertyPrice.into());
    }
    if listing_type > 1 {
        return Err(ErrorCode::InvalidListingType.into());
    }

    // Update property listing
    property.is_listed = true;
    property.list_price = price;
    property.listing_type = listing_type;

    Ok(())
}

#[derive(Accounts)]
#[instruction(property_id: u64)]
pub struct ListPropertyCtx<'info> {
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
    pub owner: Signer<'info>,
}