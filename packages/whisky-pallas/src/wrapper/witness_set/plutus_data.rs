use pallas::{
    interop::utxorpc::spec::cardano::plutus_data,
    ledger::primitives::{conway::PlutusData as PallasPlutusData, Fragment},
};
use whisky_common::WError;

#[derive(Debug, Clone)]
pub struct PlutusData {
    pub inner: PallasPlutusData,
}

impl PlutusData {
    pub fn new(plutus_data_hex: String) -> Result<Self, WError> {
        let bytes = hex::decode(plutus_data_hex).map_err(|e| {
            WError::new(
                "WhiskyPallas - Creating Plutus data:",
                &format!("Hex decode error: {}", e),
            )
        })?;
        let inner = PallasPlutusData::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Creating Plutus data:",
                &format!("Fragment decode error: {}", e),
            )
        })?;
        Ok(Self { inner })
    }

    pub fn encode(&self) -> Result<String, WError> {
        self.inner
            .encode_fragment()
            .map(|bytes| hex::encode(bytes))
            .map_err(|_| {
                WError::new(
                    "WhiskyPallas - Encoding Plutus data:",
                    "Failed to encode fragment",
                )
            })
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasPlutusData::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Decoding Plutus data:",
                &format!("Fragment decode error: {}", e.to_string()),
            )
        })?;
        Ok(Self { inner })
    }
}
