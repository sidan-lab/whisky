//! # whisky
//!
//! `whisky` is built with the same pattern as [MeshJS's lower level APIs](https://meshjs.dev/apis/transaction/builderExample) where Rust Cardano developer can import directly for use.
//!
//! ## Install
//!
//! In your Rust project, run the below
//!
//! ```sh
//! cargo add whisky
//! ```
//!
//! or add the dependency in `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! whisky = "^<the-latest-version>"
//! ```
//!
//! ## Feature Flags
//!
//! By default, all features are enabled. You can selectively enable features:
//!
//! ```toml
//! # Full (default) - all features
//! whisky = "1.0.18"
//!
//! # Just common types (minimal)
//! whisky = { version = "1.0.18", default-features = false }
//!
//! # Wallet only (includes csl + common)
//! whisky = { version = "1.0.18", default-features = false, features = ["wallet"] }
//!
//! # Provider only (includes csl + common)
//! whisky = { version = "1.0.18", default-features = false, features = ["provider"] }
//!
//! # CSL only (transaction building without wallet/provider)
//! whisky = { version = "1.0.18", default-features = false, features = ["csl"] }
//! ```
//!
//! ## Getting Started
//!
//! ```rust,ignore
//! use whisky::*;
//!
//! async fn my_first_whisky_tx(
//!     recipient_address: &str,
//!     my_address: &str,
//!     inputs: &[UTxO],
//! ) -> String {
//!     let mut mesh = TxBuilder::new_core();
//!     mesh.tx_out(
//!         &recipient_address,
//!         &[Asset::new_from_str("lovelace", "1000000")],
//!     )
//!         .change_address(my_address)
//!         .select_utxos_from(inputs, 5000000)
//!         .complete(None)
//!         .await;
//!     mesh.tx_hex()
//! }
//! ```
//!
//! ## APIs
//!
//! All user facing APIs are documentation at the [builder interface](builder/struct.TxBuilder.html).
//!

// Self-reference alias to allow proc macros to use ::whisky:: path within this crate
extern crate self as whisky;

// Data module is always available (uses whisky_common + whisky_macros)
pub mod data;

// CSL-dependent modules
#[cfg(feature = "csl")]
pub mod builder;
#[cfg(feature = "csl")]
pub mod parser;
#[cfg(feature = "csl")]
pub mod transaction;
#[cfg(feature = "csl")]
pub mod utils;

// Services require both csl and provider
#[cfg(all(feature = "csl", feature = "provider"))]
pub mod services;

// Always re-export common and macros
pub use whisky_common::*;
pub use whisky_macros::*;

// CSL re-exports
#[cfg(feature = "csl")]
pub use builder::*;
#[cfg(feature = "csl")]
pub use parser::*;
#[cfg(feature = "csl")]
pub use transaction::*;
#[cfg(feature = "csl")]
pub use utils::*;
#[cfg(feature = "csl")]
pub use whisky_csl::*;

// Wallet re-exports
#[cfg(feature = "wallet")]
pub use whisky_wallet::*;

// Provider re-exports
#[cfg(feature = "provider")]
pub use whisky_provider::*;
