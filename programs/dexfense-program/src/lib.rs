use anchor_lang::prelude::*;

declare_id!("6hr1mocCrFQGHpi28GNJ5xpHZfuov8bWF9YCfHQw1KkC");

#[program]
pub mod dexfense_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
