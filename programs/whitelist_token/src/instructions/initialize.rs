use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::state::Sale;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 8)]
    pub sale: Account<'info, Sale>,
    pub token_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = sale,
    )]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<Initialize>,
    price: u64,
    max_per_wallet: u64,
    total_supply: u64,
) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    sale.authority = ctx.accounts.authority.key();
    sale.token_mint = ctx.accounts.token_mint.key();
    sale.token_vault = ctx.accounts.token_vault.key();
    sale.price = price;
    sale.max_per_wallet = max_per_wallet;
    sale.total_supply = total_supply;
    sale.total_sold = 0;
    Ok(())
}