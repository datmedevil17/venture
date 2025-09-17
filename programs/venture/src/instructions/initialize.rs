use anchor_lang::prelude::*;
use crate::constants::{ANCHOR_DISCRIMINATOR_SIZE, MARKETPLACE_STATE_SEED};
use crate::errors::ErrorCode;
use crate::states::MarketplaceState;

pub fn initialize(ctx: Context<InitializeCtx>) -> Result<()> {
    let state = &mut ctx.accounts.marketplace_state;
    let admin = &ctx.accounts.admin;

    if state.initialized {
        return Err(ErrorCode::AlreadyInitialized.into());
    }

    state.initialized = true;
    state.total_properties = 0;
    state.total_auctions = 0;
    state.total_escrows = 0;
    state.platform_fee = 250; // 2.5%
    state.platform_treasury = admin.key();
    state.admin = admin.key();

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCtx<'info> {
    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR_SIZE + MarketplaceState::INIT_SPACE,
        seeds = [MARKETPLACE_STATE_SEED],
        bump
    )]
    pub marketplace_state: Account<'info, MarketplaceState>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}