use anchor_lang::prelude::*;  // ❗ 필수

use anchor_spl::token::Mint;

use crate::state::PoolAccount;  // PoolAccount 구조체 사용 시 필요

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<PoolAccount>(),
    )]
    pub pool: Account<'info, PoolAccount>,

    /// CHECK: will be derived in logic and checked via seeds
    #[account(mut)]
    pub token_a_vault: AccountInfo<'info>,
    /// CHECK: same
    #[account(mut)]
    pub token_b_vault: AccountInfo<'info>,

    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,

    /// CHECK: PDA will be used to sign vault ops later
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handle_initialize_pool<'info>(ctx: Context<InitializePool<'info>>) -> Result<()> {

    let pool = &mut ctx.accounts.pool;
    let pool_key = pool.key(); // 여기서 immutable 참조 종료됨
    pool.token_a_mint = ctx.accounts.token_a_mint.key();
    pool.token_b_mint = ctx.accounts.token_b_mint.key();
    pool.token_a_vault = ctx.accounts.token_a_vault.key();
    pool.token_b_vault = ctx.accounts.token_b_vault.key();
    pool.authority = ctx.accounts.authority.key();
    
    let (_, bump) = Pubkey::find_program_address(
        &[b"authority", pool_key.as_ref()],
        ctx.program_id,
    );
    
    pool.bump = bump;

    Ok(())
}