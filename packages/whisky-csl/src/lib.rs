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
mod errors;
mod models;
mod tx_builder;
mod tx_parser;
mod utils;
mod wallet;
pub use constants::*;
pub use errors::*;
pub use models::*;
pub use tx_builder::*;
pub use tx_parser::*;
pub use utils::*;
pub use wallet::*;

pub mod wasm;
pub use wasm::*;

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use wasm_bindgen::prelude::{wasm_bindgen, JsValue, WError};
