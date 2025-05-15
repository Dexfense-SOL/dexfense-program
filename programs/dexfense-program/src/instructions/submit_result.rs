use anchor_lang::prelude::*;

use crate::state::GameSessionAccount;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct SubmitResult<'info> {
    #[account(
        mut,
        seeds = [b"game_session", player.key().as_ref(), &nonce.to_le_bytes()],
        bump,
        constraint = game_session.is_active == true @ ErrorCode::AlreadyClosed,
    )]
    pub game_session: Account<'info, GameSessionAccount>,

    #[account(mut)]
    pub player: Signer<'info>,
}

pub fn handle_submit_result<'info>(
    ctx: Context<SubmitResult<'info>>,
    kill_count: u64,
    _nonce: u64,
) -> Result<()> {
    let session = &mut ctx.accounts.game_session;

    session.kill_count = kill_count as u32;
    session.is_verified = false;
    session.is_active = false;

    Ok(())
}
