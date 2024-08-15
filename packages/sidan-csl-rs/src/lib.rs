//! # sidan-csl-rs
//!
//! `sidan-csl-rs` composed of the core serialization logics with the pattern of json-to-transaction, compilable to wasm.
//! This library is not expected to be imported by Rust developers directly, but rather through the higher level APIs like [`whisky`](../whisky).
//! The wasm build and utility is mostly exposed in [MeshJS](https://meshjs.dev/).
//!
//! ## JS / TS wasm Install
//!
//! In case you want the directly out of the box wasm function for your JS / TS project, run the below
//!
//! ```sh
//! # For nodejs package
//! yarn add @sidan-lab/sidan-csl-rs-nodejs
//! # For browser package
//! yarn add @sidan-lab/sidan-csl-rs-browser
//! ```
//!
//! ## APIs
//!
//! - The serialization logic documentation at the [builder interface](builder/trait.IMeshTxBuilderCore.html).
//! - The inline documentation of core json to transaction serialization function is served at [here](core/builder/fn.js_serialize_tx_body.html).

pub mod core;
pub mod model;
pub mod wasm;
pub use cardano_serialization_lib as csl;

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};
