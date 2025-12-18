use std::str::FromStr;

use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::{conway::StakeCredential as PallasStakeCredential, Fragment};

pub enum StakeCredentialKind {
    KeyHash { key_hash_hex: String },
    ScriptHash { script_hash_hex: String },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct StakeCredential {
    inner: PallasStakeCredential,
}

impl StakeCredential {
    pub fn new(stake_credential: StakeCredentialKind) -> Result<Self, String> {
        let pallas_stake_credential = match stake_credential {
            StakeCredentialKind::KeyHash { key_hash_hex } => {
                let key_hash = Hash::<28>::from_str(&key_hash_hex)
                    .map_err(|e| format!("Invalid key hash length: {}", e))?;
                PallasStakeCredential::AddrKeyhash(key_hash)
            }

            StakeCredentialKind::ScriptHash { script_hash_hex } => {
                let script_hash = Hash::<28>::from_str(&script_hash_hex)
                    .map_err(|e| format!("Invalid script hash length: {}", e))?;
                PallasStakeCredential::ScriptHash(script_hash)
            }
        };

        Ok(Self {
            inner: pallas_stake_credential,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at StakeCredential"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasStakeCredential::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
