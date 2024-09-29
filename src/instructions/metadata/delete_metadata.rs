use crate::{constants::SEED_PREFIX, helpers::validate_authority, state::MetadataAccount};
use anchor_lang::prelude::*;
use solana_program::pubkey::MAX_SEED_LEN;

#[derive(Accounts)]
#[instruction(_id: [u8; MAX_SEED_LEN])]
pub struct DeleteMetadataAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Not the actual "payer" for this ix, this is needed just for the seed validation
    #[account()]
    pub payer: AccountInfo<'info>,

    #[account(
        mut,
        close = authority,
        seeds = [SEED_PREFIX, payer.key().as_ref(), _id.as_ref()],
        bump
    )]
    pub metadata_account: Box<Account<'info, MetadataAccount>>,
}

impl DeleteMetadataAccount<'_> {
    pub fn validate(&self, ctx: &Context<Self>) -> Result<()> {
        validate_authority(&ctx.accounts.authority)?;
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn delete_metadata_account(ctx: Context<DeleteMetadataAccount>) -> Result<()> {
        Ok(())
    }
}
