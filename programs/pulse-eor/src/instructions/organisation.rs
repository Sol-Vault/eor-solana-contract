use crate::state::{organisation::Organisation, HoldingWalletState};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

pub fn setup_organisation(ctx: Context<SetupOrganisation>) -> Result<()> {
    Ok(())
}

pub fn pay_organisation_employee(
    ctx: Context<PayOrganisationEmployee>,
    _organisation_id: String,
    amount: u64,
) -> Result<()> {
    let token_mint = &ctx.accounts.token_mint;

    // let meteora_allocation_percentage = ctx.accounts.holding_wallet_state.clone().meteora_allocation as u64;
    let holding_allocation_percentage = ctx.accounts.holding_wallet_state.clone().holding_allocation as u64;
    // let meteora_allocation = meteora_allocation_percentage / 100;
    let holding_allocation = holding_allocation_percentage * amount / 100;

    // Transfer holding allocation to holding wallet
    // token::Mint
    // token::transfer_checked();
    // Transfer holding allocation to holding wallet
    token::transfer_checked(
        ctx.accounts.into_transfer_to_holding(),
        holding_allocation * 10u64.pow(token_mint.decimals.into()),
        ctx.accounts.token_mint.decimals,
    )?;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct SetupOrganisation<'info> {
    #[account(init, payer = admin, space = Organisation::SIZE + 8, seeds = [id.as_bytes().as_ref()], bump)]
    pub organisation: Account<'info, Organisation>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_organisation_id: String, _employee_id: String, _amount: u64)]
pub struct PayOrganisationEmployee<'info> {
    #[account(
        seeds = [b"holding_wallet", employee.key().as_ref()],
        bump
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub holding_wallet: AccountInfo<'info>,
    #[account(
        seeds = [b"holding_state", employee.key().as_ref(), _organisation_id.as_bytes().as_ref()],
        bump
    )]
    pub holding_wallet_state: Box<Account<'info, HoldingWalletState>>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub employee: AccountInfo<'info>,
    // Token Account stuff
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub holding_wallet_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub payer_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> PayOrganisationEmployee<'info> {
    fn into_transfer_to_holding(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.payer_token_account.to_account_info(),
            to: self.holding_wallet_token_account.to_account_info(),
            authority: self.payer.to_account_info(),
            mint: self.token_mint.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}