use anchor_lang::prelude::*;

#[account]
pub struct HoldingWallet {
    pub payee: Pubkey,
    pub wallet_bump: u8,
    pub state_bump: u8,
}

impl HoldingWallet {
    // Calculation of size: 8 + 8
    pub const SIZE: usize = 8;
}