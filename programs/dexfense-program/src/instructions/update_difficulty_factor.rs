use anchor_lang::prelude::*;
use crate::state::GameResultSummary;
use crate::constants::BACKEND_SIGNER;

#[derive(Accounts)]
pub struct UpdateDifficultyFactors<'info> {
    #[account(mut)]
    pub game_result_summary: Account<'info, GameResultSummary>,

    #[account(address = BACKEND_SIGNER)]
    pub authority: Signer<'info>, // ✅ 백엔드만 가능하게 제한
}

pub fn handle_update_difficulty_factors<'info>(
    ctx: Context<UpdateDifficultyFactors<'info>>,
    easy: u64,
    normal: u64,
    hard: u64,
) -> Result<()> {
    let summary = &mut ctx.accounts.game_result_summary;

    summary.target_kill_easy = easy;
    summary.target_kill_normal = normal;
    summary.target_kill_hard = hard;


    msg!("✅ Difficulty target updated: Easy={}, Normal={}, Hard={}", easy, normal, hard);

    Ok(())
}
