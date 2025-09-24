use anchor_lang::prelude::*;

declare_id!("HvCnh8dsYywekZ8FPnRhro1JgQ5DuuWwtZrkhnFSeYjm");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You have send {}, {}", a,b);
         msg!("You said {:?}", message);
        Ok(())
    }
    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("your  array {:?}", arr);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
