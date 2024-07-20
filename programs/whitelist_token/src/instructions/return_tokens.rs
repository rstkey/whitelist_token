use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::{SaleAccount, BuyerInfo};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct ReturnTokens<'info> {
    #[account(
        mut,
        seeds = [b"sale".as_ref(), sale_account.authority.key().as_ref()],
        bump = sale_account.bump
    )]
    pub sale_account: Account<'info, SaleAccount>,
    #[account(
        mut,
        seeds = [b"buyer".as_ref(), sale_account.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub buyer_info: Account<'info, BuyerInfo>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ReturnTokens>, amount: u64) -> Result<()> {
    let sale_account = &mut ctx.accounts.sale_account;
    let buyer_info = &mut ctx.accounts.buyer_info;

    // Ensure the user has enough tokens to return
    require!(
        buyer_info.amount >= amount,
        ErrorCode::InsufficientTokens
    );

    // Update the buyer info
    buyer_info.amount -= amount;

    // Perform the token transfer
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.token_vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}
