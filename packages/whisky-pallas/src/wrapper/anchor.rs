use std::str::FromStr;

use pallas::{
    crypto::hash::Hash,
    ledger::primitives::{conway::Anchor as PallasAnchor, Fragment},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Anchor {
    inner: PallasAnchor,
}

impl Anchor {
    pub fn new(url: String, content_hash: String) -> Result<Self, String> {
        let inner = PallasAnchor {
            url,
            content_hash: Hash::from_str(&content_hash).map_err(|e| e.to_string())?,
        };
        Ok(Self { inner })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at Anchor"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasAnchor::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
