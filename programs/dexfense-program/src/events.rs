use anchor_lang::prelude::*;

#[event]
pub struct GameStarted {
    pub player: Pubkey,
    pub game_session: Pubkey,
    pub deposit_amount: u64,
    pub started_at: i64,
}
