/// SPL Token Helper Crate Modules
///
/// This crate provides various modules to simplify working with SPL tokens on the Solana blockchain.
/// Each module is enabled through feature flags, allowing developers to include only the necessary components
/// in their projects. Below are the available modules and their functionalities:

#[cfg(feature = "associated_token")]
/// Associated Token Module
///
/// This module provides functions and utilities for creating and managing associated token accounts.
/// It simplifies the process of handling token accounts for users by automating account creation
/// and management when interacting with SPL tokens.
pub mod associated_token;

#[cfg(feature = "metadata")]
/// Metadata Module
///
/// This module offers functionalities for handling token metadata using the `mpl_token_metadata` program.
/// It includes functions for creating and updating token metadata, which is essential for defining
/// the properties of SPL tokens such as name, symbol, and URI.
pub mod metadata;

#[cfg(feature = "token")]
/// Token Module
///
/// This module encapsulates the core SPL token functionalities, including token minting,
/// transferring, and burning operations. It provides a simplified interface for developers
/// to interact with SPL tokens on the Solana blockchain.
pub mod token;

#[cfg(feature = "simplespl")]
/// Simple SPL Module
///
/// This module provides simple and user-friendly functions for creating and managing SPL tokens
/// with minimal setup. It is designed for developers who want to quickly deploy SPL tokens
/// without dealing with the complexities of the underlying SPL token program directly.
pub mod simplespl;
