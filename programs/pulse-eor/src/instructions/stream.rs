use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Transfer, self};

use crate::state::{EmployeeContract, Organisation};

pub fn pay_contract(
    ctx: Context<PayContract>,
    _organisation_id: String,
    amount: u64,
) -> Result<()> {
    let balance = ctx.accounts.streaming_wallet_token_account.amount;
    if balance < amount {
        panic!("Not enough balance")
    }

    let cpi_accounts = Transfer {
        from: ctx.accounts.streaming_wallet_token_account.to_account_info(),
        to: ctx.accounts.streaming_wallet_token_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();

    let signer_seeds = &[
        b"streaming-wallet",
        _organisation_id.as_bytes(),
        &[ctx.accounts.organisation.stream_wallet_bump],
    ];
    let signer = &[&signer_seeds[..]];

    token::transfer(
        CpiContext::new(cpi_program, cpi_accounts).with_signer(
            signer
        ),
        amount,
    )?;

    Ok(())
}


#[derive(Accounts)]
#[instruction(_organisation_id: String)]
pub struct PayContract<'info> {
    pub employee_contract: Account<'info, EmployeeContract>,
    pub organisation: Account<'info, Organisation>,
    #[account(
        mut,
        seeds=[b"streaming-wallet", _organisation_id.as_bytes()],
        bump = organisation.stream_wallet_bump,
    )]
    pub streaming_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub streaming_wallet_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub employee_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>, 
}