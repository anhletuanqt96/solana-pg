use anchor_lang::prelude::*;

use crate::state::{Owner, Whitelist};

/**
 * InitOwner instruction
 */
#[derive(Accounts)]
pub struct InitOwner<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer=signer,
        space=8+Owner::INIT_SPACE,
        seeds=[b"owner"],
        bump
    )]
    pub owner: Account<'info, Owner>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct InitOwnerSuccess {
    pub owner: Pubkey,
}

pub fn init_owner(ctx: Context<InitOwner>, new_owner: Pubkey) -> Result<()> {
    let owner = &mut ctx.accounts.owner;
    owner.addr = new_owner;
    emit!(InitOwnerSuccess { owner: new_owner });

    Ok(())
}

/**
 * TransferOwner instruction
 */

#[derive(Accounts)]
pub struct TransferOwner<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        constraint = owner.addr == signer.key(),
    )]
    pub owner: Account<'info, Owner>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct TransferOwnerSuccess {
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
}

pub fn transfer_owner(ctx: Context<TransferOwner>, new_owner: Pubkey) -> Result<()> {
    let owner = &mut ctx.accounts.owner;
    let old_owner = owner.addr;
    owner.addr = new_owner;
    emit!(TransferOwnerSuccess {
        old_owner,
        new_owner
    });

    Ok(())
}

/**
 * Add whitelist instruction
 */
#[derive(Accounts)]
#[instruction(addr: Pubkey)]
pub struct AddWhitelist<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        constraint = owner.addr == signer.key(),
    )]
    pub owner: Account<'info, Owner>,
    #[account(
        init,
        payer=signer,
        space=8+Whitelist::INIT_SPACE,
        seeds=[b"whitelist",addr.as_ref()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}
#[event]
pub struct AddWhitelistSuccess {
    pub addr: Pubkey,
}

pub fn add_whitelist(ctx: Context<AddWhitelist>, addr: Pubkey) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.addr = addr;
    whitelist.bump = ctx.bumps.whitelist;
    emit!(AddWhitelistSuccess { addr });

    Ok(())
}

/**
 * Remove whitelist instruction
 */
#[derive(Accounts)]
pub struct RemoveWhitelist<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        constraint = owner.addr == signer.key(),
    )]
    pub owner: Account<'info, Owner>,
    #[account(
        mut,
        close = signer,
        seeds=[b"whitelist",whitelist.addr.as_ref()],
        bump=whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}
#[event]
pub struct RemoveWhitelistSuccess {
    pub addr: Pubkey,
}

pub fn remove_whitelist(ctx: Context<RemoveWhitelist>) -> Result<()> {
    let whitelist = &ctx.accounts.whitelist;
    emit!(AddWhitelistSuccess {
        addr: whitelist.addr
    });

    Ok(())
}
