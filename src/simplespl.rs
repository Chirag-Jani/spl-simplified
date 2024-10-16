use anchor_lang::prelude::{Program, System, UncheckedAccount};
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::{context::CpiContext, Accounts};
use anchor_lang::{solana_program, Result};
use anchor_lang::{Key, ToAccountInfo};
use mpl_token_metadata::types::DataV2;
use solana_program::program::invoke_signed;
pub use spl_token::ID;

use crate::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3};

pub fn mint_to<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, MintTo<'info>>,
    amount: u64,
) -> Result<()> {
    let ix = spl_token::instruction::mint_to(
        &spl_token::ID,
        ctx.accounts.mint.key,
        ctx.accounts.to.key,
        ctx.accounts.authority.key,
        &[],
        amount,
    )?;
    invoke_signed(
        &ix,
        &[ctx.accounts.to, ctx.accounts.mint, ctx.accounts.authority],
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn mint_simple<'info>(
    token_program: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    signer_seeds: &[&[u8]],
    amount: u64,
) -> Result<()> {
    let ix = spl_token::instruction::mint_to(
        &spl_token::ID,
        &mint.key(),
        &to.key(),
        &authority.key(),
        &[],
        amount,
    )?;

    invoke_signed(
        &ix,
        &[mint.clone(), to.clone(), authority.clone(), token_program],
        &[signer_seeds],
    )?;

    // .map_err(Into::into);

    Ok(())
}

pub fn metadata_thing<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, MetaCtx<'info>>,
    token_name: String,
    token_symbol: String,
    token_uri: String,
    token_tax: u16,
) -> Result<()> {
    let token_data: DataV2 = DataV2 {
        name: token_name.clone(),
        symbol: token_symbol,
        uri: token_uri,
        seller_fee_basis_points: token_tax,
        creators: None,
        collection: None,
        uses: None,
    };

    // Create the metadata accounts
    let metadata_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program,
        CreateMetadataAccountsV3 {
            payer: ctx.accounts.payer,
            update_authority: ctx.accounts.update_authority,
            mint: ctx.accounts.mint.clone(),
            metadata: ctx.accounts.metadata.to_account_info(),
            mint_authority: ctx.accounts.mint_authority,
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.mint.clone().to_account_info(),
        },
        ctx.signer_seeds,
    );

    create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;

    Ok(())
}

#[derive(Accounts)]
pub struct MintTo<'info> {
    pub mint: AccountInfo<'info>,
    pub to: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MetaCtx<'info> {
    pub token_metadata_program: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub update_authority: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub metadata: UncheckedAccount<'info>,
    pub mint_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    // pub rent: Sysvar<'info, Rent>,
}
