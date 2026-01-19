use serde_json::{json, Value};

use crate::{data::{ByteString, Constr0, Constr1, PlutusDataJson}, WError};
use whisky_macros::ImplConstr;

use super::{byte_string, constr0, constr1};

#[derive(Clone, Debug)]
pub enum Credential {
    VerificationKey(VerificationKey),
    Script(Script),
}

impl Credential {
    pub fn new((hash, is_script): (&str, bool)) -> Self {
        if is_script {
            Credential::Script(Script::from(hash))
        } else {
            Credential::VerificationKey(VerificationKey::from(hash))
        }
    }
}

#[derive(Clone, Debug, ImplConstr)]
pub struct VerificationKey(pub Constr0<ByteString>);

#[derive(Clone, Debug, ImplConstr)]
pub struct Script(pub Constr1<ByteString>);

impl PlutusDataJson for Credential {
    fn to_json(&self) -> Value {
        match self {
            Credential::VerificationKey(vk) => vk.to_json(),
            Credential::Script(script) => script.to_json(),
        }
    }

    fn from_json(value: &Value) -> Result<Self, WError> {
        let tag = value
            .get("constructor")
            .ok_or_else(|| WError::new("Credential::from_json", "missing 'constructor' field"))?
            .as_u64()
            .ok_or_else(|| WError::new("Credential::from_json", "invalid 'constructor' value"))?;

        match tag {
            0 => VerificationKey::from_json(value).map(Credential::VerificationKey),
            1 => Script::from_json(value).map(Credential::Script),
            _ => Err(WError::new(
                "Credential::from_json",
                &format!("unknown constructor tag: {}", tag),
            )),
        }
    }
}

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

impl PlutusDataJson for Address {
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

    fn to_constr_field(&self) -> Vec<serde_json::Value> {
        vec![self.to_json()]
    }

    fn from_json(value: &Value) -> Result<Self, WError> {
        // Address format: {"constructor": 0, "fields": [payment_credential, stake_credential_maybe]}
        let tag = value
            .get("constructor")
            .ok_or_else(|| WError::new("Address::from_json", "missing 'constructor' field"))?
            .as_u64()
            .ok_or_else(|| WError::new("Address::from_json", "invalid 'constructor' value"))?;

        if tag != 0 {
            return Err(WError::new(
                "Address::from_json",
                &format!("expected constructor tag 0 for Address, got {}", tag),
            ));
        }

        let fields = value
            .get("fields")
            .ok_or_else(|| WError::new("Address::from_json", "missing 'fields' field"))?
            .as_array()
            .ok_or_else(|| WError::new("Address::from_json", "invalid 'fields' value"))?;

        if fields.len() != 2 {
            return Err(WError::new(
                "Address::from_json",
                "expected 2 fields for Address",
            ));
        }

        // Parse payment credential: {"constructor": 0/1, "fields": [{"bytes": "..."}]}
        let payment_cred = &fields[0];
        let payment_tag = payment_cred
            .get("constructor")
            .and_then(|c| c.as_u64())
            .ok_or_else(|| WError::new("Address::from_json", "invalid payment credential"))?;
        let is_script_payment_key = payment_tag == 1;

        let payment_fields = payment_cred
            .get("fields")
            .and_then(|f| f.as_array())
            .ok_or_else(|| WError::new("Address::from_json", "invalid payment credential fields"))?;

        let payment_key_hash = payment_fields
            .first()
            .and_then(|f| f.get("bytes"))
            .and_then(|b| b.as_str())
            .ok_or_else(|| WError::new("Address::from_json", "invalid payment key hash"))?
            .to_string();

        // Parse stake credential: {"constructor": 0/1, "fields": [...]}
        // Constr0 means Some(staking_hash), Constr1 means None
        let stake_cred = &fields[1];
        let stake_tag = stake_cred
            .get("constructor")
            .and_then(|c| c.as_u64())
            .ok_or_else(|| WError::new("Address::from_json", "invalid stake credential"))?;

        let (stake_credential, is_script_stake_key) = if stake_tag == 1 {
            // None - no stake credential
            (None, false)
        } else {
            // Some - has stake credential
            let stake_fields = stake_cred
                .get("fields")
                .and_then(|f| f.as_array())
                .ok_or_else(|| WError::new("Address::from_json", "invalid stake credential fields"))?;

            if stake_fields.is_empty() {
                (None, false)
            } else {
                // Navigate: Constr0([Constr0([Constr0/1([{"bytes": "..."}])])])
                let inner_wrapper = stake_fields.first()
                    .and_then(|f| f.get("fields"))
                    .and_then(|f| f.as_array())
                    .and_then(|f| f.first())
                    .ok_or_else(|| WError::new("Address::from_json", "invalid stake credential structure"))?;

                let inner_tag = inner_wrapper
                    .get("constructor")
                    .and_then(|c| c.as_u64())
                    .ok_or_else(|| WError::new("Address::from_json", "invalid stake credential inner tag"))?;

                let is_script = inner_tag == 1;

                let stake_hash = inner_wrapper
                    .get("fields")
                    .and_then(|f| f.as_array())
                    .and_then(|f| f.first())
                    .and_then(|f| f.get("bytes"))
                    .and_then(|b| b.as_str())
                    .ok_or_else(|| WError::new("Address::from_json", "invalid stake key hash"))?
                    .to_string();

                (Some(stake_hash), is_script)
            }
        };

        Ok(Address {
            payment_key_hash,
            stake_credential,
            is_script_payment_key,
            is_script_stake_key,
        })
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
