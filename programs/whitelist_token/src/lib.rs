use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;

use instructions::*;
use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod whitelist_token {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        token_price: u64,
        purchase_limit_per_wallet: u64,
        total_supply: u64,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, token_price, purchase_limit_per_wallet, total_supply)
    }

    pub fn whitelist_user(
        ctx: Context<WhitelistUser>,
        user: Pubkey,
    ) -> Result<()> {
        instructions::whitelist_user::handler(ctx, user)
    }

    pub fn purchase_tokens(
        ctx: Context<PurchaseTokens>,
        amount: u64,
    ) -> Result<()> {
        instructions::purchase_tokens::handler(ctx, amount)
    }
}