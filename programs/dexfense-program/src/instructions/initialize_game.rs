use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, Mint, transfer};

use crate::error::ErrorCode;

use crate::state::GameSessionAccount;
use crate::events::GameStarted;
use crate::constants::Difficulty;
use crate::constants::DFP_TOKEN_MINT;

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

    #[account(
        init,
        seeds = [b"session_vault", game_session.key().as_ref()],
        bump,
        payer = player,
        token::mint = token_mint,
        token::authority = vault_authority,
    )]
    pub vault: Account<'info, TokenAccount>,

    /// CHECK: signer 없이 PDA로만 사용
    #[account(
        seeds = [b"vault_authority", game_session.key().as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize_game<'info>(
    ctx: Context<InitializeGame<'info>>,
    difficulty: Difficulty,
    nonce: u64,
) -> Result<()> {
    let clock = Clock::get()?;
    let session = &mut ctx.accounts.game_session;
    let deposit_amount = difficulty.deposit_amount();

    // ✅ 민트 제한 (필수)
    require!(
        ctx.accounts.token_mint.key() == DFP_TOKEN_MINT,
        ErrorCode::InvalidMint
    );

    // ✅ user → vault 로 토큰 전송
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.player.to_account_info(),
        },
    );
    transfer(cpi_ctx, deposit_amount)?;

    // ✅ GameSession 초기화
    session.player = ctx.accounts.player.key();
    session.deposit_amount = deposit_amount;
    session.started_at = clock.unix_timestamp;
    session.kill_count = 0;
    session.is_active = true;
    session.is_verified = false;
    session.is_settled = false;
    session.difficulty = difficulty; // ✅ 추가
    session.vault = ctx.accounts.vault.key();
    session.bump = ctx.bumps.game_session;

    emit!(GameStarted {
        player: session.player,
        game_session: session.key(),
        deposit_amount: session.deposit_amount,
        started_at: session.started_at,
    });

    Ok(())
}

