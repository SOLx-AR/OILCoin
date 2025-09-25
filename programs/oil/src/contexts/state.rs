#[account]
#[derive(InitSpace)]
pub struct Price {
    pub price: u64,
    pub exponent: u64,
    pub fee: u64,
    pub bump: u8,
}