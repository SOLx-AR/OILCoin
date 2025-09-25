use anchor_lang::prelude::*;

use super::{ADMIN};

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut, address = ADMIN)]
    pub admin: Signer<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + Price::INIT_SPACE,
        seeds = [b"vault_authority"],
        bump,
    )]
    pub price: Account<'info, Price>,
    pub system_program: Program<'info, System>,
}


impl <'info> UpdatePrice<'info> {
   pub fn update(&mut self, bumps: &UpdatePriceBumps ,price: u64,exponent:u64, fee: u64) -> Result<()> {
        self.price.set_inner(Price {
            price,
            fee,
            exponent,
            bump: bumps.price,
        });
    }
}
