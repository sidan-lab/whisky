// Self-reference alias to allow proc macros to use ::whisky:: path
// (proc macros generate code with ::whisky:: paths which work for external crates,
// and this alias makes them work within whisky-common itself)
extern crate self as whisky;

pub mod algo;
pub mod constants;
pub mod data;
pub mod errors;
pub mod interfaces;
pub mod models;
pub mod tx_tester;
pub mod utils;
pub use algo::*;
pub use constants::*;
pub use errors::*;
pub use interfaces::*;
pub use models::*;
pub use tx_tester::*;
pub use utils::*;

// Re-export Blueprint at root level
pub use data::blueprint::Blueprint;

// Re-export proc macros
pub use whisky_macros::impl_constr_type;
