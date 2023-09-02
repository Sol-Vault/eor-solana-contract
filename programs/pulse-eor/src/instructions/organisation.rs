use crate::state::{organisation::Organisation, HoldingWalletState, EmployeeContract};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use mercurial_vault::{cpi::accounts::DepositWithdrawLiquidity, program::Vault};

// pub fn setup_organisation(ctx: Context<SetupOrganisation>) -> Result<()> {
//     Ok(())
// }

pub fn pay_organisation_employee(
    ctx: Context<PayOrganisationEmployee>,
    _organisation_id: String,
    amount: u64,
) -> Result<()> {
    let balance = ctx.accounts.payer_token_account.amount;
    if balance < amount {
        panic!("Not enough balance")
    }

    let meteora_allocation_percentage =
        ctx.accounts.holding_wallet_state.clone().meteora_allocation as u64;
    let meteora_allocation = amount * meteora_allocation_percentage / 100;
    let holding_allocation = amount - meteora_allocation;

    // print!("Paying organisation employee {}", holding_allocation);
    let mercurial_accounts = DepositWithdrawLiquidity {
        vault: ctx.accounts.vault.to_account_info(),
        token_vault: ctx.accounts.token_vault.to_account_info(),
        lp_mint: ctx.accounts.lp_mint.to_account_info(),
        user_token: ctx.accounts.payer_token_account.to_account_info(),
        user_lp: ctx
            .accounts
            .holding_wallet_lp_token_account
            .to_account_info(),
        user: ctx.accounts.payer.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };
    print!("After mercurial");

    let mer_context = CpiContext::new(
        ctx.accounts.mercurial_program.to_account_info(),
        mercurial_accounts,
    );
    let cpi_accounts = Transfer {
        from: ctx.accounts.payer_token_account.to_account_info(),
        to: ctx.accounts.holding_wallet_token_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();

    print!("BEFORE mercurial");
    mercurial_vault::cpi::deposit(mer_context, meteora_allocation, 0)?;
    print!("AFTER mercurial");

    print!("BEFORE token transfer");
    token::transfer(
        CpiContext::new(cpi_program, cpi_accounts),
        holding_allocation,
    )?;
    print!("AFTER token transfer");

    Ok(())
}

pub fn setup_organisation(ctx: Context<SetupOrganisation>, _organisation_id: String) -> Result<()> {
    let organisation = &mut ctx.accounts.organisation;
    let bump = *ctx.bumps.get("organisation").unwrap();
    let streaming_wallet_bump = *ctx.bumps.get("streaming-wallet").unwrap();
    
    let mut admins:  Vec<Pubkey> = Vec::new();
    admins.push(ctx.accounts.admin.key());
    organisation.admins = admins;
    organisation.bump = bump;
    organisation.stream_wallet_bump = streaming_wallet_bump;

    Ok(())
}

pub fn setup_employee_contract(
    ctx: Context<SetupEmployeeContract>,
    _organisation_id: String,
    _employee_id: String,
    rate: u64,
) -> Result<()> {
    let bump = *ctx.bumps.get("employee_contract").unwrap();
    let employee_contract = &mut ctx.accounts.employee_contract;
    employee_contract.payee = ctx.accounts.payee.key();
    employee_contract.bump = bump;
    employee_contract.rate = rate;

    Ok(())
}

#[derive(Accounts)]
#[instruction(_organisation_id: String)]
pub struct SetupOrganisation<'info> {
    #[account(
        init, 
        payer = admin, 
        space = Organisation::SIZE + 8, 
        seeds = [b"organisation", _organisation_id.as_bytes().as_ref()], 
        bump
    )]
    pub organisation: Account<'info, Organisation>,
    #[account(
        seeds=[b"streaming-wallet", _organisation_id.as_bytes()],
        bump,
    )]
    pub streaming_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_organisation_id: String, _employee_id: String, _amount: u64)]
pub struct PayOrganisationEmployee<'info> {
    #[account(
        seeds = [b"holding-wallet", employee.key().as_ref()],
        bump=holding_wallet_state.wallet_bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub holding_wallet: AccountInfo<'info>,
    #[account(
        seeds = [b"holding-state", employee.key().as_ref(), _organisation_id.as_bytes().as_ref()],
        bump = holding_wallet_state.bump,
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
    #[account(mut)]
    pub payer_lp_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
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
    #[account(mut)]
    pub holding_wallet_lp_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub mercurial_program: Program<'info, Vault>,
}


#[derive(Accounts)]
#[instruction(_organisation_id: String, _employee_id: String)]
pub struct SetupEmployeeContract<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [b"employee-contract", organisation.key().as_ref(), _employee_id.as_bytes()],
        bump,
        space = EmployeeContract::SIZE,
    )]
    pub employee_contract: Box<Account<'info, EmployeeContract>>,
    #[account(
        seeds = [b"organisation", _organisation_id.as_bytes()],
        bump,
    )]
    pub organisation: Box<Account<'info, Organisation>>,
    pub payee: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: AccountInfo<'info>,
}