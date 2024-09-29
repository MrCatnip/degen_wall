use crate::{
    constants::{MAX_DATA_SIZE, MAX_SOCIALS_SIZE, SOCIALS_COUNT, VERSION},
    helpers::get_current_epoch_and_timestamp,
};
use anchor_lang::prelude::*;
use solana_program::pubkey::MAX_SEED_LEN;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMetadataAccountParams {
    pub id: [u8; MAX_SEED_LEN],
    pub data: [u8; MAX_DATA_SIZE as usize],
    pub website: String,
    pub twitter: String,
    pub community: String,
    pub image: String,
    pub name: String,
    pub ticker: String,
    pub description: String,
}

#[account]
pub struct MetadataAccount {
    pub bump: u8,               // PDA bump
    pub id: [u8; MAX_SEED_LEN], // seed needed for pda uniqueness
    pub version: u8,            // versioning, mainly intended for filtering out deprecated accounts
    pub epoch: u64,             // if caching is implemented this is our basis for "quick syncing"
    pub mint: Pubkey,           // means of payment - stat tracking feature
    pub timestamp: i64,         // sorting & timelapse feature
    pub payer: Pubkey,          // who paid - stat tracking feature
    // user input
    pub token: Pubkey,
    pub data: [u8; MAX_DATA_SIZE as usize], // pixel(s) layout,
    pub socials: String, // easier to store all strings into 1 than reserving 4 bytes of offset for every string
}

impl MetadataAccount {
    pub const LEN: usize = 8 // anchor discriminator 
    + 1 // bump
    + 32 // id
    + 1 // version
    + 8 // epoch
    + 32  // mint
    + 8 // timestamp
    + 32 // payer
    // user input
    + 32 // token
    + MAX_DATA_SIZE as usize // data -> max 100 pixels
    + 4 // string offset
    + SOCIALS_COUNT as usize - 1 // SOCIALS_COUNT - 1 -> nr of STRING_DELIMITER chars added as delimiters upon squashing them
    + MAX_SOCIALS_SIZE as usize; // more space if we squash all strings into 1

    /// instantiate the metadata account with provided args
    pub fn new(
        bump: u8,
        mint: Pubkey,
        payer: Pubkey,
        token: Pubkey,
        params: &CreateMetadataAccountParams,
    ) -> Result<Self> {
        let (epoch, timestamp) = get_current_epoch_and_timestamp();
        let CreateMetadataAccountParams {
            id,
            data,
            website,
            twitter,
            community,
            image,
            name,
            ticker,
            description,
        } = params;
        let mut data_fixed: [u8; MAX_DATA_SIZE as usize] = [255; MAX_DATA_SIZE as usize];
        let data_slice = &mut data_fixed[..data.len()];
        data_slice.copy_from_slice(data);
        let socials = format!(
            "{}|{}|{}|{}|{}|{}|{}",
            website, twitter, community, image, name, ticker, description
        );
        Ok(Self {
            bump,
            id: *id,
            version: VERSION,
            epoch,
            mint,
            timestamp,
            payer,
            token,
            data: data_fixed,
            socials,
        })
    }
}
