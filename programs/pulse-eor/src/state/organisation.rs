use anchor_lang::prelude::*;

#[account]
pub struct Organisation {
    id: String,
}

impl Organisation {
    // Calculation of size: 24 + 8
    pub const SIZE: usize = 32;
}