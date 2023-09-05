use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::state::{HyperflowStreamAggregate, HyperflowStream};


pub fn setup_stream_aggregate (
    ctx: Context<SetupStreamAggregate>,
    payee: Pubkey,
    per_second_rate: u64,
) -> Result<()> {
    let aggregate_bump = *ctx.bumps.get("stream_aggregate").unwrap();
    let stream_bump = *ctx.bumps.get("stream").unwrap();

    // For stream aggregate
    let aggregate = &mut ctx.accounts.aggregate;
    aggregate.payee = payee;
    aggregate.bump = aggregate_bump;

    // For inidvidual streams
    let stream = &mut ctx.accounts.stream;
    stream.mint = *ctx.accounts.mint.to_account_info().key;
    stream.decimals = ctx.accounts.mint.decimals;
    stream.per_second_rate = per_second_rate;
    stream.last_withdrawn = ctx.accounts.clock.unix_timestamp;
    stream.payer = *ctx.accounts.payer.to_account_info().key;
    stream.bump = stream_bump;

    let mut stream_vec: Vec<Pubkey> = Vec::new();
    stream_vec.push(stream.key());
    aggregate.streams = stream_vec;

    Ok(())
}

#[derive(Accounts)]
pub struct SetupStreamAggregate<'info> {
    #[account(
        init, 
        payer = payer, 
        space = 8 + 1 * HyperflowStreamAggregate::SIZE,
        seeds = [b"stream-aggregate", payee.key().as_ref()],
        bump
    )]
    pub aggregate: Account<'info, HyperflowStreamAggregate>,
    #[account(
        init, 
        payer = payer,
        space = 8 + 1 * HyperflowStream::SIZE,
        seeds = [b"stream", aggregate.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub stream: Account<'info, HyperflowStream>,
    pub mint: Account<'info, Mint>,
        /// CHECK: This is not dangerous because we don't read or write from this account
    pub payee: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn add_stream (
    ctx: Context<AddStream>,
) -> Result<()> {
    let aggregate = &mut ctx.accounts.aggregate;
    let mut streams = aggregate.streams.clone();
    streams.push(ctx.accounts.stream.key());
    aggregate.streams = streams;
    Ok(())
}

#[derive(Accounts)]
pub struct AddStream<'info> {
    #[account(
        mut,
        seeds=[b"stream-aggregate", aggregate.payee.key().as_ref()],
        realloc=32,
        realloc::payer=payer,
        realloc::zero=false,
        bump,
    )]
    pub aggregate: Account<'info, HyperflowStreamAggregate>,
    #[account(
        init,
        payer = payer,
        space = 8 + 1 * HyperflowStream::SIZE,
        seeds = [b"stream", aggregate.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub stream: Account<'info, HyperflowStream>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub payee: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}