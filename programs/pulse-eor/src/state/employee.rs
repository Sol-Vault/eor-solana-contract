use anchor_lang::prelude::*;

#[account]
pub struct EmployeeContract {
    pub payee: Pubkey,
    pub bump: u8,
    pub rate: u64,
    pub frequency: String,
    pub last_payment: u64,
    pub stream_active: bool,
}

impl EmployeeContract {
    // Calculation of size: 32 + 1 + 8 + 1 + 8 + 1 + 1 = 52
    // Explanation of size: 32 (payee) + 1 (bump) + 8 (rate) + 1 (frequency) + 8 (last_payment) + 1 (stream_active) + 1 (padding)
    pub const SIZE: usize = 52;
}