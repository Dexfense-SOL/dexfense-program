use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};

use crate::state::{GameSessionAccount, GameResultSummary};
use crate::error::ErrorCode;
use crate::constants;

// DEX CPI 모듈 import
use simple_amm_dex::cpi::swap;
use simple_amm_dex::cpi::accounts::Swap as AmmSwap;
use simple_amm_dex::program::SimpleAmmDex;

#[derive(Accounts)]
pub struct SwapAndSettle<'info> {
    #[account(
        mut,
        has_one = vault,
        constraint = game_session.is_verified @ ErrorCode::Unverified,
        constraint = !game_session.is_settled @ ErrorCode::AlreadySettled,
    )]
    pub game_session: Account<'info, GameSessionAccount>,

    #[account(
        mut,
        constraint = vault.mint == pool_token_from.mint @ ErrorCode::InvalidPair,
    )]
    pub vault: Account<'info, TokenAccount>,

    /// CHECK: PDA signer
    #[account(
        seeds = [b"vault_authority", game_session.key().as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub pool_token_from: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_to: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = reward_token_account.mint == pool_token_to.mint @ ErrorCode::InvalidPair,
    )]
    pub reward_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    /// CHECK: CPI only
    pub pool: AccountInfo<'info>,

    /// CHECK: CPI only
    pub pool_authority: AccountInfo<'info>,

    /// CHECK: Backend signer used only for authorization, not accessed directly
    #[account(signer)]
    pub backend: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub simple_amm_dex_program: Program<'info, SimpleAmmDex>,

    #[account(mut)]
    pub game_result_summary: Account<'info, GameResultSummary>,
}

pub fn handle_swap_and_settle<'info>(
    ctx: Context<SwapAndSettle<'info>>,
    amount_in: u64,
    is_a_to_b: bool,
    swapped_amount: u64,
) -> Result<()> {
    let game_session = &mut ctx.accounts.game_session;
    let summary = &mut ctx.accounts.game_result_summary;

    let game_session_key = game_session.key();
    let vault_signer_seeds: &[&[u8]] = &[
        b"vault_authority",
        game_session_key.as_ref(),
        &[ctx.bumps.vault_authority],
    ];
    let signer_seeds: &[&[&[u8]]] = &[vault_signer_seeds];

    require!(
        ctx.accounts.vault.mint == ctx.accounts.pool_token_from.mint,
        ErrorCode::InvalidPair
    );

    let cpi_transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.pool_token_from.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        },
        signer_seeds,
    );
    transfer(cpi_transfer_ctx, amount_in)?;

    let cpi_swap_ctx = CpiContext::new_with_signer(
        ctx.accounts.simple_amm_dex_program.to_account_info(),
        AmmSwap {
            pool: ctx.accounts.pool.to_account_info(),
            pool_token_from: ctx.accounts.pool_token_from.to_account_info(),
            pool_token_to: ctx.accounts.pool_token_to.to_account_info(),
            user_token_from: ctx.accounts.pool_token_from.to_account_info(),
            user_token_to: ctx.accounts.reward_token_account.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
            user: ctx.accounts.vault_authority.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
        signer_seeds,
    );
    swap(cpi_swap_ctx, amount_in, is_a_to_b)?;

    let multiplier = (game_session.kill_count as u128).saturating_mul(100) / 400;
    let expected_reward = amount_in as u128 * multiplier / 100;

    if expected_reward <= swapped_amount as u128 {
        // 생략 가능
    } else {
        let missing = (expected_reward - swapped_amount as u128) as u64;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury_token_account.to_account_info(),
                to: ctx.accounts.reward_token_account.to_account_info(),
                authority: ctx.accounts.backend.to_account_info(),
            },
        );
        transfer(cpi_ctx, missing)?;
    }

    if swapped_amount as u128 > expected_reward {
        let refund = (swapped_amount as u128 - expected_reward) as u64;
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.reward_token_account.to_account_info(),
                to: ctx.accounts.treasury_token_account.to_account_info(),
                authority: ctx.accounts.backend.to_account_info(),
            },
        );
        transfer(cpi_ctx, refund)?;
    }

    match game_session.difficulty {
        crate::constants::Difficulty::Easy => {
            summary.easy_count += 1;
            summary.easy_total_kills += game_session.kill_count as u64;
        }
        crate::constants::Difficulty::Normal => {
            summary.normal_count += 1;
            summary.normal_total_kills += game_session.kill_count as u64;
        }
        crate::constants::Difficulty::Hard => {
            summary.hard_count += 1;
            summary.hard_total_kills += game_session.kill_count as u64;
        }
    }

    game_session.is_settled = true;
    Ok(())
}