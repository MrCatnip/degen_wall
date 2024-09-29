use crate::{constants::*, errors::ErrorCode, state::CreateMetadataAccountParams};
use anchor_lang::prelude::*;

pub fn get_current_epoch_and_timestamp() -> (u64, i64) {
    let clock = Clock::get().unwrap();
    (clock.epoch, clock.unix_timestamp)
}

pub fn calculate_sol_fee(data: &[u8; MAX_DATA_SIZE as usize], discount: u8) -> u64 {
    let px_nr = data.len() / PX_SIZE as usize;
    px_nr as u64 * LAMPORTS_PER_PIXEL * (100 - discount) as u64 / 100
}

pub fn calculate_px_nr(data: &[u8; MAX_DATA_SIZE as usize]) -> u8 {
    let mut px_nr: u8 = 0;
    for chunk in data.chunks_exact(PX_SIZE as usize) {
        if chunk[0] == DATA_DELIMITER {
            return px_nr;
        }
        px_nr += 1;
    }
    px_nr
}

pub fn merge_strings(args: &[&str]) -> String {
    args.join(STRING_DELIMITER)
}

pub fn validate_data(data: &[u8; MAX_DATA_SIZE as usize]) -> Result<()> {
    let length = data.len();
    if length == 0 {
        return Err(error!(ErrorCode::EmptyData));
    }
    if length > MAX_DATA_SIZE as usize {
        return Err(error!(ErrorCode::DataTooBig));
    }
    if length % PX_SIZE as usize != 0 {
        return Err(error!(ErrorCode::InvalidData));
    }
    let mut is_px_nr_nonzero = false;
    for chunk in data.chunks_exact(PX_SIZE as usize) {
        if chunk[0] > PX_WIDTH || chunk[1] > PX_HEIGHT {
            if chunk[0] == DATA_DELIMITER && is_px_nr_nonzero {
                return Ok(());
            }
            return Err(error!(ErrorCode::InvalidData));
        }
        is_px_nr_nonzero |= true;
    }
    Ok(())
}

pub fn validate_url(url: &str) -> Result<()> {
    if url.starts_with("https://") || url.starts_with("http://") {
        return Err(error!(ErrorCode::NoHttpPrefix));
    }
    Ok(())
}

pub fn validate_twitter(twitter: &str) -> Result<()> {
    let length = twitter.len();
    if length < 1 {
        return Ok(());
    }
    if length > TWITTER_LENGTH as usize {
        return Err(error!(ErrorCode::TwitterStringTooBig));
    }
    let mut prev_char = '\0';
    for (i, ch) in twitter.chars().enumerate() {
        if !ch.is_alphanumeric() && ch != '_' {
            return Err(error!(ErrorCode::InvalidTwitter));
        }
        if (i == 0 || i == length - 1) && ch == '_' {
            return Err(error!(ErrorCode::InvalidTwitter));
        }
        if ch == '_' && prev_char == '_' {
            return Err(error!(ErrorCode::InvalidTwitter));
        }
        prev_char = ch;
    }
    Ok(())
}

pub fn validate_name(name: &str) -> Result<()> {
    let length = name.len();
    if length > NAME_LENGTH as usize {
        return Err(error!(ErrorCode::NameStringTooBig));
    }
    Ok(())
}

pub fn validate_ticker(ticker: &str) -> Result<()> {
    let length = ticker.len();
    if length > TICKER_LENGTH as usize {
        return Err(error!(ErrorCode::TickerStringTooBig));
    }
    Ok(())
}

pub fn validate_strl_len(args: [&String; SOCIALS_COUNT as usize]) -> Result<()> {
    let length: usize = args.iter().map(|s| s.len()).sum();
    if length > MAX_SOCIALS_SIZE as usize {
        return Err(error!(ErrorCode::OverallStringTooBig));
    }
    Ok(())
}

pub fn validate_params(params: &CreateMetadataAccountParams) -> Result<()> {
    let CreateMetadataAccountParams {
        id: _id,
        data,
        website,
        twitter,
        community,
        image,
        name,
        ticker,
        description,
    } = params;
    validate_twitter(twitter)?;
    validate_url(website)?;
    validate_url(community)?;
    validate_url(image)?;
    validate_name(name)?;
    validate_ticker(ticker)?;
    validate_data(data)?;
    validate_strl_len([
        website,
        twitter,
        community,
        image,
        name,
        ticker,
        description,
    ])?;
    Ok(())
}

pub fn validate_discount(discount: u8) -> Result<()> {
    if discount > 99 {
        return Err(error!(ErrorCode::InvalidDiscountValue));
    }
    Ok(())
}

pub fn validate_authority(authority: &Signer<'_>) -> Result<()> {
    if authority.key() != AUTHORITY_PUBKEY {
        return Err(error!(ErrorCode::YouAreNotMyBoss));
    }
    Ok(())
}

pub fn validate_treasury_account(
    treasury_public_key: &Pubkey,
    expected_treasury_public_key: &Pubkey,
) -> Result<()> {
    if treasury_public_key != expected_treasury_public_key {
        return Err(error!(ErrorCode::WhyAreYouDoingThisToMeBruv));
    }
    Ok(())
}
