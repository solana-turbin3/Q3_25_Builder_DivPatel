use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked, CloseAccount, Mint, TokenAccount, close_account, TokenInterface, TransferChecked}};

use crate::Escrow;


#[derive(Accounts)]
pub struct Refund<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority=maker,
        associated_token::token_program = token_program

    )]
    pub maker_ata_a:InterfaceAccount<'info , TokenAccount>,
    #[account(
        mut,
        has_one = mint_a,
        has_one = maker,
        seeds = [b"escrow",maker.key().as_ref() , escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow:Account<'info , Escrow>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info,TokenAccount>,

    pub token_program: Interface<'info , TokenInterface>,
    pub system_program:Program<'info,System>,
    pub associated_token_program:Program<'info,AssociatedToken>

}



impl<'info> Refund<'info> {
    pub fn refund_and_close_vault(&mut self) -> Result<()> {

        let seeds = &[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let transfer_cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            TransferChecked{
                from:self.vault.to_account_info(),
                mint:self.mint_a.to_account_info(),
                to:self.maker.to_account_info(),
                authority:self.escrow.to_account_info(),
            },
            signer_seeds
        );

        transfer_checked(transfer_cpi_context, self.vault.amount, self.mint_a.decimals)?;

        let close_cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), 
        CloseAccount{
            account:self.vault.to_account_info(),
            destination:self.maker.to_account_info(),
            authority:self.escrow.to_account_info()
        },
        signer_seeds);

        close_account(close_cpi_context)
    }
}