use crate::state::HoldingWalletState;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, self};
use mercurial_vault::{cpi::accounts::DepositWithdrawLiquidity, program::Vault};

pub fn setup_holding_wallet(
    ctx: Context<SetupHoldingWallet>,
    _organisation_id: String,
) -> Result<()> {
    print!("Setting up holding wallet");
    let holding_wallet_bump = *ctx.bumps.get("holding_wallet").unwrap();
    let holding_state_bump = *ctx.bumps.get("holding_wallet_state").unwrap();

    let holding_wallet_account = &mut ctx.accounts.holding_wallet_state;
    holding_wallet_account.payee = *ctx.accounts.employee.key;
    holding_wallet_account.wallet_bump = holding_wallet_bump;
    holding_wallet_account.bump = holding_state_bump;

    holding_wallet_account.meteora_allocation = 40;
    holding_wallet_account.holding_allocation = 60;

    Ok(())
}

pub fn employee_withdraw(
    ctx: Context<EmployeeWithdraw>,
    _organisation_id: String,
    amount: u64,
    virtual_price: f64,
) -> Result<()> {
    let total_usdc = ctx.accounts.holding_wallet_token_account.amount + (virtual_price * ctx.accounts.holding_wallet_lp_token_account.amount as f64) as u64;
    if amount > total_usdc {
        panic!("Not enough balance")
    }
    let holding_allocation = ctx.accounts.holding_wallet_state.holding_allocation;

    let holding_amount_to_withdraw = amount * holding_allocation as u64 / 100;
    let meteora_amount_to_withdraw = amount - holding_amount_to_withdraw;
    let num_lp_mint_to_withdraw = (meteora_amount_to_withdraw as f64 / virtual_price) as u64;

    let cpi_accounts = Transfer {
        from: ctx.accounts.holding_wallet_token_account.to_account_info(),
        to: ctx.accounts.withdrawer_token_account.to_account_info(),
        authority: ctx.accounts.holding_wallet.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();

    let cpi_mecurial_accounts = DepositWithdrawLiquidity {
        vault: ctx.accounts.vault.to_account_info(),
        token_vault: ctx.accounts.token_vault.to_account_info(),
        lp_mint: ctx.accounts.lp_mint.to_account_info(),
        user_token: ctx.accounts.holding_wallet_token_account.to_account_info(),
        user_lp: ctx.accounts.holding_wallet_lp_token_account.to_account_info(),
        user: ctx.accounts.holding_wallet.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };
    let cpi_mercurial_program = ctx.accounts.mercurial_program.to_account_info();

    let signer_seeds = &[
        b"holding-wallet".as_ref(),
        ctx.accounts.withdrawer.key.as_ref(),
        &[ctx.accounts.holding_wallet_state.wallet_bump],
    ];
    let signer: &[&[&[u8]]; 1] = &[&signer_seeds[..]];

    let cpi_mercurial_withdraw_context = CpiContext::new(cpi_mercurial_program, cpi_mecurial_accounts).with_signer(signer);
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer);

    mercurial_vault::cpi::withdraw(cpi_mercurial_withdraw_context, num_lp_mint_to_withdraw, 0)?;
    token::transfer(cpi_context, amount)?;
    Ok(())
}

pub fn adjust_meteora_allocation(
    ctx: Context<AdjustMeteoraAllocation>,
    _organisation_id: String,
    adjusted_meteora_allocation: u8,
    amount_to_withdraw_from_mercurial: u64,
    amount_to_deposit_to_mercurial: u64,
) -> Result<()> {
    let holding_wallet_state = &mut ctx.accounts.holding_wallet_state;
    holding_wallet_state.meteora_allocation = adjusted_meteora_allocation.clone();
    holding_wallet_state.holding_allocation = 100 - adjusted_meteora_allocation.clone();

    let signer_seeds = &[
        b"holding-wallet".as_ref(),
        ctx.accounts.employee.key.as_ref(),
        &[ctx.accounts.holding_wallet_state.wallet_bump],
    ];
    let signer: &[&[&[u8]]; 1] = &[&signer_seeds[..]];

    let mecurial_accounts = DepositWithdrawLiquidity {
        vault: ctx.accounts.vault.to_account_info(),
        token_vault: ctx.accounts.token_vault.to_account_info(),
        lp_mint: ctx.accounts.lp_mint.to_account_info(),
        user_token: ctx.accounts.holding_wallet_token_account.to_account_info(),
        user_lp: ctx.accounts.holding_wallet_lp_token_account.to_account_info(),
        user: ctx.accounts.holding_wallet.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };

    let mecurial_context = CpiContext::new(
        ctx.accounts.mercurial_program.to_account_info(),
        mecurial_accounts,
    ).with_signer(signer);

    if amount_to_withdraw_from_mercurial > 0 && amount_to_deposit_to_mercurial == 0 {
        mercurial_vault::cpi::withdraw(
            mecurial_context,
            amount_to_withdraw_from_mercurial,
            0,
        )?;
    } else {
        mercurial_vault::cpi::deposit(
            mecurial_context,
            amount_to_deposit_to_mercurial,
            0,
        )?;
    }

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
    pub holding_wallet_state: Account<'info, HoldingWalletState>,
    #[account(mut)]
    pub employee: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_organisation_id: String, _amount: u64)]
pub struct EmployeeWithdraw<'info> {
    #[account(
        mut,
        seeds = [b"holding-state", withdrawer.key().as_ref(), _organisation_id.as_bytes().as_ref()],
        bump = holding_wallet_state.bump,
    )]
    pub holding_wallet_state: Account<'info, HoldingWalletState>,
    #[account(
        mut,
        seeds = [b"holding-wallet", withdrawer.key().as_ref()],
        bump=holding_wallet_state.wallet_bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub holding_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub holding_wallet_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub withdrawer: Signer<'info>,
    #[account(mut)]
    pub withdrawer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub holding_wallet_lp_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        has_one = token_vault,
        has_one = lp_mint,
    )]
    pub vault: Box<Account<'info, mercurial_vault::state::Vault>>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    pub token_mint: Account<'info, Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub mercurial_program: Program<'info, Vault>,
}

#[derive(Accounts)]
#[instruction(_organisation_id: String)]
pub struct AdjustMeteoraAllocation<'info> {
    #[account(
        mut,
        seeds = [b"holding-state", employee.key().as_ref(), _organisation_id.as_bytes().as_ref()],
        bump = holding_wallet_state.bump,
    )]
    pub holding_wallet_state: Account<'info, HoldingWalletState>,
    #[account(
        mut,
        seeds = [b"holding-wallet", employee.key().as_ref()],
        bump=holding_wallet_state.wallet_bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub holding_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub employee: Signer<'info>,
    #[account(mut)]
    pub holding_wallet_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub holding_wallet_lp_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        has_one = token_vault,
        has_one = lp_mint,
    )]
    pub vault: Box<Account<'info, mercurial_vault::state::Vault>>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    pub token_mint: Account<'info, Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
    pub mercurial_program: Program<'info, Vault>,
}