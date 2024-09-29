use crate::{constants::SEED_PREFIX, helpers::validate_authority, state::SolTreasuryAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct CreateSolTreasuryAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = SolTreasuryAccount::LEN,
        seeds = [SEED_PREFIX],
        bump
    )]
    pub sol_treasury_account: Account<'info, SolTreasuryAccount>,

    pub system_program: Program<'info, System>,
}

impl CreateSolTreasuryAccount<'_> {
    pub fn validate(&self, ctx: &Context<Self>) -> Result<()> {
        validate_authority(&ctx.accounts.authority)?;
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn create_sol_treasury_account(
        ctx: Context<CreateSolTreasuryAccount>,
        discount: u8,
    ) -> Result<()> {
        // Save
        let sol_treasury_account = &mut ctx.accounts.sol_treasury_account;
        **sol_treasury_account = SolTreasuryAccount::new(ctx.bumps.sol_treasury_account, discount)?;
        Ok(())
    }
}
