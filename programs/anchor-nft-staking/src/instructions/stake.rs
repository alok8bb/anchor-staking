#![allow(unexpected_cfgs)]

use crate::error::StakeError;
use crate::StakeAccount;
use crate::StakeConfig;
use crate::UserAccount;

use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

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
        init,
        payer = user,
        space = 8 + StakeAccount::INIT_SPACE,
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
        seeds = [b"user", user.to_account_info().key().as_ref()],
        bump = user_state.bump
    )]
    pub user_state: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: StakeBumps) -> Result<()> {
        require!(
            self.user_state.amount_staked < self.config.max_stake,
            StakeError::MaxStakeReached
        );

        self.stake_account.set_inner(StakeAccount {
            owner: self.user.to_account_info().key(),
            mint: self.mint.to_account_info().key(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: bumps.stake_account,
        });

        // delegate the NFT to program
        let cpi_accounts = Approve {
            to: self.user_mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        approve(cpi_context, 1)?;

        // freeze the user_mint_ata account with program authority
        let freeze_cpi_accounts = FreezeDelegatedAccountCpiAccounts {
            delegate: &self.metadata_program.to_account_info(),
            token_account: &self.user_mint_ata.to_account_info(),
            edition: &self.edition.to_account_info(),
            mint: &self.mint.to_account_info(),
            token_program: &self.token_program.to_account_info(),
        };

        FreezeDelegatedAccountCpi::new(
            &self.metadata_program.to_account_info(),
            freeze_cpi_accounts,
        ).invoke()?;

        self.user_state.amount_staked += 1;

        Ok(())
    }
}
