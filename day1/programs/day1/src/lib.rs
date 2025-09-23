use anchor_lang::prelude::*;

declare_id!("Cgi8X7TvLQtPq1rc57exeit33F7ykD62GDTM9oLnJHtu");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize2(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
