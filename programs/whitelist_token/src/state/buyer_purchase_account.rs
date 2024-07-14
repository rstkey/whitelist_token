use anchor_lang::prelude::*;

#[account]
pub struct BuyerPurchaseAccount {
    pub amount: u64,
}