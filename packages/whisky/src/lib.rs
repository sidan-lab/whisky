//! # whisky
//!
//! `whisky` is built with the same pattern as [MeshJS's lower level APIs](https://meshjs.dev/apis/transaction/builderExample) where Rust Cardano developer can import directly for use, building on top of [`sidan-csl-rs`](../sidan_csl_rs/).
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
//! use whisky::{
//!     builder::MeshTxBuilder,
//!     model::{Asset, UTxO},
//! };
//!
//! async fn my_first_whisky_tx(
//!     recipient_address: &str,
//!     my_address: &str,
//!     inputs: &[UTxO],
//! ) -> String {
//!     let mut mesh = MeshTxBuilder::new_core();
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
//! All user facing APIs are documentation at the [builder interface](builder/struct.MeshTxBuilder.html).
//!
pub mod builder;
pub mod provider;
pub mod service;
pub mod transaction;
pub mod utils;
pub use sidan_csl_rs::core;
pub use sidan_csl_rs::csl;
pub use sidan_csl_rs::model;
