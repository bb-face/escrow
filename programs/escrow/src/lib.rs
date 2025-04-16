#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
use instructions::*;

mod state;

declare_id!("6GucshZBVJ9oDhSaLJm6RproW1Hj28NGkFUnwfuiAn4m");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn take(ctx: Context<Make>, seed: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        // ctx.accounts.withdraw_and_close_vault()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Make {}
