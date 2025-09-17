use anchor_lang::prelude::*;
use crate::constants::MARKETPLACE_STATE_SEED;
use crate::errors::ErrorCode;
use crate::states::MarketplaceState;

pub fn update_platform_settings(
    ctx: Context<UpdatePlatformSettingsCtx>,
    new_fee: Option<u64>,
    new_treasury: Option<Pubkey>,
) -> Result<()> {
    let state = &mut ctx.accounts.marketplace_state;
    let admin = &ctx.accounts.admin;

    // Only admin can update settings
    if state.admin != admin.key() {
        return Err(ErrorCode::NotAuthorizedToReleaseEscrow.into());
    }

    // Update platform fee if provided
    if let Some(fee) = new_fee {
        if fee > 1000 { // Max 10% fee
            return Err(ErrorCode::InvalidEscrowAmount.into());
        }
        state.platform_fee = fee;
        msg!("Platform fee updated to {}", fee);
    }

    // Update treasury address if provided
    if let Some(treasury) = new_treasury {
        state.platform_treasury = treasury;
        msg!("Platform treasury updated to {}", treasury);
    }

    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePlatformSettingsCtx<'info> {
    #[account(
        mut,
        seeds = [MARKETPLACE_STATE_SEED],
        bump
    )]
    pub marketplace_state: Account<'info, MarketplaceState>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
}