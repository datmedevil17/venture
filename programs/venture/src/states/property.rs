use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Property {
    pub property_id: u64,
    pub owner: Pubkey,
    pub mint: Pubkey, // NFT mint address
    #[max_len(64)]
    pub title: String,
    #[max_len(512)]
    pub description: String,
    #[max_len(256)]
    pub image_url: String,
    #[max_len(128)]
    pub location: String,
    #[max_len(32)]
    pub property_type: String,
    pub size_sqft: u64,
    pub bedrooms: u8,
    pub bathrooms: u8,
    pub year_built: u16,
    pub created_at: u64,
    pub is_listed: bool,
    pub list_price: u64,
    pub listing_type: u8, // 0: Direct Sale, 1: Auction
}