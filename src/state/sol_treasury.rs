use crate::{constants::TREASURY_PUBKEY, helpers::validate_discount};
use anchor_lang::prelude::*;

#[account]
pub struct SolTreasuryAccount {
    pub bump: u8,         // PDA bump
    pub treasury: Pubkey, // address to send the payment to
    pub discount: u8,     // 0-100 in %
}

impl SolTreasuryAccount {
    pub const LEN: usize = 8 // anchor discriminator 
    + 1 // bump 
    + 32 // treasury 
    + 1; // discount

    /// instantiate the sol treasury account with provided args
    pub fn new(bump: u8, discount: u8) -> Result<Self> {
        validate_discount(discount)?;
        Ok(Self {
            bump,
            treasury: TREASURY_PUBKEY,
            discount,
        })
    }

    // update sol treasury discount
    pub fn update_discount(discount: u8) -> Result<u8> {
        validate_discount(discount)?;
        Ok(discount)
    }
}
