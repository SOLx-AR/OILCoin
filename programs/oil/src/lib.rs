use anchor_lang::prelude::*;

declare_id!("EJuzQGN7LgsyZopBupAd84NEgWct27odrQ5nvZkgVfzD");

#[program]
pub mod oil {
    use super::*;
//comprar
    pub fn mint(ctx: Context<MintOIL>, amount: u64) -> Result<()> {
        ctx.accounts.mint(amount)
    }

//vender
   pub fn burn(ctx: Context<BurnOIL>, amount: u64) -> Result<()> {
        ctx.accounts.burn(amount)
    }
      
//retiro
 pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw()
    }

//informar precio
    pub fn price(ctx: Context<UpdatePrice>, price: u64, exponent: u64, fee:u64) -> Result<()> {
        ctx.accounts.price(&ctx.bumps, price, exponent, fee)
    } 
   
   /* 
   // crear request the venta de OIL
   */

  /* 
   // aceptar/rechar como admin request the venta de OIL
   */

  /* 
   // rechazar siendo el creador request the venta de OIL
   */
}
