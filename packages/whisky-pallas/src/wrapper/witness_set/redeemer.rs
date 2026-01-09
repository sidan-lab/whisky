use pallas::ledger::primitives::{conway::Redeemer as PallasRedeemer, Fragment};
use whisky_common::WError;

use crate::wrapper::witness_set::plutus_data::PlutusData;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum RedeemerTag {
    Spend,
    Mint,
    Cert,
    Reward,
    Vote,
    Propose,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExUnits {
    pub mem: u64,
    pub steps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redeemer {
    pub inner: PallasRedeemer,
}

impl Redeemer {
    pub fn new(
        tag: RedeemerTag,
        index: u32,
        data: PlutusData,
        ex_units: ExUnits,
    ) -> Result<Self, WError> {
        let pallas_tag = match tag {
            RedeemerTag::Spend => pallas::ledger::primitives::conway::RedeemerTag::Spend,
            RedeemerTag::Mint => pallas::ledger::primitives::conway::RedeemerTag::Mint,
            RedeemerTag::Cert => pallas::ledger::primitives::conway::RedeemerTag::Cert,
            RedeemerTag::Reward => pallas::ledger::primitives::conway::RedeemerTag::Reward,
            RedeemerTag::Vote => pallas::ledger::primitives::conway::RedeemerTag::Vote,
            RedeemerTag::Propose => pallas::ledger::primitives::conway::RedeemerTag::Propose,
        };
        let inner = PallasRedeemer {
            tag: pallas_tag,
            index,
            data: data.inner,
            ex_units: pallas::ledger::primitives::conway::ExUnits {
                mem: ex_units.mem,
                steps: ex_units.steps,
            },
        };
        Ok(Self { inner })
    }

    pub fn encode(&self) -> Result<String, WError> {
        self.inner
            .encode_fragment()
            .map(|bytes| hex::encode(bytes))
            .map_err(|_| {
                WError::new(
                    "WhiskyPallas - Encoding redeemer:",
                    "Failed to encode fragment",
                )
            })
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasRedeemer::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Decoding redeemer:",
                &format!("Fragment decode error: {}", e.to_string()),
            )
        })?;
        Ok(Self { inner })
    }
}
