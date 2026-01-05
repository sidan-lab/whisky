use std::str::FromStr;

use pallas::{
    codec::utils::NonEmptySet,
    crypto::hash::Hash,
    ledger::primitives::{conway::RequiredSigners as PallasRequiredSigners, Fragment},
};
use whisky_common::WError;

#[derive(Debug, Clone)]
pub struct RequiredSigners {
    pub inner: PallasRequiredSigners,
}

impl RequiredSigners {
    pub fn new(signers: Vec<String>) -> Result<Self, WError> {
        let hashes: Result<Vec<_>, _> = signers
            .iter()
            .map(|signer| {
                Hash::<28>::from_str(signer).map_err(|e| {
                    WError::new(
                        "RequiredSigners::new",
                        &format!("Invalid signer hash: {}", e),
                    )
                })
            })
            .collect();

        let hashes = hashes?;
        let required_signers = NonEmptySet::from_vec(hashes).ok_or_else(|| {
            WError::new(
                "RequiredSigners::new",
                "required signers has to be non empty",
            )
        })?;

        Ok(Self {
            inner: required_signers,
        })
    }

    pub fn encode(&self) -> Result<String, WError> {
        let encoded = self.inner.encode_fragment().map_err(|e| {
            WError::new(
                "RequiredSigners::encode",
                &format!("Fragment encode error: {}", e),
            )
        })?;
        Ok(hex::encode(encoded))
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasRequiredSigners::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "RequiredSigners::decode_bytes",
                &format!("Fragment decode error: {}", e.to_string()),
            )
        })?;
        Ok(Self { inner })
    }
}
