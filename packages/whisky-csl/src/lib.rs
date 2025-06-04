//! #whisky-csl
//!
//! `whisky-csl` composed of the core serialization logics with the pattern of json-to-transaction, compilable to wasm.
//! This library is not expected to be imported by Rust developers directly, but rather through the higher level APIs like [`whisky`](../whisky).
//! The wasm build and utility is mostly exposed in [MeshJS](https://meshjs.dev/).
//!
//! ## JS / TS wasm Install
//!
//! In case you want the directly out of the box wasm function for your JS / TS project, run the below
//!
//! ```sh
//! # For nodejs package
//! yarn add @sidan-lab/whisky-csl-nodejs
//! # For browser package
//! yarn add @sidan-lab/whisky-csl-browser
//! ```
//!
//! ## APIs
//!
//! - The serialization logic documentation at the [builder interface](builder/trait.ITxBuilderCore.html).
//! - The inline documentation of core json to transaction serialization function is served at [here](core/builder/fn.js_serialize_tx_body.html).

mod constants;
pub mod tx_builder;
pub mod tx_parser;
pub mod utils;
pub use constants::*;
pub use tx_builder::*;
pub use tx_parser::*;
pub use utils::*;

pub use cardano_serialization_lib as csl;
use whisky_common::TxBuilderBody;

#[derive(Clone, Debug)]
pub struct WhiskyCSL {
    pub core: CoreCSL,
    pub parser: CSLParser,
    pub tx_builder_body: TxBuilderBody,
    pub tx_evaluation_multiplier_percentage: u64,
    pub tx_hex: String,
}
