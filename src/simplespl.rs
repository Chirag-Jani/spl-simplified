use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::Key;
use anchor_lang::{context::CpiContext, Accounts};
use anchor_lang::{solana_program, Result};
use solana_program::program::invoke_signed;
pub use spl_token::ID;

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

#[derive(Accounts)]
pub struct MintTo<'info> {
    pub mint: AccountInfo<'info>,
    pub to: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
}
