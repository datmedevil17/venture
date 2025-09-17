use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MarketplaceState {
    pub initialized: bool,
    pub total_properties: u64,
    pub total_auctions: u64,
    pub total_escrows: u64,
    pub platform_fee: u64, // in basis points
    pub platform_treasury: Pubkey,
    pub admin: Pubkey,
}