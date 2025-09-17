use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bid {
    pub auction_id: u64,
    pub bidder: Pubkey,
    pub amount: u64,
    pub timestamp: u64,
    pub is_winning: bool,
}
