use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::state::SaleAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        seeds = [b"sale".as_ref(), authority.key().as_ref()],
        bump,
        payer = authority, 
        space = 8 + std::mem::size_of::<SaleAccount>() + 5120,
    )]
    pub sale_account: Account<'info, SaleAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = sale_account,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<Initialize>,
    token_price: u64,
    purchase_limit_per_wallet: u64,
    total_supply: u64,
) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    let (_, bump) = Pubkey::find_program_address(&[b"sale".as_ref(), authority.as_ref()], ctx.program_id);

    let sale_account = &mut ctx.accounts.sale_account;
    sale_account.bump = bump;
    sale_account.authority = ctx.accounts.authority.key();
    sale_account.token_mint = ctx.accounts.token_mint.key();
    sale_account.vault = ctx.accounts.vault.key();
    sale_account.token_price = token_price;
    sale_account.purchase_limit_per_wallet = purchase_limit_per_wallet;
    sale_account.total_supply = total_supply;
    sale_account.sold_tokens = 0;

    Ok(())
}