pub mod constants;
pub mod errors;
pub mod events;
pub mod helpers;
pub mod instructions;
pub mod state;

use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub use constants::*;
pub use errors::ErrorCode;
pub use helpers::{validate_authority, validate_discount};
pub use instructions::*;
use solana_program::pubkey::MAX_SEED_LEN;
pub use state::{
    metadata::{CreateMetadataAccountParams, MetadataAccount},
    pool::PoolAccount,
    sol_treasury::SolTreasuryAccount,
};

#[allow(unused_imports)]
use solana_security_txt::security_txt;

declare_id!("DEGenPMwjmLCw9LmdvfCUK5M4XKrbep2rts4DDqG3J5x");

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Degen Wall",
    project_url: "https://example.com",
    contacts: "cacat@gmail.com",
    policy: "-",
    preferred_languages: "en",
    source_code: "https://cacat.com"
}

#[program]
pub mod degen_wall {
    use super::*;

    pub fn create_metadata_account(
        ctx: Context<CreateMetadataAccount>,
        params: CreateMetadataAccountParams,
    ) -> Result<()> {
        CreateMetadataAccount::create_metadata_account(ctx, params)
    }

    pub fn create_metadata_account_mint(
        ctx: Context<CreateMetadataMintAccount>,
        params: CreateMetadataAccountParams,
    ) -> Result<()> {
        CreateMetadataMintAccount::create_metadata_account_mint(ctx, params)
    }

    pub fn create_pool_account(
        ctx: Context<CreatePoolAccount>,
        discount: u8,
        burn: bool,
    ) -> Result<()> {
        CreatePoolAccount::create_pool_account(ctx, discount, burn)
    }

    pub fn create_sol_treasury_account(
        ctx: Context<CreateSolTreasuryAccount>,
        discount: u8,
    ) -> Result<()> {
        CreateSolTreasuryAccount::create_sol_treasury_account(ctx, discount)
    }

    pub fn update_pool_account(
        ctx: Context<UpdatePoolAccount>,
        discount: u8,
        burn: bool,
    ) -> Result<()> {
        UpdatePoolAccount::update_pool_account(ctx, discount, burn)
    }

    pub fn update_sol_treasury_account(
        ctx: Context<UpdateSolTreasuryAccount>,
        discount: u8,
    ) -> Result<()> {
        UpdateSolTreasuryAccount::update_sol_treasury_account(ctx, discount)
    }

    pub fn delete_metadata_account(
        ctx: Context<DeleteMetadataAccount>,
        _id: [u8; MAX_SEED_LEN],
    ) -> Result<()> {
        DeleteMetadataAccount::delete_metadata_account(ctx)
    }

    pub fn delete_pool_account(ctx: Context<DeletePoolAccount>) -> Result<()> {
        DeletePoolAccount::delete_pool_account(ctx)
    }

    pub fn delete_sol_treasury_account(ctx: Context<DeleteSolTreasuryAccount>) -> Result<()> {
        DeleteSolTreasuryAccount::delete_sol_treasury_account(ctx)
    }
}
