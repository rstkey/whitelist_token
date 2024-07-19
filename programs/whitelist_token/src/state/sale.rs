use anchor_lang::prelude::*;

#[account]
pub struct SaleAccount {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub vault: Pubkey,
    pub token_price: u64,
    pub purchase_limit_per_wallet: u64,
    pub bump: u8,
    pub total_supply: u64,
    pub sold_tokens: u64,
    pub whitelisted_users: Vec<Pubkey>,
    pub buyers: Vec<Pubkey>,
}