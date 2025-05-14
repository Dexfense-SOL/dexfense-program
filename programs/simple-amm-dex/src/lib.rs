use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

pub mod state;
pub mod instructions;
pub mod error;


use instructions::*;

declare_id!("8Y28XVxZdKpN47nh4KVDKYNqJ8WkzVtXrXqezECHASCD");  // 임시 placeholder

#[program]
pub mod simple_amm_dex {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        handle_initialize_pool(ctx)
    }

    pub fn swap(ctx: Context<Swap>, amount_in: u64, is_a_to_b: bool) -> Result<()> {
        handle_swap(ctx, amount_in, is_a_to_b)
    }
}
