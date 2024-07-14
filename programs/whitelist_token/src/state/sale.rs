use anchor_lang::prelude::*;

#[account]
pub struct Sale {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub price: u64,
    pub max_per_wallet: u64,
    pub total_supply: u64,
    pub total_sold: u64,
}