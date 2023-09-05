use anchor_lang::prelude::*;
use instructions::*;
use hyperflow::*;

declare_id!("DafQCS2iwRB48xvjYa1Nsxz5wFDuMXq37qCmy4Sov8ce");

pub mod instructions;
pub mod state;
pub mod error;
pub mod hyperflow;

#[program]
pub mod pulse_eor {
    use super::*;

    pub fn setup_holding_wallet(
        ctx: Context<SetupHoldingWallet>,
        _organisation_id: String,
    ) -> Result<()> {
        print!("Setting up holding wallet");
        instructions::employee::setup_holding_wallet(ctx, _organisation_id)
    }

    pub fn pay_organisation_employee(
        ctx: Context<PayOrganisationEmployee>,
        _organisation_id: String,
        _employee_id: String,
        _amount: u64,
    ) -> Result<()> {
        print!("Paying organisation employee");
        instructions::organisation::pay_organisation_employee(ctx, _organisation_id, _amount)
    }

    pub fn employee_withdraw(
        ctx: Context<EmployeeWithdraw>,
        _organisation_id: String,
        amount: u64,
        virtual_price: f64,
    ) -> Result<()> {
        print!("Employee withdrawing");
        instructions::employee::employee_withdraw(ctx, _organisation_id, amount, virtual_price)
    }

    pub fn adjust_meteora_allocation(
        ctx: Context<AdjustMeteoraAllocation>,
        _organisation_id: String,
        meteora_allocation: u8,
        amount_to_withdraw_from_mercurial: u64,
        amount_to_deposit_to_mercurial: u64,
    ) -> Result<()> {
        print!("Adjusting meteora allocation");
        instructions::employee::adjust_meteora_allocation(ctx, _organisation_id, meteora_allocation, amount_to_withdraw_from_mercurial, amount_to_deposit_to_mercurial)
    }
    
    pub fn setup_organisation(
        ctx: Context<SetupOrganisation>,
        _organisation_id: String,
    ) -> Result<()> {
        print!("Setting up organisation");
        instructions::organisation::setup_organisation(ctx, _organisation_id)
    }
    
    pub fn setup_employee_contract(
        ctx: Context<SetupEmployeeContract>,
        _organisation_id: String,
        _employee_id: String,
        rate: u64,
    ) -> Result<()> {
        print!("Setting up employee contract");
        instructions::organisation::setup_employee_contract(ctx, _organisation_id, _employee_id, rate)
    }

    pub fn pay_contract(
        ctx: Context<PayContract>,
        _organisation_id: String,
        _employee_id: String,
        amount: u64,
    ) -> Result<()> {
        print!("Paying contract");
        instructions::stream::pay_contract(ctx, _organisation_id, _employee_id, amount)
    }

    pub fn withdraw_from_stream_wallet(
        ctx: Context<WithdrawFromStreamWallet>,
        _organisation_id: String,
        amount: u64,
    ) -> Result<()> {
        print!("Withdrawing from stream wallet");
        instructions::stream::withdraw_from_stream_wallet(ctx, _organisation_id, amount)
    }

    pub fn set_up_token_balance(
        ctx: Context<SetTokenBalance>,
        amount: u64,
    ) -> Result<()> {
        print!("Setting up token balance");
        hyperflow::balance::set_up_token_balance(ctx, amount)
    }

    pub fn reset_delegation(
        ctx: Context<ResetDelegation>,
        amount: u64,
    ) -> Result<()> {
        print!("Resetting delegation");
        hyperflow::balance::reset_delegation(ctx, amount)
    }

    pub fn setup_stream_aggregate (
        ctx: Context<SetupStreamAggregate>,
        payee: Pubkey,
        per_second_rate: u64,
    ) -> Result<()> {
        print!("Setting up stream aggregate");
        hyperflow::stream::setup_stream_aggregate(ctx, payee, per_second_rate)
    }

    pub fn add_stream (
        ctx: Context<AddStream>,
    ) -> Result<()> {
        print!("Adding stream");
        hyperflow::stream::add_stream(ctx)
    }

    pub fn withdraw_from_stream (
        ctx: Context<WithdrawFromStream>,
        amount: u64,
    ) -> Result<()> {
        print!("Withdrawing from stream");
        hyperflow::transact::withdraw_from_stream(ctx, amount)
    }

    
}
