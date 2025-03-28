use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::state::Vault;

#[derive(Accounts)]
pub struct MerchantSend<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"vault"],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        // associated_token::mint = token_mint,
        // associated_token::authority = vault,
        // associated_token::token_program = token_program,
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = receiver,
        associated_token::mint = token_mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program,
    )]
    pub receiver_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[event]
pub struct MerchantSendSuccess {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
}

pub fn merchant_send(ctx: Context<MerchantSend>, amount: u64) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[b"vault", &[ctx.accounts.vault.bump]]];

    let decimals = ctx.accounts.token_mint.decimals;

    let cpi_accounts = TransferChecked {
        mint: ctx.accounts.token_mint.to_account_info(),
        from: ctx.accounts.vault_ata.to_account_info(),
        to: ctx.accounts.receiver_ata.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);
    transfer_checked(cpi_context, amount, decimals)?;

    emit!(MerchantSendSuccess {
        sender: ctx.accounts.vault_ata.key(),
        receiver: ctx.accounts.receiver_ata.key(),
        token_mint: ctx.accounts.token_mint.key(),
        amount
    });
    Ok(())
}

// TODO:
// remove check in vault_ata
// payer = receiver
