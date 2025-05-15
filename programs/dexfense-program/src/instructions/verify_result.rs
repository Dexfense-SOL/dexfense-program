use anchor_lang::prelude::*;
use crate::state::GameSessionAccount;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct VerifyResult<'info> {
    #[account(
        mut,
        seeds = [b"game_session", session.player.as_ref(), &nonce.to_le_bytes()],
        bump,
        constraint = session.is_active == false @ ErrorCode::GameStillActive,
        constraint = session.is_verified == false @ ErrorCode::AlreadyVerified,
    )]
    pub session: Account<'info, GameSessionAccount>,

    #[account(address = crate::constants::BACKEND_SIGNER)]
    pub backend_signer: Signer<'info>,
}

pub fn handle_verify_result<'info>(
    ctx: Context<VerifyResult<'info>>,
    kill_count: u64,
    _nonce: u64,
) -> Result<()> {
    let session = &mut ctx.accounts.session;

    session.is_verified = session.kill_count == kill_count as u32;

    Ok(())
}
