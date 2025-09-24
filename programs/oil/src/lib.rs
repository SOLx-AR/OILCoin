use anchor_lang::prelude::*;

declare_id!("EJuzQGN7LgsyZopBupAd84NEgWct27odrQ5nvZkgVfzD");

#[program]
pub mod oil {
    use super::*;
//comprar
    pub fn mint(ctx: Context<Mint>, amount: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
//vender
   /*pub fn burn(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
     */ 
//retiro
   /* pub fn withdraw(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
//informar precio
    pub fn price(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    } */
}

#[derive(Accounts)]
pub struct Initialize {}
