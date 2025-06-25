use serde_json::{json, Value};

use crate::PlutusDataToJson;

use super::{byte_string, constr0, constr1};

#[derive(Clone, Debug)]
pub struct Address {
    pub payment_key_hash: String,
    pub stake_credential: Option<String>,
    pub is_script_payment_key: bool,
    pub is_script_stake_key: bool,
}

impl Address {
    pub fn new(
        payment_key_hash: &str,
        stake_credential: Option<&str>,
        is_script_payment_key: bool,
        is_script_stake_key: bool,
    ) -> Self {
        Address {
            payment_key_hash: payment_key_hash.to_string(),
            stake_credential: stake_credential.map(|s| s.to_string()),
            is_script_payment_key,
            is_script_stake_key,
        }
    }
}

impl PlutusDataToJson for Address {
    fn to_json(&self) -> Value {
        if self.is_script_payment_key {
            script_address(
                &self.payment_key_hash,
                self.stake_credential.as_deref(),
                self.is_script_stake_key,
            )
        } else {
            pub_key_address(
                &self.payment_key_hash,
                self.stake_credential.as_deref(),
                self.is_script_stake_key,
            )
        }
    }

    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }
}

pub fn payment_pub_key_hash(pub_key_hash: &str) -> Value {
    byte_string(pub_key_hash)
}

pub fn pub_key_hash(pub_key_hash: &str) -> Value {
    byte_string(pub_key_hash)
}

pub fn maybe_staking_hash(stake_credential: &str, is_script_stake_key: bool) -> Value {
    if stake_credential.is_empty() {
        constr1(json!([]))
    } else if is_script_stake_key {
        constr0(vec![constr0(vec![constr1(vec![byte_string(
            stake_credential,
        )])])])
    } else {
        constr0(vec![constr0(vec![constr0(vec![byte_string(
            stake_credential,
        )])])])
    }
}

pub fn pub_key_address(
    bytes: &str,
    stake_credential: Option<&str>,
    is_script_stake_key: bool,
) -> Value {
    constr0(vec![
        constr0(vec![byte_string(bytes)]),
        maybe_staking_hash(stake_credential.unwrap_or(""), is_script_stake_key),
    ])
}

pub fn script_address(
    bytes: &str,
    stake_credential: Option<&str>,
    is_script_stake_key: bool,
) -> Value {
    constr0(vec![
        constr1(vec![byte_string(bytes)]),
        maybe_staking_hash(stake_credential.unwrap_or(""), is_script_stake_key),
    ])
}
