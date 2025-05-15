// programs/dexfense-program/src/instructions/initialize_treasury.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, InitializeAccount, initialize_account};

use crate::constants::BACKEND_SIGNER;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(
        init,
        seeds = [b"treasury"],
        bump,
        payer = admin,
        token::mint = token_mint,
        token::authority = treasury_authority,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA signer only
    #[account(
        seeds = [b"treasury_authority"],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(address = BACKEND_SIGNER)]
    pub backend: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
    // Nothing to initialize inside logic — handled by macro
    Ok(())
}
