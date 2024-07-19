use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::{SaleAccount, BuyerInfo};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct PurchaseTokens<'info> {
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

pub fn handler(ctx: Context<PurchaseTokens>, amount: u64) -> Result<()> {
    {
        let sale_account = &mut ctx.accounts.sale_account;
        let buyer_info = &mut ctx.accounts.buyer_info;

        // Check if the user is whitelisted
        require!(
            sale_account.whitelisted_users.contains(&buyer_info.buyer),
            ErrorCode::NotOnWhitelist
        );

        // Check if the purchase amount exceeds the limit per wallet
        require!(
            buyer_info.amount + amount <= sale_account.purchase_limit_per_wallet,
            ErrorCode::WalletLimitExceeded
        );

        // Check if there are enough tokens available in the supply
        require!(
            sale_account.sold_tokens + amount <= sale_account.total_supply,
            ErrorCode::TokenSupplyShortage
        );

        // Update the buyer info and sale account
        buyer_info.amount += amount;
        sale_account.sold_tokens += amount;
    }

    let sale_account = &ctx.accounts.sale_account;
    // Perform the token transfer
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.token_vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.sale_account.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let seeds = &[b"sale".as_ref(), sale_account.authority.as_ref(), &[sale_account.bump]];
    let signer = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, amount)?;

    {
        let sale_account = &mut ctx.accounts.sale_account;
        let buyer_info = &mut ctx.accounts.buyer_info;
        
        sale_account.buyers.push(buyer_info.buyer.key());
    }

    Ok(())
}
