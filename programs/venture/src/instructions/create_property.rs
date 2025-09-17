use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, MintTo};
use anchor_spl::associated_token::AssociatedToken;
use mpl_token_metadata::{
    instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs},
    types::DataV2,
};
use crate::constants::{ANCHOR_DISCRIMINATOR_SIZE, PROPERTY_SEED};
use crate::errors::ErrorCode;
use crate::states::{Property, MarketplaceState};

pub fn create_property(
    ctx: Context<CreatePropertyCtx>,
    title: String,
    description: String,
    image_url: String,
    location: String,
    property_type: String,
    size_sqft: u64,
    bedrooms: u8,
    bathrooms: u8,
    year_built: u16,
) -> Result<()> {
    // Validation
    if title.len() > 64 {
        return Err(ErrorCode::TitleTooLong.into());
    }
    if description.len() > 512 {
        return Err(ErrorCode::DescriptionTooLong.into());
    }
    if image_url.len() > 256 {
        return Err(ErrorCode::ImageUrlTooLong.into());
    }
    if location.len() > 128 {
        return Err(ErrorCode::LocationTooLong.into());
    }
    if property_type.len() > 32 {
        return Err(ErrorCode::PropertyTypeTooLong.into());
    }

    let state = &mut ctx.accounts.marketplace_state;
    let property = &mut ctx.accounts.property;
    let clock = Clock::get()?;

    // Update state
    state.total_properties += 1;
    
    // Initialize property
    property.property_id = state.total_properties;
    property.owner = ctx.accounts.owner.key();
    property.mint = ctx.accounts.mint.key();
    property.title = title.clone();
    property.description = description.clone();
    property.image_url = image_url.clone();
    property.location = location;
    property.property_type = property_type;
    property.size_sqft = size_sqft;
    property.bedrooms = bedrooms;
    property.bathrooms = bathrooms;
    property.year_built = year_built;
    property.created_at = clock.unix_timestamp as u64;
    property.is_listed = false;
    property.list_price = 0;
    property.listing_type = 0;

    // Mint the NFT to the owner
    let mint_cpi = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let mint_cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), mint_cpi);
    token::mint_to(mint_cpi_ctx, 1)?;

    // Create NFT metadata using Metaplex instruction
    let metadata_data = DataV2 {
        name: title,
        symbol: "PROP".to_string(),
        uri: image_url,
        seller_fee_basis_points: 250, // 2.5% royalty
        creators: Some(vec![mpl_token_metadata::types::Creator {
            address: ctx.accounts.owner.key(),
            verified: true,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    let metadata_ix = CreateMetadataAccountV3 {
        metadata: ctx.accounts.metadata.key(),
        mint: ctx.accounts.mint.key(),
        mint_authority: ctx.accounts.owner.key(),
        payer: ctx.accounts.owner.key(),
        update_authority: (ctx.accounts.owner.key(), true),
        system_program: ctx.accounts.system_program.key(),
        rent: Some(ctx.accounts.rent.key()),
    };

    let metadata_ix_args = CreateMetadataAccountV3InstructionArgs {
        data: metadata_data,
        is_mutable: true,
        collection_details: None,
    };

    // Create the instruction
    let instruction = metadata_ix.instruction(metadata_ix_args);

    // Invoke the instruction
    anchor_lang::solana_program::program::invoke(
        &instruction,
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.owner.to_account_info(), // payer
            ctx.accounts.owner.to_account_info(), // update authority
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    msg!("Property NFT created with ID: {}", property.property_id);

    Ok(())
}

#[derive(Accounts)]
pub struct CreatePropertyCtx<'info> {
    #[account(mut)]
    pub marketplace_state: Account<'info, MarketplaceState>,
    
    #[account(
        init,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR_SIZE + Property::INIT_SPACE,
        seeds = [
            PROPERTY_SEED,
            (marketplace_state.total_properties + 1).to_le_bytes().as_ref()
        ],
        bump
    )]
    pub property: Account<'info, Property>,
    
    /// CHECK: This is a mint account that will be created by the token program
    #[account(
        mut,
        signer
    )]
    pub mint: UncheckedAccount<'info>,
    
    /// CHECK: This is a token account that will be created by the associated token program
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    
    /// CHECK: Metaplex metadata account - validated by Metaplex program
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}