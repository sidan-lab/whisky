use std::str::FromStr;

use pallas::{
    codec::utils::NonEmptySet,
    crypto::hash::Hash,
    ledger::primitives::{conway::RequiredSigners as PallasRequiredSigners, Fragment},
};

pub struct RequiredSigners {
    pub inner: PallasRequiredSigners,
}

impl RequiredSigners {
    pub fn new(signers: Vec<String>) -> Self {
        let required_signers = NonEmptySet::from_vec(
            signers
                .iter()
                .map(|signer| Hash::<28>::from_str(signer).expect("Invalid signer hash"))
                .collect(),
        )
        .expect("requied signers has to be non empty");

        Self {
            inner: required_signers,
        }
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at RequiredSigners"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasRequiredSigners::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
