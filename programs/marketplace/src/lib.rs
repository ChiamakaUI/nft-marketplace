
use anchor_lang::prelude::*;

mod state;
mod contexts;

use crate::contexts::*;

declare_id!("4zgbQJ13t5EJ66qt2LpSdXowP4TWzPQZKNkSC8YvEp7k");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, ctx.bumps)?;
        Ok(())
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_listing()?;
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.pay()?; 
        ctx.accounts.transfer_nft()?;
        ctx.accounts.close_vault_account()?;
        ctx.accounts.reward_buyer()?;
        Ok(())
    }
}
