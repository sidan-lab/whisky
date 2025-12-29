use std::str::FromStr;

use pallas::{
    codec::utils::Bytes,
    ledger::primitives::{conway::VKeyWitness as PallasVKeyWitness, Fragment},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VKeyWitness {
    pub inner: PallasVKeyWitness,
}

impl VKeyWitness {
    pub fn new(vkey: String, signature: String) -> Result<Self, String> {
        let inner = PallasVKeyWitness {
            vkey: Bytes::from_str(&vkey).map_err(|e| e.to_string())?,
            signature: Bytes::from_str(&signature).map_err(|e| e.to_string())?,
        };
        Ok(Self { inner })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at VKeyWitness"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasVKeyWitness::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
