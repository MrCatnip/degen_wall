use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Stop being poor!")]
    StopBeingPoor,
    #[msg("Why are you doing this to me bruv?")]
    WhyAreYouDoingThisToMeBruv,
    #[msg("You are not my boss!")]
    YouAreNotMyBoss,
    #[msg("Not enough money for gas!")]
    NotEnoughMoneyForGas,
    #[msg("Invalid WSOL Account")]
    InvalidWSOLAccount,
    #[msg("Invalid Mint account!")]
    InvalidMintAccount,
    #[msg("Invalid Treasury Account!")]
    InvalidTreasuryAccount,
    #[msg("Invalid Treasury Owner!")]
    InvalidTreasuryOwner,
    #[msg("Invalid Payer Token Account!")]
    InvalidPayerTokenAccount,
    #[msg("Invalid Discount Value!")]
    InvalidDiscountValue,
    #[msg("This is pointless dude!")]
    ThisIsPointlessDude,
    #[msg("Empty data!")]
    EmptyData,
    #[msg("Data too big!")]
    DataTooBig,
    #[msg("Invalid data!")]
    InvalidData,
    #[msg("URL too big!")]
    URLTooBig,
    #[msg("Twitter string too big!")]
    TwitterStringTooBig,
    #[msg("Invalid Twitter!")]
    InvalidTwitter,
    #[msg("Name string too big!")]
    NameStringTooBig,
    #[msg("Ticker string too big!")]
    TickerStringTooBig,
    #[msg("No Http Prefix!")]
    NoHttpPrefix,
    #[msg("Overall string too big!")]
    OverallStringTooBig,
}
