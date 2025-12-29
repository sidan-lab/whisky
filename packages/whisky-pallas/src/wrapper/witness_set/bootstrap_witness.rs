use std::str::FromStr;

use pallas::codec::utils::Bytes;
use pallas::ledger::primitives::{conway::BootstrapWitness as PallasBootstrapWitness, Fragment};
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BootstrapWitness {
    pub inner: PallasBootstrapWitness,
}

impl BootstrapWitness {
    pub fn new(
        public_key: String,
        signature: String,
        chain_code: String,
        attributes: String,
    ) -> Result<Self, String> {
        let inner = PallasBootstrapWitness {
            public_key: Bytes::from_str(&public_key)
                .expect("Invalid public key in bootstrap witness"),
            signature: Bytes::from_str(&signature).expect("Invalid signature in bootstrap witness"),
            chain_code: Bytes::from_str(&chain_code)
                .expect("Invalid chain code in bootstrap witness"),
            attributes: Bytes::from_str(&attributes)
                .expect("Invalid attributes in bootstrap witness"),
        };
        Ok(Self { inner })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at BootstrapWitness"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasBootstrapWitness::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
