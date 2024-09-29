use crate::{
    constants::{SEED_PREFIX, WSOL_PUBKEY, TREASURY_PUBKEY},
    errors::ErrorCode,
    helpers::validate_authority,
    state::PoolAccount,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
#[instruction()]
pub struct CreatePoolAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account()]
    pub vault_wsol: Account<'info, TokenAccount>,

    #[account()]
    pub vault_mint: Account<'info, TokenAccount>,

    #[account()]
    pub treasury: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        space = PoolAccount::LEN,
        seeds = [SEED_PREFIX, mint.key().as_ref()],
        bump
    )]
    pub pool_account: Account<'info, PoolAccount>,

    pub system_program: Program<'info, System>,
}

impl CreatePoolAccount<'_> {
    pub fn validate(&self, ctx: &Context<Self>) -> Result<()> {
        let mint = &ctx.accounts.mint.key();
        validate_authority(&ctx.accounts.authority)?;
        if &ctx.accounts.vault_wsol.mint != &WSOL_PUBKEY {
            return Err(error!(ErrorCode::InvalidWSOLAccount));
        }
        if &ctx.accounts.vault_mint.mint != mint {
            return Err(error!(ErrorCode::InvalidMintAccount));
        }
        if &ctx.accounts.treasury.mint != mint {
            return Err(error!(ErrorCode::InvalidTreasuryAccount));
        }
        if &ctx.accounts.treasury.owner != &TREASURY_PUBKEY {
            return Err(error!(ErrorCode::InvalidTreasuryOwner));
        }
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn create_pool_account(
        ctx: Context<CreatePoolAccount>,
        discount: u8,
        burn: bool,
    ) -> Result<()> {
        // Save
        let pool_account = &mut ctx.accounts.pool_account;
        **pool_account = PoolAccount::new(
            ctx.bumps.pool_account,
            ctx.accounts.mint.key(),
            ctx.accounts.vault_wsol.key(),
            ctx.accounts.vault_mint.key(),
            ctx.accounts.treasury.key(),
            discount,
            burn,
        )?;
        Ok(())
    }
}
