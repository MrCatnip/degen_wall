use crate::{
    constants::SEED_PREFIX,
    errors::ErrorCode,
    events::MetadataAccountCreated,
    helpers::{calculate_sol_fee, validate_params, validate_treasury_account},
    state::{CreateMetadataAccountParams, MetadataAccount, PoolAccount},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(params: CreateMetadataAccountParams)]
pub struct CreateMetadataMintAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = MetadataAccount::LEN,
        seeds = [SEED_PREFIX, authority.key().as_ref(), params.id.as_ref()],
        bump
    )]
    pub metadata_account: Box<Account<'info, MetadataAccount>>,

    #[account()]
    pub mint: Box<Account<'info, Mint>>,

    #[account()]
    pub token: Box<Account<'info, Mint>>,

    #[account(
        seeds = [SEED_PREFIX, mint.key().as_ref()],
        bump
    )]
    pub pool_account: Box<Account<'info, PoolAccount>>,

    #[account(mut)]
    pub treasury_mint: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer_token_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub vault_wsol: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub vault_mint: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

impl CreateMetadataMintAccount<'_> {
    pub fn validate(
        &self,
        ctx: &Context<Self>,
        params: &CreateMetadataAccountParams,
    ) -> Result<()> {
        let pool_account = &ctx.accounts.pool_account;
        let mint = &ctx.accounts.mint.key();
        validate_treasury_account(&ctx.accounts.treasury_mint.key(), &pool_account.treasury)?;
        if mint != &pool_account.mint {
            return Err(error!(ErrorCode::InvalidMintAccount));
        }
        if &ctx.accounts.vault_wsol.key() != &pool_account.vault_wsol {
            return Err(error!(ErrorCode::InvalidWSOLAccount));
        }
        if &ctx.accounts.vault_mint.key() != &pool_account.vault_mint {
            return Err(error!(ErrorCode::InvalidMintAccount));
        }
        if mint != &ctx.accounts.payer_token_account.mint {
            return Err(error!(ErrorCode::InvalidPayerTokenAccount));
        }
        validate_params(params)?;
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn create_metadata_account_mint(
        ctx: Context<CreateMetadataMintAccount>,
        params: CreateMetadataAccountParams,
    ) -> Result<()> {
        // Check balance
        let payer_token_account = &ctx.accounts.payer_token_account;
        let vault_wsol_amount = ctx.accounts.vault_wsol.amount;
        let vault_mint_amount = ctx.accounts.vault_mint.amount;
        let pool_account = &ctx.accounts.pool_account;
        let discount = pool_account.discount;
        let data = &params.data;
        let required_fee: u64 = calculate_sol_fee(data, discount);
        let constant_product = vault_wsol_amount as u128 * vault_mint_amount as u128;
        let wsol_after_sell = vault_wsol_amount - required_fee;
        let required_fee_mint: u64 =
            (constant_product / (wsol_after_sell as u128)) as u64 - vault_mint_amount;
        if payer_token_account.amount < required_fee_mint {
            return Err(error!(ErrorCode::StopBeingPoor));
        }
        // Burn!
        let authority = &mut ctx.accounts.authority;
        let mint = &ctx.accounts.mint;
        let token_program = &ctx.accounts.token_program;
        if pool_account.burn {
            let cpi_accounts = Burn {
                mint: mint.to_account_info(),
                from: payer_token_account.to_account_info(),
                authority: authority.to_account_info(),
            };
            let cpi_program = token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            burn(cpi_ctx, required_fee_mint)?;
        }
        // Or Pay!
        else {
            let cpi_accounts = Transfer {
                from: payer_token_account.to_account_info(),
                to: ctx.accounts.treasury_mint.to_account_info(),
                authority: authority.to_account_info(),
            };
            let cpi_program = token_program.to_account_info();
            transfer(
                CpiContext::new(cpi_program, cpi_accounts),
                required_fee_mint,
            )?;
        }
        // Save
        let metadata_account = &mut *ctx.accounts.metadata_account;
        **metadata_account = MetadataAccount::new(
            ctx.bumps.metadata_account,
            mint.key(),
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
