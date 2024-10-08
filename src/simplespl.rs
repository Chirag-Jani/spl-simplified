use anchor_lang::solana_program::account_info::AccountInfo;

use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::{context::CpiContext, Accounts};
use anchor_lang::{solana_program, Result};
use std::ops::Deref;

pub use spl_token;
pub use spl_token::ID;

pub mod simple_token {
    use anchor_lang::Key;

    use super::*;

    pub fn amount() -> Result<u64> {
        Ok(200)
    }

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
        solana_program::program::invoke_signed(
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
        /*   let ix = spl_token::instruction::mint_to(
            &spl_token::ID,
            &mint.key(),
            &to.key(),
            &authority.key(),
            &[],
            amount,
        )?;

        solana_program::program::invoke_signed(
            &ix,
            &[token_program, mint, to, authority],
            &[signer_seeds],
        )
        .map_err(Into::into) */
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintTo<'info> {
    pub mint: AccountInfo<'info>,
    pub to: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
}
