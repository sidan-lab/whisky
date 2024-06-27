//! # sidan-csl-rs
//!
//! Welcome to my Rust package. This library is designed to provide an example of how to structure
//! a foreword or introductory text in your Rust documentation.
//!
//! ## Features
//!
//! - Feature 1: Description of feature 1.
//! - Feature 2: Description of feature 2.
//!
//! ## Quick Start
//!
//! Here's a quick example to get you started:
//!
//! ```
//! use my_rust_package::some_function;
//!
//! some_function();
//! ```
//!
//! For more information, see the [GitHub repository](https://github.com/example/my_rust_package).
//!
//!
pub mod builder;
pub mod core;
pub mod model;
pub use cardano_serialization_lib as csl;

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use noop_proc_macro::wasm_bindgen;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
use wasm_bindgen::prelude::{wasm_bindgen, JsError, JsValue};
