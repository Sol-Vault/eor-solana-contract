use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Transfer, self};

use crate::state::{EmployeeContract, Organisation};
use crate::error::NovaError;

pub fn pay_contract(
    ctx: Context<PayContract>,
    _organisation_id: String,
    _employee_id: String,
    amount: u64,
) -> Result<()> {
    let balance = ctx.accounts.streaming_wallet_token_account.amount;
    if balance < amount {
        return err!(NovaError::NotEnoughBalanceError)
    }

    if ctx.accounts.organisation.stream_authority != *ctx.accounts.payer.to_account_info().key {
        return err!(NovaError::PayerIsNotStreamAuthority)
    }

    if ctx.accounts.employee_contract.payee != *ctx.accounts.employee_token_account.to_account_info().key {
        return err!(NovaError::NotEmployeeTokenAccount)
    }

    let cpi_accounts = Transfer {
        from: ctx.accounts.streaming_wallet_token_account.to_account_info(),
        to: ctx.accounts.employee_token_account.to_account_info(),
        authority: ctx.accounts.streaming_wallet.to_account_info(),
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

pub fn withdraw_from_stream_wallet(
    ctx: Context<WithdrawFromStreamWallet>,
    _organisation_id: String,
    amount: u64,
) -> Result<()> {
    let balance = ctx.accounts.streaming_wallet_token_account.amount;
    if balance < amount {
        panic!("Not enough balance")
    }

    let payer_key = ctx.accounts.payer.to_account_info().key;
    let admins = ctx.accounts.organisation.admins.clone();
    if !admins.contains(&payer_key) {
        panic!("Payer is not an admin")
    }

    let cpi_accounts = Transfer {
        from: ctx.accounts.streaming_wallet_token_account.to_account_info(),
        to: ctx.accounts.withdrawee_token_account.to_account_info(),
        authority: ctx.accounts.streaming_wallet.to_account_info(),
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
#[instruction(_organisation_id: String, employee_id: String)]
pub struct PayContract<'info> {
    #[account(
        mut,
        seeds=[b"employee-contract", _organisation_id.as_bytes(), employee_id.as_bytes()],
        bump = employee_contract.bump,
    )]
    pub employee_contract: Account<'info, EmployeeContract>,
    #[account(
        seeds = [b"organisation", _organisation_id.as_bytes().as_ref()], 
        bump=organisation.bump,
    )]
    pub organisation: Account<'info, Organisation>,
    #[account(
        mut,
        seeds=[b"streaming-wallet", _organisation_id.as_bytes()],
        bump = organisation.stream_wallet_bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
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

#[derive(Accounts)]
#[instruction(_organisation_id: String)]
pub struct WithdrawFromStreamWallet<'info> {
    #[account(
        seeds = [b"organisation", _organisation_id.as_bytes().as_ref()], 
        bump=organisation.bump,
    )]
    pub organisation: Account<'info, Organisation>,
    #[account(
        mut,
        seeds=[b"streaming-wallet", _organisation_id.as_bytes()],
        bump = organisation.stream_wallet_bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub streaming_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub streaming_wallet_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub withdrawee_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>, 
}