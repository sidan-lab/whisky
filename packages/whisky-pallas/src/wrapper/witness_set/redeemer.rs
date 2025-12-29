use pallas::ledger::primitives::{conway::Redeemer as PallasRedeemer, Fragment};

use crate::wrapper::witness_set::plutus_data::PlutusData;

pub enum RedeemerTag {
    Spend,
    Mint,
    Cert,
    Reward,
    Vote,
    Propose,
}

pub struct ExUnits {
    pub mem: u64,
    pub steps: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Redeemer {
    pub inner: PallasRedeemer,
}

impl Redeemer {
    pub fn new(
        tag: RedeemerTag,
        index: u32,
        data: PlutusData,
        ex_units: ExUnits,
    ) -> Result<Self, String> {
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

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at Redeemer"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasRedeemer::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
