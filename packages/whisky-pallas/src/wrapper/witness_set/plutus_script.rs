use std::str::FromStr;

use pallas::codec::utils::Bytes;
use pallas::ledger::primitives::conway::PlutusScript as PallasPlutusScript;
use pallas::ledger::primitives::Fragment;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PlutusScript<const VERSION: usize> {
    pub inner: PallasPlutusScript<VERSION>,
}

impl<const VERSION: usize> PlutusScript<VERSION> {
    pub fn new(script: String) -> Result<Self, String> {
        let inner = PallasPlutusScript::<VERSION>(
            Bytes::from_str(&script)
                .map_err(|e| format!("Invalid Plutus script: {}", e.to_string()))?,
        );

        Ok(Self { inner })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at PlutusV1Script"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasPlutusScript::<VERSION>::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
