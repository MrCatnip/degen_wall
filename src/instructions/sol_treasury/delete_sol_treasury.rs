use crate::{constants::SEED_PREFIX, helpers::validate_authority, state::SolTreasuryAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct DeleteSolTreasuryAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        close = authority,
        seeds = [SEED_PREFIX],
        bump
    )]
    pub sol_treasury_account: Account<'info, SolTreasuryAccount>,
}

impl DeleteSolTreasuryAccount<'_> {
    pub fn validate(&self, ctx: &Context<Self>) -> Result<()> {
        validate_authority(&ctx.accounts.authority)?;
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn delete_sol_treasury_account(ctx: Context<DeleteSolTreasuryAccount>) -> Result<()> {
        Ok(())
    }
}
