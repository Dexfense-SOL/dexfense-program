use anchor_lang::prelude::*;
use crate::state::game_session::*;
use crate::constants::Difficulty;
use crate::events::GameStarted;

#[derive(Accounts)]
#[instruction(difficulty: Difficulty, nonce: u64)]
pub struct InitializeGame<'info> {
    #[account(
        init,
        seeds = [b"game_session", player.key().as_ref(), &nonce.to_le_bytes()],
        bump,
        payer = player,
        space = 8 + std::mem::size_of::<GameSessionAccount>(),
    )]
    pub game_session: Account<'info, GameSessionAccount>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_game(
    ctx: Context<InitializeGame>,
    difficulty: Difficulty,
    nonce: u64,
) -> Result<()> {
    let game = &mut ctx.accounts.game_session;
    let clock = Clock::get()?;

    game.player = ctx.accounts.player.key();
    game.deposit_amount = difficulty.deposit_amount();
    game.started_at = clock.unix_timestamp;
    game.kill_count = 0;
    game.is_active = true;
    game.is_settled = false;
    game.bump = *ctx.bumps.get("game_session").unwrap();

    emit!(GameStarted {
        player: game.player,
        game_session: ctx.accounts.game_session.key(),
        deposit_amount: game.deposit_amount,
        started_at: game.started_at,
    });

    Ok(())
}
