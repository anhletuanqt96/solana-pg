#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BZjK9jZL4w7UjGSD1KmAqaB77SSDDfQ1nn4HkfMowUFv");

#[program]
pub mod pg {
    use super::*;

    pub fn init_vault(ctx: Context<InitVault>) -> Result<()> {
        initialize::init_vault(ctx)
    }

    /**
     * Owner
     */
    pub fn init_owner(ctx: Context<InitOwner>, new_owner: Pubkey) -> Result<()> {
        owner::init_owner(ctx, new_owner)
    }

    pub fn transfer_owner(ctx: Context<TransferOwner>, new_owner: Pubkey) -> Result<()> {
        owner::transfer_owner(ctx, new_owner)
    }
    /**
     * Whitelist
     */
    pub fn add_whitelist(ctx: Context<AddWhitelist>, addr: Pubkey) -> Result<()> {
        owner::add_whitelist(ctx, addr)
    }

    pub fn remove_whitelist(ctx: Context<RemoveWhitelist>) -> Result<()> {
        owner::remove_whitelist(ctx)
    }

    pub fn user_pay(ctx: Context<UserPay>, amount: u64) -> Result<()> {
        user_pay::user_pay(ctx, amount)
    }
}
