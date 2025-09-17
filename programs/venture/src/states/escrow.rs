use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub escrow_id: u64,
    pub property_id: u64,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub amount: u64,
    pub deposited_amount: u64,
    #[max_len(256)]
    pub conditions: String,
    pub created_at: u64,
    pub is_completed: bool,
    pub released_to_seller: bool,
}