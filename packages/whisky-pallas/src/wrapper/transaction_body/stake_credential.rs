use std::str::FromStr;

use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::{conway::StakeCredential as PallasStakeCredential, Fragment};
use whisky_common::WError;

pub enum StakeCredentialKind {
    KeyHash { key_hash_hex: String },
    ScriptHash { script_hash_hex: String },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct StakeCredential {
    pub inner: PallasStakeCredential,
}

impl StakeCredential {
    pub fn new(stake_credential: StakeCredentialKind) -> Result<Self, WError> {
        let pallas_stake_credential = match stake_credential {
            StakeCredentialKind::KeyHash { key_hash_hex } => {
                let key_hash = Hash::<28>::from_str(&key_hash_hex).map_err(|e| {
                    WError::new("StakeCredential - Invalid key hash length", &e.to_string())
                })?;
                PallasStakeCredential::AddrKeyhash(key_hash)
            }

            StakeCredentialKind::ScriptHash { script_hash_hex } => {
                let script_hash = Hash::<28>::from_str(&script_hash_hex).map_err(|e| {
                    WError::new(
                        "StakeCredential - Invalid script hash length",
                        &e.to_string(),
                    )
                })?;
                PallasStakeCredential::ScriptHash(script_hash)
            }
        };

        Ok(Self {
            inner: pallas_stake_credential,
        })
    }

    pub fn encode(&self) -> Result<String, WError> {
        let encoded_fragment = self
            .inner
            .encode_fragment()
            .map_err(|e| WError::new("StakeCredential - Fragment encode error", &e.to_string()))?;
        Ok(hex::encode(encoded_fragment))
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasStakeCredential::decode_fragment(&bytes)
            .map_err(|e| WError::new("StakeCredential - Fragment decode error", &e.to_string()))?;
        Ok(Self { inner })
    }
}
