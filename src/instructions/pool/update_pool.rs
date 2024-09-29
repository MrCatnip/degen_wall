use crate::{
    constants::SEED_PREFIX, errors::ErrorCode, helpers::validate_authority, state::PoolAccount,
};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction()]
pub struct UpdatePoolAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [SEED_PREFIX, mint.key().as_ref()],
        bump
    )]
    pub pool_account: Account<'info, PoolAccount>,
}

impl UpdatePoolAccount<'_> {
    pub fn validate(&self, ctx: &Context<Self>) -> Result<()> {
        validate_authority(&ctx.accounts.authority)?;
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn update_pool_account(
        ctx: Context<UpdatePoolAccount>,
        discount: u8,
        burn: bool,
    ) -> Result<()> {
        // Update only if needed
        let pool_account = &mut ctx.accounts.pool_account;
        let mut is_update_needed = false;
        if pool_account.discount != discount {
            is_update_needed |= true;
            pool_account.discount = PoolAccount::update_discount(discount)?;
        }
        if pool_account.burn != burn {
            is_update_needed |= true;
            pool_account.burn = PoolAccount::update_burn(burn);
        }
        if !is_update_needed {
            return Err(error!(ErrorCode::ThisIsPointlessDude));
        }
        Ok(())
    }
}
