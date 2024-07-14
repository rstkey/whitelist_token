use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;

use instructions::*;
use state::*;

declare_id!("8CfNbqPDAVDWZhTWEKg475aRzERSkXhRd4R37CHBShhQ");

#[program]
pub mod whitelist_token_sale {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        price: u64,
        max_per_wallet: u64,
        total_supply: u64,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, price, max_per_wallet, total_supply)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
        instructions::add_to_whitelist::handler(ctx, user)
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        instructions::buy_tokens::handler(ctx, amount)
    }
}