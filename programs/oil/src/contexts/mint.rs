use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    tokenInterface::{Mint, TokenAccount, mint_to, Transfer},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use super::{MINT_USDC, MINT_OIL};
#[derive(Accounts)]
pub struct MintOIL<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, address = MINT_OIL)]
    pub mint_OIL: InterfaceAccount<'info, Mint>,
    #[account(mut, address = MINT_USDC)]
    pub mint_USDC: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_OIL,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_OIL: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_USDC,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_USDC: InterfaceAccount<'info, TokenAccount>,
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
    pub pyth_price: Box<Account<'info, PriceUpdateV2>>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl <'info> MintOIL<'info> {
   pub fn mint(&self, amount: u64) -> Result<u64> {
        let FEED_ID = get_feed_id_from_hex("1fd93efedb8b2db34465b5dbad8beca7288ad8bbcd47213ab1fdfe57ac86a240")?;
        let (pyth_price, conf, exponent) = match pyth_price.get_price_no_older_than(FEED_ID, 600).map_err(|_| error!(ErrorCode::InvalidPythPrice)) {
            Some(price) => (price.price, price.conf, price.expo),
            None => return Err(error!(ErrorCode::InvalidPythPrice)),
        };
        // 1% in basis points
        usdc_amount_to_pay = amount * pyth_price as u64 / 10u64.pow((-exponent) as u32) / 10000 * (10000 + self.price.fee);

        // Transfer USDC from user to vault
        let cpi_accounts = anchor_spl::token::Transfer {
            from: self.user_ata_USDC.to_account_info().clone(),
            to: self.vault.to_account_info().clone(),
            authority: self.user.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info().clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, usdc_amount_to_pay)?;

        // Mint OIL to user
        let cpi_accounts = anchor_spl::token::MintTo {
            mint: self.mint_OIL.to_account_info().clone(),
            to: self.user_ata_OIL.to_account_info().clone(),
            authority: self.price.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info().clone();
        let seeds = &[b"vault_authority".as_ref(), &[self.price.bump]];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        mint_to(cpi_ctx, amount)

    }
}