use anchor_lang::prelude::*;

// The `#[account]` attribute is used to define an account on-chain.
#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64, // Seeds and bump are used to derive the PDA for the escrow account
    pub bump: u8,
    pub maker: Pubkey, // The maker of the escrow: the account that will initialise the escrow,
    // deposit the funds and receive the funds back
    pub mint_a: Pubkey, // The mint of the token that the maker will deposit
    pub mint_b: Pubkey, // The mint of the token that the maker will receive
    /* Note about mint_a and mint_b:
        They are not ATAs, they are Mint accounts. A mint account can be USDC for example
        or Wrapper SOL.
        ATAs are the specific token holders of the mint accounts.
    */
    pub receive: u64, // The amount of the token that the maker will receive
}
