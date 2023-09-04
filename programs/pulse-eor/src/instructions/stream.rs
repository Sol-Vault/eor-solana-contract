use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Transfer, self, Mint};

use crate::state::{EmployeeContract, Organisation};

pub fn pay_contract(
    ctx: Context<PayContract>,
    _organisation_id: String,
    _employee_id: String,
) -> Result<()> {
    let now = ctx.accounts.clock.unix_timestamp as u64;
    let last_payment = ctx.accounts.employee_contract.last_payment;
    let time_between = now - last_payment;
    let rate = ctx.accounts.employee_contract.rate as f64; // Assuming rate can be converted to f64

    const SECONDS_IN_A_WEEK: f64 = 7.0 * 24.0 * 60.0 * 60.0;
    const SECONDS_IN_A_MONTH: f64 = 30.0 * 24.0 * 60.0 * 60.0;

    let per_second_rate: f64;

    match ctx.accounts.employee_contract.frequency.as_str() {
        "WEEKLY" => {
            per_second_rate = rate / SECONDS_IN_A_WEEK;
        },
        "MONTHLY" => {
            per_second_rate = rate / SECONDS_IN_A_MONTH;
        },
        _ => {
            panic!("Invalid frequency")
        }
    }

    let total_rate = (time_between as f64 * per_second_rate).round() as u64;
    let decimals = ctx.accounts.mint.decimals as u32;
    let amount_to_send = total_rate * 10u64.pow(decimals);

    let delegated_amount = ctx.accounts.treasury_token_account.delegated_amount;
    if delegated_amount < amount_to_send {
        panic!("Not enough delegated balance")
    }

    let cpi_accounts = Transfer {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        to: ctx.accounts.employee_token_account.to_account_info(),
        authority: ctx.accounts.organisation_streaming_authority.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let treasury_key = ctx.accounts.treasury.key();
    let signer_seeds = &[
        b"organisation-streaming-authority",
        _organisation_id.as_bytes(),
        treasury_key.as_ref(),
        &[ctx.accounts.organisation.organisation_streaming_authority_bump],
    ];

    let signer = &[&signer_seeds[..]];

    token::transfer(
        CpiContext::new(cpi_program, cpi_accounts).with_signer(
            signer
        ),
        amount_to_send,
    )?;

    Ok(())
}

pub fn realtime_employee_withdraw (
    ctx: Context<RealtimeEmployeeWithdraw>,
    _organisation_id: String,
    _employee_id: String,
) -> Result<()> {
    let now = ctx.accounts.clock.unix_timestamp as u64;
    let last_payment = ctx.accounts.employee_contract.last_payment;
    let time_between = now - last_payment;
    let rate = ctx.accounts.employee_contract.rate as f64; // Assuming rate can be converted to f64

    const SECONDS_IN_A_WEEK: f64 = 7.0 * 24.0 * 60.0 * 60.0;
    const SECONDS_IN_A_MONTH: f64 = 30.0 * 24.0 * 60.0 * 60.0;

    let mut per_second_rate: f64 = 0.0;

    match ctx.accounts.employee_contract.frequency.as_str() {
        "WEEKLY" => {
            per_second_rate = rate / SECONDS_IN_A_WEEK;
        },
        "MONTHLY" => {
            per_second_rate = rate / SECONDS_IN_A_MONTH;
        },
        _ => {
            panic!("Invalid frequency")
        }
    }

    let total_rate = (time_between as f64 * per_second_rate).round() as u64;
    let decimals = ctx.accounts.mint.decimals as u32;
    let amount_to_send = total_rate * 10u64.pow(decimals);

    let delegated_amount = ctx.accounts.treasury_token_account.delegated_amount;
    if delegated_amount < amount_to_send {
        panic!("Not enough delegated balance")
    }

    let cpi_accounts = Transfer {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        to: ctx.accounts.employee.to_account_info(),
        authority: ctx.accounts.organisation_streaming_authority.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let treasury_key = ctx.accounts.treasury.key();

    let signer_seeds = &[
        b"organisation-streaming-authority",
        _organisation_id.as_bytes(),
        treasury_key.as_ref(),
        &[ctx.accounts.organisation.organisation_streaming_authority_bump],
    ];

    let signer = &[&signer_seeds[..]];

    token::transfer(
        CpiContext::new(cpi_program, cpi_accounts).with_signer(
            signer
        ),
        amount_to_send,
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
        seeds = [b"organisation-streaming-authority", _organisation_id.as_bytes(), treasury.key().as_ref()],
        bump
    )]
    pub organisation_streaming_authority: AccountInfo<'info>,
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub employee_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint : Account<'info, Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>, 
    pub clock: Sysvar<'info, Clock>,
}

// #[derive(Accounts)]
// #[instruction(_organisation_id: String)]
// pub struct WithdrawFromStreamWallet<'info> {
//     #[account(
//         seeds = [b"organisation", _organisation_id.as_bytes().as_ref()], 
//         bump=organisation.bump,
//     )]
//     pub organisation: Account<'info, Organisation>,
//     #[account(
//         mut,
//         seeds=[b"streaming-wallet", _organisation_id.as_bytes()],
//         bump = organisation.stream_wallet_bump,
//     )]
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub streaming_wallet: AccountInfo<'info>,
//     #[account(mut)]
//     pub streaming_wallet_token_account: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub withdrawee_token_account: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub token_program: Program<'info, Token>, 
// }

#[derive(Accounts)]
#[instruction(_organisation_id: String, _employee_id: String)]
pub struct RealtimeEmployeeWithdraw<'info> {
    #[account(
        seeds = [b"employee-contract", _organisation_id.as_bytes(), _employee_id.as_bytes()],
        bump = employee_contract.bump,
    )]
    pub employee_contract: Box<Account<'info, EmployeeContract>>,
    #[account(
        seeds = [b"organisation", _organisation_id.as_bytes()],
        bump = organisation.bump,
    )]
    pub organisation: Box<Account<'info, Organisation>>,
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub employee: Signer<'info>,
    #[account(
        mut,
        seeds = [b"organisation-streaming-authority", _organisation_id.as_bytes(), treasury.key().as_ref()],
        bump
    )]
    pub organisation_streaming_authority: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}