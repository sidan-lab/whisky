use std::str::FromStr;

use pallas::{
    crypto::hash::Hash,
    ledger::primitives::{conway::Constitution as PallasConstitution, Fragment},
};

use crate::wrapper::Anchor;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Constitution {
    pub inner: PallasConstitution,
}

impl Constitution {
    pub fn new(anchor: Anchor, guardrail_script_hash: Option<String>) -> Result<Self, String> {
        let guardrail_script = match guardrail_script_hash {
            Some(hash_str) => {
                Some(Hash::<28>::from_str(&hash_str).expect("Invalid guardrail script hash length"))
            }
            None => None,
        };

        Ok(Self {
            inner: PallasConstitution {
                anchor: anchor.inner,
                guardrail_script,
            },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at Constitution"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasConstitution::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
