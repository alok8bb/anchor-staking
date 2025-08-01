#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

use crate::{error::StakeError, StakeAccount, StakeConfig, UserAccount};
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        Metadata, MetadataAccount,
    },
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    pub collection_mint: Account<'info, Mint>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        seeds::program = metadata_program,
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub edition: Account<'info, MetadataAccount>,

    #[account(
        mut,
        close = user,
        seeds = [b"stake", mint.key().as_ref(), config.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_config: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        // check for time elapsed
        let time_elapsed =
            ((Clock::get()?.unix_timestamp - self.stake_account.staked_at) / 86400) as u32;
        require!(
            time_elapsed > self.config.freeze_period,
            StakeError::FreezePeriodNotExpired
        );

        self.user_config.points += (self.config.points_per_stake as u32) * time_elapsed;

        // unfreeze the nft
        let accounts = ThawDelegatedAccountCpiAccounts {
            mint: &self.mint.to_account_info(),
            delegate: &self.stake_account.to_account_info(),
            edition: &self.edition.to_account_info(),
            token_account: &self.user_mint_ata.to_account_info(),
            token_program: &self.token_program.to_account_info(),
        };

        let mint_key = self.mint.key();
        let config_key = self.config.key();

        let seeds = &[
            b"stake",
            mint_key.as_ref(),
            config_key.as_ref(),
            &[self.stake_account.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let _ = ThawDelegatedAccountCpi::new(&self.metadata_program.to_account_info(), accounts)
            .invoke_signed(signer_seeds)?;

        Ok(())
    }
}
