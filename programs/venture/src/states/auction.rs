use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Auction {
    pub auction_id: u64,
    pub property_id: u64,
    pub seller: Pubkey,
    pub starting_price: u64,
    pub reserve_price: u64,
    pub current_bid: u64,
    pub highest_bidder: Option<Pubkey>,
    pub bid_count: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub is_ended: bool,
    pub winner: Option<Pubkey>,
}