use anchor_lang::prelude::*;

#[account]
pub struct PoolAccount {
    pub token_a_mint: Pubkey,       // Token A mint
    pub token_b_mint: Pubkey,       // Token B mint
    pub token_a_vault: Pubkey,      // Token A vault
    pub token_b_vault: Pubkey,      // Token B vault
    pub authority: Pubkey,          // Vault authority (PDA)
    pub bump: u8,                   // PDA bump for authority
}
