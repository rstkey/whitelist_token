use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The user is not included in the whitelist")]
    NotOnWhitelist,
    #[msg("The purchase amount surpasses the limit for the wallet")]
    WalletLimitExceeded,
    #[msg("There are not enough tokens available in the supply")]
    TokenSupplyShortage,
}