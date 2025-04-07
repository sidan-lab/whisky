use serde_json::Value;

use super::{byte_string, con_str0, integer};

pub fn currency_symbol(policy_id: &str) -> Value {
    byte_string(policy_id)
}

pub fn token_name(token_name: &str) -> Value {
    byte_string(token_name)
}

pub fn asset_class(policy_id: &str, asset_name: &str) -> Value {
    con_str0(vec![currency_symbol(policy_id), token_name(asset_name)])
}

pub fn tx_out_ref(tx_hash: &str, index: i64) -> Value {
    con_str0(vec![con_str0(vec![byte_string(tx_hash)]), integer(index)])
}

pub fn output_reference(tx_hash: &str, index: i64) -> Value {
    con_str0(vec![byte_string(tx_hash), integer(index)])
}

pub fn posix_time(posix_time: i64) -> Value {
    integer(posix_time)
}
