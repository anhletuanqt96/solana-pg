use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub owner: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Owner {
    pub addr: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Whitelist {
    pub addr: Pubkey,
    pub bump: u8,
}
