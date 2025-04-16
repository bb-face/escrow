use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::transfer_checked, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked}
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
			payer = maker,
			seeds = [b"escrow", escrow.maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
			bump = escrow.bump,
		)]
    pub escrow: Account<'info, Escrow>,

    #[account(
			payer = maker,
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
    pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
			self.escrow.self_inner(Escrow{
				seed: self.
    maker: self.maker.key(),
    mint_a: self.mint_a.key(),
    mint_b: self.mint_b.key(),
    receive,
    bump: bumps.escrow,
				
			})

        Ok(())
    }

		pub fn deposit(&mut self, deposit: u64) -> Result<()> {
			let transfer_accounts = TransferChecked {
    from: self.maker_ata_a.to_account_info(),
    mint: self.mint_b,
    to: self.vault.get_account_info(),
    authority: self.maker.to_account_info(),
}

let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

transfer_checked(cpi_ctx, amount: deposit, self.mint_a.decimals);
}
