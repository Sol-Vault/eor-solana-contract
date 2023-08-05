use anchor_lang::prelude::*;

declare_id!("DafQCS2iwRB48xvjYa1Nsxz5wFDuMXq37qCmy4Sov8ce");

#[program]
pub mod pulse_eor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
