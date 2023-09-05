use anchor_lang::prelude::*;

#[account]
pub struct HypeflowTokenBalance {
    pub bump: u8,
    pub delegate_bump: u8,
    // Init with one, then extend as needed
    pub stream_aggreagtes: Vec<Pubkey>,
}

impl HypeflowTokenBalance {
    pub const SIZE: usize = 2 + 1 * 32 + 1 * 32;
}