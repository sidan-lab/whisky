use std::str::FromStr;

use pallas::{
    crypto::hash::Hash,
    ledger::primitives::{conway::Anchor as PallasAnchor, Fragment},
};
use whisky_common::WError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Anchor {
    pub inner: PallasAnchor,
}

impl Anchor {
    pub fn new(url: String, content_hash: String) -> Result<Self, WError> {
        let inner = PallasAnchor {
            url,
            content_hash: Hash::from_str(&content_hash)
                .map_err(|e| WError::new("Anchor - Invalid content hash", &e.to_string()))?,
        };
        Ok(Self { inner })
    }

    pub fn to_whisky_anchor(&self) -> whisky_common::Anchor {
        whisky_common::Anchor {
            anchor_url: self.inner.url.clone(),
            anchor_data_hash: self.inner.content_hash.to_string(),
        }
    }

    pub fn encode(&self) -> Result<String, WError> {
        let encoded_fragment = self
            .inner
            .encode_fragment()
            .map_err(|e| WError::new("Anchor - Fragment encode error", &e.to_string()))?;
        Ok(hex::encode(encoded_fragment))
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasAnchor::decode_fragment(&bytes)
            .map_err(|e| WError::new("Anchor - Fragment decode error", &e.to_string()))?;
        Ok(Self { inner })
    }
}
