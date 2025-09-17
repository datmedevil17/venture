
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
pub const PROPERTY_SEED: &[u8] = b"property";
pub const AUCTION_SEED: &[u8] = b"auction";
pub const BID_SEED: &[u8] = b"bid";
pub const ESCROW_SEED: &[u8] = b"escrow";
pub const MARKETPLACE_STATE_SEED: &[u8] = b"marketplace_state";

// Platform fees in basis points (100 = 1%)
pub const PLATFORM_FEE: u64 = 250; // 2.5%
pub const MIN_PROPERTY_PRICE: u64 = 1_000_000_000; // 1 SOL minimum
pub const MIN_BID_INCREMENT: u64 = 100_000_000; // 0.1 SOL minimum increment
pub const MAX_AUCTION_DURATION: u64 = 30 * 24 * 60 * 60; // 30 days in seconds