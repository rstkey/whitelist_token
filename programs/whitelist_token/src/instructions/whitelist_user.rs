use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct WhitelistUser<'info> {
    #[account(
        mut,
        seeds = [b"sale".as_ref(), authority.key().as_ref()],
        bump = sale_account.bump
    )]
    pub sale_account: Account<'info, SaleAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [b"buyer", sale_account.key().as_ref(), user.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<BuyerInfo>(),
    )]
    pub buyer_info: Account<'info, BuyerInfo>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WhitelistUser>, user: Pubkey) -> Result<()> {
    let sale_account = &mut ctx.accounts.sale_account;
    let buyer_info = &mut ctx.accounts.buyer_info;

    buyer_info.buyer = user;
    buyer_info.amount = 0;
    buyer_info.whitelisted = true;

    sale_account.whitelisted_users.push(buyer_info.buyer.key());
    Ok(())
}