use anchor_lang::prelude::*;

#[account]
pub struct BuyerInfo {
    pub buyer: Pubkey,
    pub amount: u64,
    pub whitelisted: bool
}