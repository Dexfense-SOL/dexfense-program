use anchor_lang::prelude::*;

use crate::state::GameResultSummary;
use crate::constants::BACKEND_SIGNER; // ← 이게 꼭 필요해요!

#[derive(Accounts)]
pub struct InitializeGameResultSummary<'info> {
    #[account(
        init,
        seeds = [b"game_result_summary"],
        bump,
        payer = admin,
        space = 8 + std::mem::size_of::<GameResultSummary>(),
    )]
    pub game_result_summary: Account<'info, GameResultSummary>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(address = BACKEND_SIGNER)]
    pub backend: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_game_result_summary(
    ctx: Context<InitializeGameResultSummary>,
    target_kill_easy: u64,
    target_kill_normal: u64,
    target_kill_hard: u64,
) -> Result<()> {
    let summary = &mut ctx.accounts.game_result_summary;

    summary.easy_count = 0;
    summary.easy_total_kills = 0;

    summary.normal_count = 0;
    summary.normal_total_kills = 0;

    summary.hard_count = 0;
    summary.hard_total_kills = 0;

    summary.difficulty_factor_easy = 100;
    summary.difficulty_factor_normal = 100;
    summary.difficulty_factor_hard = 100;

    summary.target_kill_easy = target_kill_easy;
    summary.target_kill_normal = target_kill_normal;
    summary.target_kill_hard = target_kill_hard;

    Ok(())
}
