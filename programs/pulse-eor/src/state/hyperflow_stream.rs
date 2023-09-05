use anchor_lang::prelude::*;

#[account]
pub struct HyperflowStream {
    pub mint: Pubkey,
    pub decimals: u8,
    // per second rate
    pub per_second_rate: u64,
    pub last_withdrawn: i64,
    pub payer: Pubkey,
    pub bump: u8,
    pub delegate: Pubkey,
}

impl HyperflowStream {
    pub const SIZE: usize = 41;
}

#[account]
pub struct HyperflowStreamAggregate {
    pub payee: Pubkey,
    pub streams: Vec<Pubkey>,
    pub bump: u8,
}

impl HyperflowStreamAggregate {
    pub const SIZE: usize = 33 + 1 * HyperflowStream::SIZE;
}