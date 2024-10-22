use anchor_lang::context::CpiContext;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::Key;
use anchor_lang::{solana_program, Result};
use mpl_token_metadata::types::{Creator, DataV2};
use solana_program::program::invoke_signed;
pub use spl_token::ID;

use crate::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3};

/// Mints new SPL tokens with associated metadata.
///
/// This function creates metadata for a new token using the `mpl_token_metadata`
/// program, and mints the specified number of tokens to the `to` account.
///
/// # Arguments
///
/// * `token_name` - The name of the token to be minted.
/// * `token_symbol` - The symbol for the token.
/// * `token_uri` - A URI pointing to the token's metadata (e.g., hosted image or metadata information).
/// * `token_tax` - The seller fee basis points (bps) for token transactions.
/// * `payer` - The account responsible for paying transaction fees.
/// * `token_metadata_program` - The account for the token metadata program.
/// * `update_authority` - The account authorized to update the token's metadata.
/// * `metadata` - The metadata account for the token.
/// * `mint_authority` - The account authorized to mint the tokens.
/// * `system_program` - The system program account.
/// * `rent` - The rent sysvar account.
/// * `token_program` - The SPL token program account.
/// * `mint` - The token mint account.
/// * `to` - The account where the minted tokens will be transferred to.
/// * `owner` - The owner of the `to` account.
/// * `signer_seeds` - The seeds used to sign the transaction.
/// * `amount` - The number of tokens to mint.
///
/// # Example
///
/// ```rust
/// use my_crate::mint_simple;
/// use anchor_lang::solana_program::account_info::AccountInfo;
///
/// let result = mint_simple(
///     "TokenName".to_string(),
///     "TKN".to_string(),
///     "https://example.com/token-metadata".to_string(),
///     500, // 5% seller fee
///     payer_account_info,
///     token_metadata_program_info,
///     update_authority_info,
///     metadata_account_info,
///     mint_authority_info,
///     system_program_info,
///     rent_sysvar_info,
///     token_program_info,
///     mint_account_info,
///     to_account_info,
///     owner_account_info,
///     &[&signer_seeds],
///     1000 // Mint 1000 tokens
/// ).unwrap();
/// ```
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
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    owner: AccountInfo<'info>,
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
        &owner.key(),
        &[],
        amount,
    )?;

    invoke_signed(
        &ix,
        &[mint.clone(), to.clone(), owner.clone(), token_program],
        &[signer_seeds],
    )?;

    Ok(())
}

/// Creates metadata for a token using the `mpl_token_metadata` program.
///
/// This function sets up the metadata for a token, including its name, symbol, URI,
/// seller fee, and creator information. It also uses the provided `signer_seed` for
/// signing the metadata creation transaction.
///
/// # Arguments
///
/// * `token_name` - The name of the token.
/// * `token_symbol` - The symbol of the token.
/// * `token_uri` - A URI that points to the token metadata.
/// * `token_tax` - The seller fee in basis points (bps).
/// * `payer` - The account paying for the transaction.
/// * `token_metadata_program` - The token metadata program account.
/// * `update_authority` - The account authorized to update the metadata.
/// * `mint` - The mint account for the token.
/// * `metadata` - The metadata account.
/// * `mint_authority` - The account authorized to mint the tokens.
/// * `system_program` - The system program account.
/// * `rent` - The rent sysvar account.
/// * `signer_seed` - A slice of slices of seeds for signing the metadata creation transaction.
///
/// # Example
///
/// ```rust
/// use my_crate::metadata_thing;
/// use anchor_lang::solana_program::account_info::AccountInfo;
///
/// metadata_thing(
///     "TokenName".to_string(),
///     "TKN".to_string(),
///     "https://example.com/token-metadata".to_string(),
///     500, // 5% seller fee
///     payer_account_info,
///     token_metadata_program_info,
///     update_authority_info,
///     mint_account_info,
///     metadata_account_info,
///     mint_authority_info,
///     system_program_info,
///     rent_sysvar_info,
///     &[&signer_seed],
/// ).unwrap();
/// ```
fn metadata_thing<'info>(
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
    let mut creators: Vec<Creator> = Vec::<Creator>::new();
    let creator: Creator = Creator {
        address: mint.key(),
        verified: true,
        share: 100,
    };

    creators.push(creator);

    let token_data: DataV2 = DataV2 {
        name: token_name.clone(),
        symbol: token_symbol,
        uri: token_uri,
        seller_fee_basis_points: token_tax,
        creators: Some(creators),
        collection: None,
        uses: None,
    };

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
        signer_seed,
    );

    create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;

    Ok(())
}

/// Transfers SPL tokens from one account to another.
///
/// This function facilitates the transfer of SPL tokens between accounts
/// on the Solana blockchain. It uses the `spl_token::instruction::transfer` function
/// to create the transfer instruction, and signs the transaction using the provided
/// `signer_seeds`.
///
/// # Arguments
///
/// * `mint` - The mint account of the token.
/// * `token_program_id` - The token program account (usually `spl_token`).
/// * `source_pubkey` - The source account's public key from which tokens will be transferred.
/// * `destination_pubkey` - The destination account's `AccountInfo`.
/// * `authority_pubkey` - The authority account that will sign the transfer.
/// * `amount` - The amount of tokens to transfer.
/// * `signer_seeds` - A slice of slices of seeds for signing the transaction.
///
/// # Example
///
/// ```rust
/// use my_crate::transfer_simple;
/// use anchor_lang::solana_program::account_info::AccountInfo;
///
/// transfer_simple(
///     mint_account_info,
///     token_program_account_info,
///     source_pubkey,
///     destination_account_info,
///     authority_account_info,
///     500, // Transfer 500 tokens
///     &[&signer_seeds],
/// ).unwrap();
/// ```
pub fn transfer_simple<'info>(
    mint: AccountInfo<'info>,
    token_program_id: AccountInfo<'info>,
    source_pubkey: Pubkey,
    destination_pubkey: AccountInfo<'info>,
    authority_pubkey: AccountInfo<'info>,
    amount: u64,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    let ix = spl_token::instruction::transfer(
        &token_program_id.key(),
        &source_pubkey,
        &destination_pubkey.key(),
        &authority_pubkey.key(),
        &[],
        amount,
    )?;

    invoke_signed(
        &ix,
        &[mint, destination_pubkey, authority_pubkey, token_program_id],
        &[signer_seeds],
    )?;

    Ok(())
}
