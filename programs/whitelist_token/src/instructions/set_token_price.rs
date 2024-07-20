use anchor_lang::prelude::*;
use crate::state::SaleAccount;

#[derive(Accounts)]
pub struct SetTokenPrice<'info> {
    #[account(
        mut,
        seeds = [b"sale".as_ref(), authority.key().as_ref()],
        bump = sale_account.bump
    )]
    pub sale_account: Account<'info, SaleAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<SetTokenPrice>, new_price: u64) -> Result<()> {
    let sale_account = &mut ctx.accounts.sale_account;
    sale_account.token_price = new_price;
    Ok(())
}
