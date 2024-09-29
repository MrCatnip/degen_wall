use anchor_lang::{prelude::*, solana_program::pubkey};

#[constant]
pub const SEED_PREFIX: &[u8] = b"degen_wall";
#[constant]
pub const LAMPORTS_PER_PIXEL: u64 = 1_000_000; // 0.001 SOL
#[constant]
pub const WSOL_PUBKEY: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
#[constant]
pub const AUTHORITY_PUBKEY: Pubkey = pubkey!("H3v4uZwVuoCHDyTFezH196wUHxmm7NfBH2yxzUB6MpDZ");
#[constant]
pub const TREASURY_PUBKEY: Pubkey = pubkey!("AWJQAWxPE3hJz2XVrJDmBDdQk4pC2SjeKpLFhjUncCKM");
#[constant]
pub const VERSION: u8 = 2;
#[constant]
pub const PX_WIDTH: u8 = 100;
#[constant]
pub const PX_HEIGHT: u8 = 50;
#[constant]
pub const MAX_PX_NR: u8 = 100;
#[constant]
pub const PX_SIZE: u8 = 5;
#[constant]
pub const MAX_DATA_SIZE: u16 = MAX_PX_NR as u16 * PX_SIZE as u16;
#[constant]
pub const TWITTER_LENGTH: u8 = 15;
#[constant]
pub const NAME_LENGTH: u8 = 30;
#[constant]
pub const TICKER_LENGTH: u8 = 10;
#[constant]
pub const MAX_SOCIALS_SIZE: u8 = 195;
#[constant]
pub const DATA_DELIMITER: u8 = 255;
#[constant]
pub const STRING_DELIMITER: &str = "|";

pub const SOCIALS_COUNT: u8 = 7;
