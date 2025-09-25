use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    tokenInterface::{Mint, TokenAccount, mint_to, Transfer},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use super::MINT_USDC;
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, address = ADMIN)]
    pub admin: Signer<'info>,
    #[account(mut, address = MINT_USDC)]
    pub mint_USDC: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        associated_token::mint = mint_USDC,
        associated_token::authority = admin,
        associated_token::token_program = token_program
    )]
    pub admin_ata_USDC: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        associated_token::mint = mint_USDC,
        associated_token::authority = price,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [b"vault_authority"],
        bump = price.bump,
    )]
    pub price: Account<'info, Price>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl <'info> Withdraw<'info> {
   pub fn Withdraw(&self, amount: u64) -> Result<u64> {
       
        let cpi_accounts = anchor_spl::token::Transfer {
            from: self.vaut.to_account_info().clone(),
            to: self.admin_ata_USDC.to_account_info().clone(),
            authority: self.price.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info().clone();
        let signer_seeds = &[&[b"vault_authority", &[self.price.bump]][..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)

    }
}