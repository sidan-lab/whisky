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
//! ## Getting Started
//!
//! ```rust
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
pub mod builder;
pub mod services;
pub mod transaction;
pub mod utils;
pub use builder::*;
pub use transaction::*;
pub use whisky_common::*;
pub use whisky_csl::*;
pub use whisky_provider::*;
pub use whisky_wallet::*;
