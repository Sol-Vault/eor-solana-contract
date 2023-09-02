use anchor_lang::prelude::*;

#[account]
pub struct EmployeeContract {
    pub payee: Pubkey,
    pub bump: u8,
    pub rate: u64,
}

impl EmployeeContract {
    // Calculation of size: 32 + 1 + 8 = 41
    pub const SIZE: usize = 73;
}