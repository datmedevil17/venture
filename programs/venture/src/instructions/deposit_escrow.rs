
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use crate::constants::ESCROW_SEED;
use crate::errors::ErrorCode;
use crate::states::Escrow;

pub fn deposit_escrow(
    ctx: Context<DepositEscrowCtx>,
    escrow_id: u64,
    amount: u64,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    let depositor = &ctx.accounts.depositor;

    // Validations
    if escrow.escrow_id != escrow_id {
        return Err(ErrorCode::EscrowNotFound.into());
    }
    if escrow.is_completed {
        return Err(ErrorCode::EscrowAlreadyCompleted.into());
    }
    if amount == 0 {
        return Err(ErrorCode::InvalidEscrowAmount.into());
    }
    
    // Only buyer or seller can deposit
    if depositor.key() != escrow.buyer && depositor.key() != escrow.seller {
        return Err(ErrorCode::NotAuthorizedToReleaseEscrow.into());
    }

    // Transfer funds to escrow account
    let transfer_instruction = system_instruction::transfer(
        &depositor.key(),
        &escrow.key(),
        amount,
    );

    invoke(
        &transfer_instruction,
        &[
            depositor.to_account_info(),
            escrow.to_account_info(),
        ],
    )?;

    // Update escrow balance
    escrow.deposited_amount += amount;

    msg!("Deposited {} lamports to escrow {}", amount, escrow_id);

    Ok(())
}

#[derive(Accounts)]
#[instruction(escrow_id: u64)]
pub struct DepositEscrowCtx<'info> {
    #[account(
        mut,
        seeds = [
            ESCROW_SEED,
            escrow_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}