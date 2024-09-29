use crate::helpers::validate_discount;
use anchor_lang::prelude::*;

#[account]
pub struct PoolAccount {
    pub bump: u8,           // PDA bump
    pub mint: Pubkey,       // means of payment, SO111..2 for SOL
    pub vault_wsol: Pubkey, // DEX WSOL vault for calculating token price
    pub vault_mint: Pubkey, // DEX token vault for calculating token price
    pub treasury: Pubkey,   // token account to send the payment to
    pub discount: u8,       // 0-100 in %
    pub burn: bool,         // whether tokens should be burned instead of sent to treasury
}

impl PoolAccount {
    pub const LEN: usize = 8 // anchor discriminator 
    + 1 // bump 
    + 32 // mint 
    + 32 // vault_wsol
    + 32  // vault_mint
    + 32 // treasury
    + 1 // discount
    + 1; // burn

    /// instantiate the pool account with provided args
    pub fn new(
        bump: u8,
        mint: Pubkey,
        vault_wsol: Pubkey,
        vault_mint: Pubkey,
        treasury: Pubkey,
        discount: u8,
        burn: bool,
    ) -> Result<Self> {
        validate_discount(discount)?;
        Ok(Self {
            bump,
            mint,
            vault_wsol,
            vault_mint,
            treasury,
            discount,
            burn,
        })
    }

    // update pool discount
    pub fn update_discount(discount: u8) -> Result<u8> {
        validate_discount(discount)?;
        Ok(discount)
    }

    // update pool burn
    pub fn update_burn(burn: bool) -> bool {
        burn
    }
}
