use crate::{
    constants::SEED_PREFIX, errors::ErrorCode, helpers::validate_authority,
    state::SolTreasuryAccount,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct UpdateSolTreasuryAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_PREFIX],
        bump
    )]
    pub sol_treasury_account: Account<'info, SolTreasuryAccount>,
}

impl UpdateSolTreasuryAccount<'_> {
    pub fn validate(&self, ctx: &Context<Self>) -> Result<()> {
        validate_authority(&ctx.accounts.authority)?;
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn update_sol_treasury_account(
        ctx: Context<UpdateSolTreasuryAccount>,
        discount: u8,
    ) -> Result<()> {
        // Update only if needed
        let sol_treasury_account = &mut ctx.accounts.sol_treasury_account;
        if sol_treasury_account.discount == discount {
            return Err(error!(ErrorCode::ThisIsPointlessDude));
        }
        sol_treasury_account.discount = SolTreasuryAccount::update_discount(discount)?;
        Ok(())
    }
}
