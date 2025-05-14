use anchor_lang::prelude::*;

#[account]
pub struct GameSessionAccount {
    pub player: Pubkey,
    pub deposit_amount: u64,
    pub started_at: i64,
    pub kill_count: u32,
    pub is_active: bool,
    pub is_settled: bool,
    pub bump: u8,
}
