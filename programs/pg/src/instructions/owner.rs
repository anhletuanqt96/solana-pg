use anchor_lang::prelude::*;

use crate::state::Owner;

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

#[derive(Accounts)]
pub struct TransferOwner<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        constraint = owner.owner == signer.key(),
    )]
    pub owner: Account<'info, Owner>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct InitOwnerSuccess {
    pub owner: Pubkey,
}

#[event]
pub struct TransferOwnerSuccess {
    pub old_owner: Pubkey,
    pub new_owner: Pubkey,
}

pub fn init_owner(ctx: Context<InitOwner>, new_owner: Pubkey) -> Result<()> {
    let owner = &mut ctx.accounts.owner;
    owner.owner = new_owner;
    emit!(InitOwnerSuccess { owner: new_owner });

    Ok(())
}

pub fn transfer_owner(ctx: Context<TransferOwner>, new_owner: Pubkey) -> Result<()> {
    let owner = &mut ctx.accounts.owner;
    let old_owner = owner.owner;
    owner.owner = new_owner;
    emit!(TransferOwnerSuccess {
        old_owner,
        new_owner
    });

    Ok(())
}
