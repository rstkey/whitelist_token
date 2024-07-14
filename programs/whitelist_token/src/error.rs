use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("User is not whitelisted")]
    NotWhitelisted,
    #[msg("Purchase exceeds wallet limit")]
    ExceedsWalletLimit,
    #[msg("Insufficient token supply")]
    InsufficientSupply,
}