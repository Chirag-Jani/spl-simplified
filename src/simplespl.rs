use anchor_lang::solana_program::account_info::AccountInfo;

use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::{context::CpiContext, Accounts};
use anchor_lang::{solana_program, Result};
use std::ops::Deref;

pub use spl_token;
pub use spl_token::ID;

pub mod customblock {
    use super::*;

    pub fn amount() -> Result<u64> {
        Ok(200)
    }
}
