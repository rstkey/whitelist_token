use anchor_lang::prelude::*;

#[account]
pub struct WhitelistEntry {
    pub user: Pubkey,
}