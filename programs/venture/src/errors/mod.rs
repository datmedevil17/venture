use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Marketplace already initialized")]
    AlreadyInitialized,
    
    #[msg("Property title too long (max 64 characters)")]
    TitleTooLong,
    
    #[msg("Property description too long (max 512 characters)")]
    DescriptionTooLong,
    
    #[msg("Image URL too long (max 256 characters)")]
    ImageUrlTooLong,
    
    #[msg("Location too long (max 128 characters)")]
    LocationTooLong,
    
    #[msg("Property type too long (max 32 characters)")]
    PropertyTypeTooLong,
    
    #[msg("Invalid property price (minimum 1 SOL)")]
    InvalidPropertyPrice,
    
    #[msg("Property not found")]
    PropertyNotFound,
    
    #[msg("Property not listed for sale")]
    PropertyNotListed,
    
    #[msg("Not property owner")]
    NotPropertyOwner,
    
    #[msg("Property already listed")]
    PropertyAlreadyListed,
    
    #[msg("Invalid listing type")]
    InvalidListingType,
    
    #[msg("Auction not found")]
    AuctionNotFound,
    
    #[msg("Auction already ended")]
    AuctionAlreadyEnded,
    
    #[msg("Auction not ended yet")]
    AuctionNotEnded,
    
    #[msg("Bid amount too low")]
    BidAmountTooLow,
    
    #[msg("Cannot bid on own auction")]
    CannotBidOnOwnAuction,
    
    #[msg("Reserve price not met")]
    ReservePriceNotMet,
    
    #[msg("Invalid auction duration")]
    InvalidAuctionDuration,
    
    #[msg("Escrow not found")]
    EscrowNotFound,
    
    #[msg("Escrow already completed")]
    EscrowAlreadyCompleted,
    
    #[msg("Insufficient escrow balance")]
    InsufficientEscrowBalance,
    
    #[msg("Not authorized to release escrow")]
    NotAuthorizedToReleaseEscrow,
    
    #[msg("Conditions too long (max 256 characters)")]
    ConditionsTooLong,
    
    #[msg("Invalid escrow amount")]
    InvalidEscrowAmount,
}