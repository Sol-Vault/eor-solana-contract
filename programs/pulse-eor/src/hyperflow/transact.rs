use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::state::{HypeflowTokenBalance, HyperflowStream, HyperflowStreamAggregate};

pub fn withdraw_from_stream(ctx: Context<WithdrawFromStream>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.payer_token_account.to_account_info(),
        to: ctx.accounts.withdrawer.to_account_info(),
        authority: ctx.accounts.delegate.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let token_account_key = ctx.accounts.payer_token_account.key();
    let mint_key = ctx.accounts.mint.key();

    let delegate_signer_seeds = &[
        b"token-delegate",
        token_account_key.as_ref(),
        mint_key.as_ref(),
        &[ctx.accounts.token_balance_state.delegate_bump],
    ];

    let signer = &[&delegate_signer_seeds[..]];

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer);

    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFromStream<'info> {
    #[account(
        mut,
        constraint = aggregate.payee == withdrawer.key(),
    )]
    pub withdrawer: Signer<'info>,
    #[account(
        mut,
        constraint=withdrawer_token_account.owner.key() == stream.payer.key(),
    )]
    pub withdrawer_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"stream-aggregate", withdrawer.key().as_ref()],
        bump=aggregate.bump
    )]
    pub aggregate: Account<'info, HyperflowStreamAggregate>,
    #[account(
        seeds = [b"stream", aggregate.key().as_ref(), mint.key().as_ref()],
        bump=stream.bump
    )]
    pub stream: Account<'info, HyperflowStream>,
    #[account(
        mut,
        constraint = payer_token_account.owner.key() == stream.payer.key(),
    )]
    pub payer_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds=[b"token-delegate", payer_token_account.key().as_ref(), mint.key().as_ref()],
        bump=token_balance_state.delegate_bump
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub delegate: AccountInfo<'info>,
    #[account(
        seeds=[b"token-balance-state", payer_token_account.key().as_ref(), mint.key().as_ref()],
        bump=token_balance_state.bump
    )]
    pub token_balance_state: Account<'info, HypeflowTokenBalance>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}
