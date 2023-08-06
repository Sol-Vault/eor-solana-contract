use anchor_lang::prelude::*;

#[account]
pub struct HoldingWalletState {
    pub payee: Pubkey,
    pub meteora_allocation: u8,
    pub holding_allocation: u8,
    pub bump: u8,
    pub wallet_bump: u8,
}

impl HoldingWalletState {
    // Calculation of size: 32 + 1 + 1 + 1 + 1 = 36
    pub const SIZE: usize = 36;
}