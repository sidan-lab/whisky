use std::str::FromStr;

use pallas::{
    codec::utils::Bytes,
    ledger::primitives::{Fragment, PoolMetadata as PallasPoolMetadata},
};
use whisky_common::WError;

pub struct PoolMetadata {
    pub inner: PallasPoolMetadata,
}

impl PoolMetadata {
    pub fn new(url: String, hash: String) -> Result<Self, WError> {
        let hash_bytes = Bytes::from_str(&hash).map_err(|e| {
            WError::new(
                "WhiskyPallas - Creating PoolMetadata:",
                &format!("Invalid hash hex: {}", e),
            )
        })?;
        let inner = PallasPoolMetadata {
            url,
            hash: hash_bytes,
        };
        Ok(Self { inner })
    }

    pub fn encode(&self) -> Result<String, WError> {
        let encoded_fragment = self
            .inner
            .encode_fragment()
            .map_err(|e| WError::new("WhiskyPallas - PoolMetadata encode:", &e.to_string()))?;
        Ok(hex::encode(encoded_fragment))
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasPoolMetadata::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Creating PoolMetadata:",
                &format!("Fragment decode error: {}", e),
            )
        })?;
        Ok(Self { inner })
    }
}
