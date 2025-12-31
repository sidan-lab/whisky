use std::str::FromStr;

use pallas::codec::utils::Bytes;
use pallas::ledger::primitives::conway::PlutusScript as PallasPlutusScript;
use pallas::ledger::primitives::Fragment;
use whisky_common::WError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PlutusScript<const VERSION: usize> {
    pub inner: PallasPlutusScript<VERSION>,
}

impl<const VERSION: usize> PlutusScript<VERSION> {
    pub fn new(script: String) -> Result<Self, WError> {
        let inner = PallasPlutusScript::<VERSION>(Bytes::from_str(&script).map_err(|e| {
            WError::new(
                "WhiskyPallas - Creating Plutus script:",
                &format!("Invalid Plutus script: {}", e.to_string()),
            )
        })?);

        Ok(Self { inner })
    }

    pub fn encode(&self) -> Result<String, WError> {
        self.inner
            .encode_fragment()
            .map(|bytes| hex::encode(bytes))
            .map_err(|_| {
                WError::new(
                    "WhiskyPallas - Encoding Plutus script:",
                    "Failed to encode fragment",
                )
            })
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasPlutusScript::<VERSION>::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Decoding Plutus script:",
                &format!("Fragment decode error: {}", e.to_string()),
            )
        })?;
        Ok(Self { inner })
    }
}
