use crate::state::HoldingWallet;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use anchor_lang::solana_program::system_instruction;
use anchor_lang::system_program;

pub fn setup_holding_wallet(
    ctx: Context<SetupHoldingWallet>,
    _organisation_id: String,
) -> Result<()> {
    let holding_wallet_bump = ctx.bumps.get("holding_wallet").unwrap();
    let holding_state_bump = ctx.bumps.get("holding_state_account").unwrap();

    let holding_wallet_account = &mut ctx.accounts.holding_wallet_account;
    holding_wallet_account.payee = *ctx.accounts.employee.key;
    holding_wallet_account.wallet_bump = *holding_wallet_bump;
    holding_wallet_account.state_bump = *holding_state_bump;

    Ok(())
}

#[derive(Accounts)]
#[instruction(_organisation_id: String)]
pub struct SetupHoldingWallet<'info> {
    #[account(
        seeds = [b"holding_wallet", employee.key().as_ref()],
        bump
    )]
    pub holding_wallet: AccountInfo<'info>,
    #[account(
        init ,
        payer = employee,
        space = 8 + HoldingWallet::SIZE,
        seeds = [b"holding_state", employee.key().as_ref(), _organisation_id.as_bytes().as_ref()], bump
    )]
    pub holding_wallet_account: Account<'info, HoldingWallet>,
    #[account(mut)]
    pub employee: Signer<'info>,
    pub system_program: Program<'info, System>,
}
