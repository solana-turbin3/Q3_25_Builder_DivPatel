#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("26p9wRdb4s1gV9TnkzmoTHtR5PfEqWkT37YongR6gk9m");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment> , amount:u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment> , amount:u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
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
        seeds=[b"state",user.key.as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault_state: Account<'info , Vault_state>,
    #[account(
        mut,
        seeds = [b"vault",vault_state.key().as_ref()],
        bump = vault_state.vault_bump
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

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
        ];

        let signer_seeds = &[&seeds[..]];
        let cpi_context = CpiContext::new_with_signer(self.system_program.to_account_info(), Transfer{
            from:self.vault.to_account_info(),
            to:self.user.to_account_info()
        },signer_seeds);

        transfer(cpi_context, amount)?;

        Ok(())
    }
}




#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        seeds = [b"state" , user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info , Vault_state>,
    #[account(
        mut , 
        seeds = [b"vault" , vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault : SystemAccount<'info>,
    pub system_program:Program<'info,System>
}


impl<'info> Close<'info> {
    pub fn close(&mut self ) -> Result<()> {

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(self.system_program.to_account_info(), Transfer{
            from:self.vault.to_account_info(),
            to:self.user.to_account_info(),
        }, signer_seeds);

        transfer(cpi_context, self.vault.lamports())?;

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