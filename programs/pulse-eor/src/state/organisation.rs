use anchor_lang::prelude::*;

#[account]
pub struct Organisation {
    pub admins: Vec<Pubkey>,
    pub bump: u8,
    pub organisation_streaming_authority_bump: u8,
    pub stream_authority: Pubkey,
}

impl Organisation {
    // Calculation of size: 32 * 10 + 1 + 1 + 32 = 325
    pub const SIZE: usize  = 326;
}