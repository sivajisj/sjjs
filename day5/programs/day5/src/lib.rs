use anchor_lang::prelude::*;

declare_id!("GX8W6K2heHtqNU8nb8DS4xEeUkF3rcMKZ3QuDgGTRJcv");

#[program]
pub mod day5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
