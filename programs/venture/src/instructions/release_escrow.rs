use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};
use crate::constants::{ESCROW_SEED, PROPERTY_SEED};
use crate::errors::ErrorCode;
use crate::states::{Escrow, Property, MarketplaceState};

pub fn release_escrow(
    ctx: Context<ReleaseEscrowCtx>,
    escrow_id: u64,
    release_to_seller: bool,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    let property = &mut ctx.accounts.property;
    let state = &ctx.accounts.marketplace_state;
    let authority = &ctx.accounts.authority;

    // Validations
    if escrow.escrow_id != escrow_id {
        return Err(ErrorCode::EscrowNotFound.into());
    }
    if escrow.is_completed {
        return Err(ErrorCode::EscrowAlreadyCompleted.into());
    }
    if escrow.deposited_amount == 0 {
        return Err(ErrorCode::InsufficientEscrowBalance.into());
    }

    // Only buyer, seller, or marketplace admin can release escrow
    if authority.key() != escrow.buyer &&
       authority.key() != escrow.seller &&
       authority.key() != state.admin {
        return Err(ErrorCode::NotAuthorizedToReleaseEscrow.into());
    }

    let release_amount = escrow.deposited_amount;
    let platform_fee = (release_amount * state.platform_fee) / 10000;
    let net_amount = release_amount - platform_fee;

    if release_to_seller {
        // Release funds to seller and transfer NFT to buyer
        **escrow.to_account_info().try_borrow_mut_lamports()? -= release_amount;
        **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += net_amount;
        **ctx.accounts.platform_treasury.to_account_info().try_borrow_mut_lamports()? += platform_fee;

        // Transfer NFT to buyer
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        // Update property ownership
        property.owner = escrow.buyer;
        property.is_listed = false;
        escrow.released_to_seller = true;
        msg!("Escrow released to seller. Property transferred to buyer.");
    } else {
        // Release funds back to buyer (refund scenario)
        **escrow.to_account_info().try_borrow_mut_lamports()? -= release_amount;
        **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? += release_amount;
        escrow.released_to_seller = false;
        msg!("Escrow refunded to buyer.");
    }

    escrow.is_completed = true;
    escrow.deposited_amount = 0;

    Ok(())
}

#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct ReleaseEscrowCtx<'info> {
    pub marketplace_state: Account<'info, MarketplaceState>,
    
    #[account(
        mut,
        seeds = [
            ESCROW_SEED,
            escrow_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    
    #[account(
        mut,
        seeds = [
            PROPERTY_SEED,
            escrow.property_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub property: Account<'info, Property>,
    
    /// CHECK: Buyer account
    #[account(mut)]
    pub buyer: UncheckedAccount<'info>,
    
    /// CHECK: Seller account
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    
    /// CHECK: This is a token account owned by the seller containing the NFT
    #[account(mut)]
    pub seller_token_account: UncheckedAccount<'info>,
    
    /// CHECK: This is a token account owned by the buyer to receive the NFT
    #[account(mut)]
    pub buyer_token_account: UncheckedAccount<'info>,
    
    /// CHECK: Platform treasury
    #[account(mut)]
    pub platform_treasury: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}