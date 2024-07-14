use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::{Sale, WhitelistEntry, BuyerPurchaseAccount};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub sale: Account<'info, Sale>,
    #[account(
        seeds = [b"whitelist", sale.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
    #[account(mut)]
    pub token_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + 8,
        seeds = [b"purchase", sale.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub buyer_purchase_account: Account<'info, BuyerPurchaseAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    let whitelist_entry = &ctx.accounts.whitelist_entry;

    // Check if user is whitelisted
    require!(whitelist_entry.user == ctx.accounts.buyer.key(), ErrorCode::NotWhitelisted);

    // Check purchase limit
    let buyer_purchase_account = &mut ctx.accounts.buyer_purchase_account;
    require!(
        buyer_purchase_account.amount.checked_add(amount).unwrap() <= sale.max_per_wallet,
        ErrorCode::ExceedsWalletLimit
    );

    // Check total supply
    require!(
        sale.total_sold.checked_add(amount).unwrap() <= sale.total_supply,
        ErrorCode::InsufficientSupply
    );

    // Transfer tokens
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.token_vault.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: sale.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Update state
    sale.total_sold = sale.total_sold.checked_add(amount).unwrap();
    buyer_purchase_account.amount = buyer_purchase_account.amount.checked_add(amount).unwrap();

    Ok(())
}