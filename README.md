# SPL Token Helper Crate

**Status:** ✅ Initial Version Released ✅

This crate simplifies the process of creating SPL tokens on the Solana blockchain by providing a user-friendly abstraction layer. Our goal is to streamline the token creation process with minimal setup while also offering flexibility for advanced users. You can use the following functions:

- `mint_simple`: Mint SPL tokens with associated metadata.
- `transfer_simple`: Transfer SPL tokens between accounts.
- `burn_simple`: Burn SPL tokens from an account.

## Features

- **Easy SPL Token Creation:** Quickly create and manage SPL tokens with metadata in just a few steps.
- **Manual Setup Options:** Advanced users can manually configure token parameters and metadata through the provided functions.
- **Token Transfer Support:** Transfer SPL tokens between accounts using a simplified interface.
- **Metadata Integration:** Automatically creates token metadata using the `mpl_token_metadata` program, with support for custom metadata attributes like name, symbol, URI, and seller fees.
- **Token Burning:** Burn SPL tokens from an account with ease.
- **Developer-Friendly:** The crate is modular and well-documented, making it easy to integrate into existing Solana projects.

## Usage

### Minting Tokens with Metadata

The `mint_simple` function allows you to mint SPL tokens and automatically create the associated metadata.

```rust
use spl_token_helper::mint_simple;
use anchor_lang::solana_program::account_info::AccountInfo;

// Mint SPL tokens with metadata
let result = mint_simple(
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

// Transfer SPL tokens between accounts
let result = transfer_simple(
    mint_account_info,
    token_program_account_info,
    source_pubkey,
    destination_account_info,
    authority_account_info,
    500, // Transfer 500 tokens
    &[&signer_seeds],
).unwrap();
```

### Burning SPL Tokens

The `burn_simple` function allows you to burn (destroy) SPL tokens from a specified account.

```rust
use spl_token_helper::burn_simple;

// Burn SPL tokens from an account
let result = burn_simple(
    mint_account_info,
    token_program_id,
    source_pubkey,
    authority_account_info,
    500, // Burn 500 tokens
    &[&signer_seeds],
).unwrap();
```

## Example Implementations

You can find example implementations of each version of this crate in the [spl-simplified-demo](https://github.com/Chirag-Jani/spl-simplified-demo) repository. Check specific commits for version numbers included in the messages, such as "feat: demo v0.2.8".

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
