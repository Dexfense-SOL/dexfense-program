use anchor_lang::prelude::*;

declare_id!("6hr1mocCrFQGHpi28GNJ5xpHZfuov8bWF9YCfHQw1KkC");

pub mod state;
pub mod constants;
pub mod events;
pub mod instructions;
pub mod error;

use constants::Difficulty;

use instructions::*;

#[program]
pub mod dexfense_program {
    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        difficulty: Difficulty,
        nonce: u64,
    ) -> Result<()> {
        handle_initialize_game(ctx, difficulty, nonce)
    }
    

    pub fn submit_result(
        ctx: Context<SubmitResult>,
        kill_count: u64,
        nonce: u64,
    ) -> Result<()> {
        handle_submit_result(ctx, kill_count, nonce)
    }

    pub fn verify_result(
        ctx: Context<VerifyResult>,
        kill_count: u64,
        nonce: u64,
    ) -> Result<()> {
        handle_verify_result(ctx, kill_count, nonce)
    }

    pub fn initialize_treasury(
        ctx: Context<InitializeTreasury>
    ) -> Result<()> {
        handle_initialize_treasury(ctx)
    }

    pub fn initialize_game_result_summary(
        ctx: Context<InitializeGameResultSummary>,
        target_kill_easy: u64,
        target_kill_normal: u64,
        target_kill_hard: u64,
    ) -> Result<()> {
        handle_initialize_game_result_summary(ctx, target_kill_easy, target_kill_normal, target_kill_hard)
    }

    pub fn update_difficulty_factors(
        ctx: Context<UpdateDifficultyFactors>,
        easy: u64,
        normal: u64,
        hard: u64,
    ) -> Result<()> {
        handle_update_difficulty_factors(ctx, easy, normal, hard)
    }

    pub fn adjust_difficulty_factor(
        ctx: Context<AdjustFactors>,
        difficulty: Difficulty,
    ) -> Result<()> {
        handle_adjust_difficulty_factor(ctx, difficulty)
    }
}
