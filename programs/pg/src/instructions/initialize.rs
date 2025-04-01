use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::state::Vault;

#[event_cpi]
#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer=signer,
        space=8+Vault::INIT_SPACE,
        seeds=[b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct InitSuccess {
    pub vault: Pubkey,
}

pub fn init_vault(ctx: Context<InitVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.owner = *ctx.accounts.signer.key;
    vault.bump = ctx.bumps.vault;
    emit_cpi!(InitSuccess { vault: vault.key() });
    Ok(())
}
