use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
			mint::token_program = token_program
		)]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
			mint::token_program = token_program
		)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
			mut,
			associated_token::mint = mint_a,
			associated_token::authority = taker,
			associated_token::token_program = token_program,
		)]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
			seeds = [b"escrow", escrow.maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
			bump = escrow.bump,
		)]
    pub escrow: Account<'info, Escrow>,

    #[account(
			associated_token::mint = mint_a,
			associated_token::authority = escrow,
			associated_token::token_program = token_program,
		)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
  // Transfer the token from the taker to the maker ata;
  pub fn deposit(&mut self) -> Result<()> {
    let cpi_program = self.token_program.to_account_info();

    let cpi_accounts = TransferChecked {
      from: self.taker_ata_b.to_account_info(),
      mint: self.mint_b.to_account_info(),
      to: self.maker_ata_b.to_account_info(),
      authority: self.taker.to_account_info(),
    };

  }

  // Trasnfer the token from the escrow to the taker ata;
    pub fn release(&mut self) -> Result<()> {
      let cpi_program = self.token_program.to_account_info();

      let transfer_accounts = TransferChecked {
        from: self.vault.to_account_info(),
        mint: self.mint_b.to_account_info(),
        to: self.escrow.
        authority: self.vault.to_account_info(),
      }
        Ok(())
    }
}
