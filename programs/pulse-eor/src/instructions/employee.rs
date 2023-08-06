use crate::state::HoldingWalletState;
use anchor_lang::prelude::*;

pub fn setup_holding_wallet(
    ctx: Context<SetupHoldingWallet>,
    _organisation_id: String,
) -> Result<()> {
    print!("Setting up holding wallet");
    let holding_wallet_bump = *ctx.bumps.get("holding_wallet").unwrap();
    let holding_state_bump = *ctx.bumps.get("holding_wallet_account").unwrap();

    let holding_wallet_account = &mut ctx.accounts.holding_wallet_account;
    holding_wallet_account.payee = *ctx.accounts.employee.key;
    holding_wallet_account.wallet_bump = holding_wallet_bump;
    holding_wallet_account.bump = holding_state_bump;

    holding_wallet_account.meteora_allocation = 40;
    holding_wallet_account.holding_allocation = 60;


    Ok(())
}

#[derive(Accounts)]
#[instruction(_organisation_id: String)]
pub struct SetupHoldingWallet<'info> {
    #[account(
        seeds = [b"holding-wallet", employee.key().as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub holding_wallet: AccountInfo<'info>,
    #[account(
        init ,
        payer = employee,
        space = 8 + HoldingWalletState::SIZE,
        seeds = [b"holding-state", employee.key().as_ref(), _organisation_id.as_bytes().as_ref()],
        bump
    )]
    pub holding_wallet_account: Account<'info, HoldingWalletState>,
    #[account(mut)]
    pub employee: Signer<'info>,
    pub system_program: Program<'info, System>,
}
