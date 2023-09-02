use anchor_lang::prelude::*;

#[account]
pub struct Organisation {
    pub admins: Vec<Pubkey>,
    pub bump: u8,
    pub stream_wallet_bump: u8,
    pub stream_authority: Pubkey,
}

impl Organisation {
    pub const SIZE: usize = 1 + 1 + 32 * 10 + 32;
}