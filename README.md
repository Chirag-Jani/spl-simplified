# SPL Token Helper Crate

**Status:** 🚧 Under Development 🚧

This crate simplifies the process of creating SPL tokens on the Solana blockchain by providing a user-friendly abstraction layer. Our goal is to streamline the token creation process with minimal setup, while also offering flexibility for advanced users.

## Features

- **Easy SPL Token Creation:** Quickly create and manage SPL tokens with metadata in just a few steps.
- **Manual Setup Options:** Advanced users can manually configure token parameters and metadata through the provided functions.
- **Token Transfer Support:** Transfer SPL tokens between accounts using a simplified interface.
- **Metadata Integration:** Automatically creates token metadata using the `mpl_token_metadata` program, with support for custom metadata attributes like name, symbol, URI, and seller fees.
- **Developer-Friendly:** The crate is modular and well-documented, making it easy to integrate into existing Solana projects.

## Usage

### Minting Tokens with Metadata

The `mint_simple` function allows you to mint SPL tokens and automatically create the associated metadata.

```rust
use spl_token_helper::mint_simple;
use anchor_lang::solana_program::account_info::AccountInfo;

mint_simple(
    "TokenName".to_string(),
    "TKN".to_string(),
    "https://example.com/token-metadata".to_string(),
    500, // 5% seller fee
    payer_account_info,
    token_metadata_program_info,
    update_authority_info,
    metadata_account_info,
    mint_authority_info,
    system_program_info,
    rent_sysvar_info,
    token_program_info,
    mint_account_info,
    to_account_info,
    owner_account_info,
    &[&signer_seeds],
    1000 // Mint 1000 tokens
).unwrap();
```

### Transferring SPL Tokens

The `transfer_simple` function lets you transfer SPL tokens between accounts using a minimal setup.

```rust
use spl_token_helper::transfer_simple;

transfer_simple(
    mint_account_info,
    token_program_account_info,
    source_pubkey,
    destination_account_info,
    authority_account_info,
    500, // Transfer 500 tokens
    &[&signer_seeds],
).unwrap();
```

## Contributing

This project is under active development, and we welcome contributions from the community! If you're interested in contributing, please feel free to:

- Fork the repository
- Open issues or feature requests
- Submit pull requests with improvements

## Call to Action

If you find this project useful or want to stay updated on its progress, please consider:

- ⭐ Starring this repository
- 👁️‍🗨️ Following the updates
- 🍴 Forking the repo to explore and contribute

Your support and feedback will help us build a better tool for the Solana developer community!

---

We look forward to your contributions! 🚀
