use anchor_lang::prelude::*;
use crate::state::organisation::Organisation;

pub fn setup_organisation(
    ctx: Context<SetupOrganisation>,
    id: String,
) -> Result<()> {
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct SetupOrganisation<'info> {
    #[account(init, payer = admin, space = Organisation::SIZE + 8, seeds = [id.as_bytes().as_ref()], bump)]
    pub organisation: Account<'info, Organisation>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}