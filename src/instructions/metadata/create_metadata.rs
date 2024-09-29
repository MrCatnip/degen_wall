use crate::{
    constants::{SEED_PREFIX, WSOL_PUBKEY},
    errors::ErrorCode,
    events::MetadataAccountCreated,
    helpers::{calculate_sol_fee, validate_params, validate_treasury_account},
    state::{CreateMetadataAccountParams, MetadataAccount, SolTreasuryAccount},
};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction::transfer},
};
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction(params: CreateMetadataAccountParams)]
pub struct CreateMetadataAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = MetadataAccount::LEN,
        seeds = [SEED_PREFIX, authority.key().as_ref(), params.id.as_ref()],
        bump
    )]
    pub metadata_account: Account<'info, MetadataAccount>,

    #[account()]
    pub token: Box<Account<'info, Mint>>,

    #[account(
        seeds = [SEED_PREFIX],
        bump
    )]
    pub sol_treasury_account: Box<Account<'info, SolTreasuryAccount>>,

    #[account(mut)]
    /// CHECK: This is validated against the address stored in SolTreasury
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl CreateMetadataAccount<'_> {
    pub fn validate(
        &self,
        ctx: &Context<Self>,
        params: &CreateMetadataAccountParams,
    ) -> Result<()> {
        let sol_treasury_account = &ctx.accounts.sol_treasury_account;
        validate_treasury_account(&ctx.accounts.treasury.key(), &sol_treasury_account.treasury)?;
        validate_params(params)?;
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn create_metadata_account(
        ctx: Context<CreateMetadataAccount>,
        params: CreateMetadataAccountParams,
    ) -> Result<()> {
        // Check balance
        let authority: &mut Signer = &mut ctx.accounts.authority;
        let discount = ctx.accounts.sol_treasury_account.discount;
        let data = &params.data;
        let required_fee: u64 = calculate_sol_fee(data, discount);
        if authority.lamports() < required_fee {
            return Err(error!(ErrorCode::StopBeingPoor));
        }
        // Transfer
        let treasury = &mut ctx.accounts.treasury;
        let ix = transfer(&authority.key(), &treasury.key(), required_fee);
        match invoke(
            &ix,
            &[authority.to_account_info(), treasury.to_account_info()],
        ) {
            Ok(()) => (),
            Err(_) => return Err(error!(ErrorCode::NotEnoughMoneyForGas)),
        };
        // Save
        let metadata_account = &mut ctx.accounts.metadata_account;
        **metadata_account = MetadataAccount::new(
            ctx.bumps.metadata_account,
            WSOL_PUBKEY,
            authority.key(),
            ctx.accounts.token.key(),
            &params,
        )?;
        // Emit event
        emit!(MetadataAccountCreated {
            id: metadata_account.id,
            mint: metadata_account.mint,
            timestamp: metadata_account.timestamp,
            payer: metadata_account.payer,
            token: metadata_account.token,
            data: metadata_account.data,
            website: params.website.clone(),
            twitter: params.twitter.clone(),
            community: params.community.clone(),
            image: params.image.clone(),
            name: params.name.clone(),
            ticker: params.ticker.clone(),
            description: params.description.clone(),
        });
        Ok(())
    }
}
