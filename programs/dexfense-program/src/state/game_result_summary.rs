use anchor_lang::prelude::*;

#[account]
pub struct GameResultSummary {
    pub easy_count: u64,
    pub easy_total_kills: u64,

    pub normal_count: u64,
    pub normal_total_kills: u64,

    pub hard_count: u64,
    pub hard_total_kills: u64,
    
    pub difficulty_factor_easy: u64,   // 기본값 100 = 1.0배
    pub difficulty_factor_normal: u64,
    pub difficulty_factor_hard: u64,

    pub target_kill_easy: u64,         // 예: 25
    pub target_kill_normal: u64,       // 예: 20
    pub target_kill_hard: u64,         // 예: 15
}
