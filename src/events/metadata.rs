use crate::constants::MAX_DATA_SIZE;
use anchor_lang::prelude::*;
use solana_program::pubkey::MAX_SEED_LEN;

#[event]
pub struct MetadataAccountCreated {
    pub id: [u8; MAX_SEED_LEN],
    pub mint: Pubkey,
    pub timestamp: i64,
    pub payer: Pubkey,
    pub token: Pubkey,
    pub data: [u8; MAX_DATA_SIZE as usize],
    pub website: String,
    pub twitter: String,
    pub community: String,
    pub image: String,
    pub name: String,
    pub ticker: String,
    pub description: String,
}
