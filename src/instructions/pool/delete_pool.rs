use crate::{constants::SEED_PREFIX, helpers::validate_authority, state::PoolAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction()]
pub struct DeletePoolAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        close = authority,
        seeds = [SEED_PREFIX, mint.key().as_ref()],
        bump
    )]
    pub pool_account: Account<'info, PoolAccount>,
}

impl DeletePoolAccount<'_> {
    pub fn validate(&self, ctx: &Context<Self>) -> Result<()> {
        validate_authority(&ctx.accounts.authority)?;
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn delete_pool_account(ctx: Context<DeletePoolAccount>) -> Result<()> {
        Ok(())
    }
}
