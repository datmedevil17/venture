#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("8G9j36JgtL33qV4kJ7mW1QbnemPSxVB2Zyept7fnWLmx");

#[program]
pub mod property_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCtx>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }

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
        instructions::create_property::create_property(
            ctx, title, description, image_url, location,
            property_type, size_sqft, bedrooms, bathrooms, year_built
        )
    }

    pub fn list_property(
        ctx: Context<ListPropertyCtx>,
        property_id: u64,
        price: u64,
        listing_type: u8, // 0: Direct Sale, 1: Auction
    ) -> Result<()> {
        instructions::list_property::list_property(ctx, property_id, price, listing_type)
    }

    pub fn create_auction(
        ctx: Context<CreateAuctionCtx>,
        property_id: u64,
        starting_price: u64,
        reserve_price: u64,
        duration: u64,
    ) -> Result<()> {
        instructions::create_auction::create_auction(ctx, property_id, starting_price, reserve_price, duration)
    }

    pub fn place_bid(
        ctx: Context<PlaceBidCtx>,
        auction_id: u64,
        bid_amount: u64,
    ) -> Result<()> {
        instructions::place_bid::place_bid(ctx, auction_id, bid_amount)
    }

    pub fn end_auction(
        ctx: Context<EndAuctionCtx>,
        auction_id: u64,
    ) -> Result<()> {
        instructions::end_auction::end_auction(ctx, auction_id)
    }

    pub fn create_escrow(
        ctx: Context<CreateEscrowCtx>,
        property_id: u64,
        buyer: Pubkey,
        amount: u64,
        conditions: String,
    ) -> Result<()> {
        instructions::create_escrow::create_escrow(ctx, property_id, buyer, amount, conditions)
    }

    pub fn deposit_escrow(
        ctx: Context<DepositEscrowCtx>,
        escrow_id: u64,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit_escrow::deposit_escrow(ctx, escrow_id, amount)
    }

    pub fn release_escrow(
        ctx: Context<ReleaseEscrowCtx>,
        escrow_id: u64,
        release_to_seller: bool,
    ) -> Result<()> {
        instructions::release_escrow::release_escrow(ctx, escrow_id, release_to_seller)
    }

     pub fn buy_direct(
        ctx: Context<BuyDirectCtx>,
        property_id: u64,
    ) -> Result<()> {
        instructions::buy_direct::buy_direct(ctx, property_id)
    }

    pub fn cancel_listing(
        ctx: Context<CancelListingCtx>,
        property_id: u64,
    ) -> Result<()> {
        instructions::cancel_listing::cancel_listing(ctx, property_id)
    }

    pub fn update_platform_settings(
        ctx: Context<UpdatePlatformSettingsCtx>,
        new_fee: Option<u64>,
        new_treasury: Option<Pubkey>,
    ) -> Result<()> {
        instructions::update_platform_settings::update_platform_settings(ctx, new_fee, new_treasury)
    }
}