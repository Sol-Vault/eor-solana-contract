use anchor_lang::prelude::*;
use instructions::*;

declare_id!("DafQCS2iwRB48xvjYa1Nsxz5wFDuMXq37qCmy4Sov8ce");

pub mod instructions;
pub mod state;

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
}
