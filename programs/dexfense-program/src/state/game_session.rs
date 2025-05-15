use anchor_lang::prelude::*;
use crate::constants::Difficulty;

#[account]
pub struct GameSessionAccount {
    pub player: Pubkey,
    pub deposit_amount: u64,
    pub started_at: i64,
    pub kill_count: u32,
    pub is_verified: bool,
    pub is_active: bool,
    pub is_settled: bool,
    pub difficulty: Difficulty,
    pub vault: Pubkey, // ✅ 세션 참가비를 담고 있는 PDA vault 계정
    pub bump: u8,    
}