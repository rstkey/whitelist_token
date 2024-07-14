use anchor_lang::prelude::*;
use crate::state::{Sale, WhitelistEntry};

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(mut)]
    pub sale: Account<'info, Sale>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32,
        seeds = [b"whitelist", sale.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
    let whitelist_entry = &mut ctx.accounts.whitelist_entry;
    whitelist_entry.user = user;
    Ok(())
}