use serde_json::{json, Value};

use super::{byte_string, con_str0, con_str1};

pub fn payment_pub_key_hash(pub_key_hash: &str) -> Value {
    byte_string(pub_key_hash)
}

pub fn pub_key_hash(pub_key_hash: &str) -> Value {
    byte_string(pub_key_hash)
}

pub fn maybe_staking_hash(stake_credential: &str, is_script_stake_key: bool) -> Value {
    if stake_credential.is_empty() {
        con_str1(json!([]))
    } else if is_script_stake_key {
        con_str0(vec![con_str0(vec![con_str1(vec![byte_string(
            stake_credential,
        )])])])
    } else {
        con_str0(vec![con_str0(vec![con_str0(vec![byte_string(
            stake_credential,
        )])])])
    }
}

pub fn pub_key_address(
    bytes: &str,
    stake_credential: Option<&str>,
    is_script_stake_key: bool,
) -> Value {
    con_str0(vec![
        con_str0(vec![byte_string(bytes)]),
        maybe_staking_hash(stake_credential.unwrap_or(""), is_script_stake_key),
    ])
}

pub fn script_address(
    bytes: &str,
    stake_credential: Option<&str>,
    is_script_stake_key: bool,
) -> Value {
    con_str0(vec![
        con_str1(vec![byte_string(bytes)]),
        maybe_staking_hash(stake_credential.unwrap_or(""), is_script_stake_key),
    ])
}
