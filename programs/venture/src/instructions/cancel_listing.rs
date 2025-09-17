use anchor_lang::prelude::*;
use crate::constants::PROPERTY_SEED;
use crate::errors::ErrorCode;
use crate::states::Property;

pub fn cancel_listing(
    ctx: Context<CancelListingCtx>,
    property_id: u64,
) -> Result<()> {
    let property = &mut ctx.accounts.property;

    // Validations
    if property.property_id != property_id {
        return Err(ErrorCode::PropertyNotFound.into());
    }
    if property.owner != ctx.accounts.owner.key() {
        return Err(ErrorCode::NotPropertyOwner.into());
    }
    if !property.is_listed {
        return Err(ErrorCode::PropertyNotListed.into());
    }

    // Cancel listing
    property.is_listed = false;
    property.list_price = 0;
    property.listing_type = 0;

    msg!("Property {} listing cancelled", property_id);

    Ok(())
}

#[derive(Accounts)]
#[instruction(property_id: u64)]
pub struct CancelListingCtx<'info> {
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