#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::{accounts::signer, prelude::*, system_program::{transfer, Transfer}};

declare_id!("26p9wRdb4s1gV9TnkzmoTHtR5PfEqWkT37YongR6gk9m");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer=user,
        seeds = [b"state" , user.key().as_ref()],
        bump,
        space = Vault_state::INIT_SPACE,
    )]
    pub vault_state: Account<'info , Vault_state>,
    #[account(
        mut,
        seeds = [b"vault",vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program:Program<'info,System>

}


impl<'info> Initialize<'info> {
    pub fn initialize(&mut self , bumps:&InitializeBumps) -> Result<()> {
        
        let rent_exemption = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());

        let cpi_context = CpiContext::new(self.system_program.to_account_info(), Transfer{
            from:self.user.to_account_info(),
            to:self.vault.to_account_info()
        });

        transfer(cpi_context, rent_exemption)?;

        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct Payment<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        init,
        payer=user,
        seeds=[b"state",user.key.as_ref()],
        bumps = vault_state.vault_bump,
        space = Vault_state::INIT_SPACE
    )]
    pub vault_state: Account<'info , Vault_state>;
    #[account(
        mut,
        seeds = [b"vault",vault_state.key.as_ref()],
        bumps:vault_state.vault_bump
    )]
    pub vault : SystemAccount<'info>,
    pub system_program: Program<'info,System>
}


impl<'info> Payment<'info>{
    pub fn deposit(&mut self , amount: u64) -> Result<()> {

        let cpi_context = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer{
                from:self.user.to_account_info(),
                to:self.vault.to_account_info()
            }
        );

        transfer(cpi_context, amount)?;
        Ok(())
    }

    pub fn withdraw(&mut self , amount: u64) -> Result<()>  {

        let signer_seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
        ];

        let cpi_context = CpiContext::new_with_signer(self.system_program.to_account_info(), Transfer{
            from:self.vault.to_account_info(),
            to:self.user.to_account_info()
        },& [signer_seeds]);

        Ok(())
    }
}





#[account]
pub struct Vault_state{
    state_bump:u8,
    vault_bump:u8
}

impl Space for Vault_state{
    const INIT_SPACE:usize= 8 + 1 + 1;
}