use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::Key;
use anchor_lang::{context::CpiContext, Accounts};
use anchor_lang::{solana_program, Result};
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
    token_name: String,
    token_symbol: String,
    token_uri: String,
    token_tax: u16,
    payer: AccountInfo<'info>,
    token_metadata_program: AccountInfo<'info>,
    update_authority: AccountInfo<'info>,
    metadata: AccountInfo<'info>,
    mint_authority: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    ////
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    signer_seeds: &[&[u8]],
    amount: u64,
) -> Result<()> {
    metadata_thing(
        token_name.clone(),
        token_symbol.clone(),
        token_uri.clone(),
        token_tax.clone(),
        payer,
        token_metadata_program,
        update_authority,
        mint.clone(),
        metadata,
        mint_authority,
        system_program,
        rent,
        &[&signer_seeds],
    )?;

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

fn metadata_thing<'info>(
    // ctx: CpiContext<'_, '_, '_, 'info, MetaCtx<'info>>,
    token_name: String,
    token_symbol: String,
    token_uri: String,
    token_tax: u16,
    payer: AccountInfo<'info>,
    token_metadata_program: AccountInfo<'info>,
    update_authority: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    metadata: AccountInfo<'info>,
    mint_authority: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    rent: AccountInfo<'info>,
    signer_seed: &[&[&[u8]]],
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
        token_metadata_program,
        CreateMetadataAccountsV3 {
            payer,
            update_authority,
            mint,
            metadata,
            mint_authority,
            system_program,
            rent,
        },
        signer_seed, // Keep the signer seeds from the original context
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
