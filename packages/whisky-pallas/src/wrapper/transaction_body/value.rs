use pallas::ledger::primitives::{conway::Value as PallasValue, Fragment};

use crate::wrapper::transaction_body::MultiassetPositiveCoin;

#[derive(Debug, PartialEq, Clone)]
pub struct Value {
    pub inner: PallasValue,
}

impl Value {
    pub fn new(coin: u64, multiasset: Option<MultiassetPositiveCoin>) -> Self {
        match multiasset {
            Some(ma) => Self {
                inner: PallasValue::Multiasset(coin, ma.inner),
            },
            None => Self {
                inner: PallasValue::Coin(coin),
            },
        }
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at Value"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasValue::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
