use anchor_lang::prelude::*;

declare_id!("26p9wRdb4s1gV9TnkzmoTHtR5PfEqWkT37YongR6gk9m");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
