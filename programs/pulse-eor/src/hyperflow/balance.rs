use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Approve, self};

use crate::state::HypeflowTokenBalance;

pub fn set_up_token_balance(
    ctx: Context<SetTokenBalance>,
    amount: u64,
) -> Result<()> {
    let token_delegate_bump = *ctx.bumps.get("token_delegate").unwrap();
    let token_balance_state_bump = *ctx.bumps.get("token_balance_state").unwrap();
    let aggregates:  Vec<Pubkey> = Vec::new();

    let token_balance_state = &mut ctx.accounts.token_balance_state;
    token_balance_state.bump = token_balance_state_bump;
    token_balance_state.delegate_bump = token_delegate_bump;
    token_balance_state.stream_aggreagtes = aggregates;

    let cpi_accounts = Approve {
        to: ctx.accounts.payer_token_account.to_account_info(),
        delegate: ctx.accounts.token_delegate.to_account_info(),
        authority: ctx.accounts.payer.to_account_info()
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::approve(cpi_ctx, amount)?;

    Ok(())
}

pub fn reset_delegation(
    ctx: Context<ResetDelegation>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = Approve {
        to: ctx.accounts.payer_token_account.to_account_info(),
        delegate: ctx.accounts.token_delegate.to_account_info(),
        authority: ctx.accounts.payer.to_account_info()
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::approve(cpi_ctx, amount)?;
    Ok(())
}


#[derive(Accounts)]
pub struct SetTokenBalance<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub payer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        seeds=[b"token-delegate", payer_token_account.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub token_delegate: AccountInfo<'info>,
    #[account(
        init,
        payer=payer,
        space=HypeflowTokenBalance::SIZE + 8,
        seeds=[b"token-balance-state", payer_token_account.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub token_balance_state: Box<Account<'info, HypeflowTokenBalance>>,
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResetDelegation<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub payer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        seeds=[b"token-delegate", payer_token_account.key().as_ref(), mint.key().as_ref()],
        bump=token_balance_state.delegate_bump
    )]
    pub token_delegate: AccountInfo<'info>,
    #[account(
        seeds=[b"token-balance-state", payer_token_account.key().as_ref(), mint.key().as_ref()],
        bump=token_balance_state.bump
    )]
    pub token_balance_state: Account<'info, HypeflowTokenBalance>,
    pub token_program: AccountInfo<'info>,
}