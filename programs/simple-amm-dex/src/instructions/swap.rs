use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::state::PoolAccount;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub pool: Account<'info, PoolAccount>,

    #[account(mut)]
    pub user_token_from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_to: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub pool_token_to: Account<'info, TokenAccount>,

    /// CHECK: used only as signer in CPI
    pub authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    /// CHECK: This is the user's main signer account and only used as a signer for CPI
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

pub fn handle_swap<'info>(
    ctx: Context<Swap<'info>>,
    amount_in: u64,
    is_a_to_b: bool,
) -> Result<()> {
    let pool = &ctx.accounts.pool;

    // 준비: 방향별 분기
    let (input_vault, output_vault, user_input, user_output) = if is_a_to_b {
        (
            &ctx.accounts.pool_token_from,
            &ctx.accounts.pool_token_to,
            &ctx.accounts.user_token_from,
            &ctx.accounts.user_token_to,
        )
    } else {
        (
            &ctx.accounts.pool_token_to,
            &ctx.accounts.pool_token_from,
            &ctx.accounts.user_token_to,
            &ctx.accounts.user_token_from,
        )
    };

    // x * y = k 기반 계산
    let reserve_in = input_vault.amount;
    let reserve_out = output_vault.amount;

    require!(reserve_in > 0 && reserve_out > 0, ErrorCode::InsufficientLiquidity);

    let amount_out = (amount_in as u128)
        .checked_mul(reserve_out as u128)
        .unwrap()
        .checked_div((reserve_in as u128).checked_add(amount_in as u128).unwrap())
        .unwrap() as u64;

    // ✅ 사용자 → 풀 입금
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: user_input.to_account_info(),
            to: input_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    token::transfer(cpi_ctx, amount_in)?;

    // ✅ 풀 → 사용자 출금 (PDA signer 사용)
    let pool_key = ctx.accounts.pool.key();
    let seeds = &[b"authority", pool_key.as_ref(), &[pool.bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: output_vault.to_account_info(),
            to: user_output.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        signer,
    );
    token::transfer(cpi_ctx, amount_out)?;

    Ok(())
}
