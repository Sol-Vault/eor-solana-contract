use anchor_lang::prelude::*;

#[error_code]
pub enum NovaError {
    NotEnoughBalanceError,
    PayerIsNotStreamAuthority,
    NotEmployeeTokenAccount,
}