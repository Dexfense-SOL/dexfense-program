use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::GameResultSummary;

#[derive(Accounts)]
pub struct AdjustFactors<'info> {
    #[account(mut)]
    pub game_result_summary: Account<'info, GameResultSummary>,

    #[account(address = BACKEND_SIGNER)]
    pub backend: Signer<'info>,
}

pub fn handle_adjust_difficulty_factor(
    ctx: Context<AdjustFactors>,
    difficulty: Difficulty,
) -> Result<()> {
    let summary = &mut ctx.accounts.game_result_summary;

    match difficulty {
        Difficulty::Easy => {
            if summary.easy_count < 50 {
                msg!("⏳ Not enough Easy data. Count = {}", summary.easy_count);
                return Ok(());
            }
            let avg = summary.easy_total_kills / summary.easy_count;
            let target = summary.target_kill_easy;
            let mut factor = summary.difficulty_factor_easy;
            factor = adjust_factor(avg, target, factor);
            summary.difficulty_factor_easy = factor;
            summary.easy_count = 0;
            summary.easy_total_kills = 0;
            msg!("📊 Easy Factor updated: avg={}, target={}, factor={}", avg, target, factor);
        }

        Difficulty::Normal => { 
            if summary.normal_count < 50 {
                msg!("⏳ Not enough Easy data. Count = {}", summary.normal_count);
                return Ok(());
            }
            let avg = summary.normal_total_kills / summary.normal_count;
            let target = summary.target_kill_normal;
            let mut factor = summary.difficulty_factor_normal;
            factor = adjust_factor(avg, target, factor);
            summary.difficulty_factor_normal = factor;
            summary.normal_count = 0;
            summary.normal_total_kills = 0;
            msg!("📊 Normal Factor updated: avg={}, target={}, factor={}", avg, target, factor);
         }

        Difficulty::Hard => { 
            if summary.hard_count < 50 {
                msg!("⏳ Not enough Easy data. Count = {}", summary.hard_count);
                return Ok(());
            }
            let avg = summary.hard_total_kills / summary.hard_count;
            let target = summary.target_kill_hard;
            let mut factor = summary.difficulty_factor_hard;
            factor = adjust_factor(avg, target, factor);
            summary.difficulty_factor_hard = factor;
            summary.hard_count = 0;
            summary.hard_total_kills = 0;
            msg!("📊 Hard Factor updated: avg={}, target={}, factor={}", avg, target, factor);    
        }
    }

    Ok(())
}

fn adjust_factor(avg: u64, target: u64, current: u64) -> u64 {
    if avg < target {
        current.saturating_mul(105).saturating_div(100)
    } else if avg > target {
        current.saturating_mul(95).saturating_div(100)
    } else {
        current
    }
}
