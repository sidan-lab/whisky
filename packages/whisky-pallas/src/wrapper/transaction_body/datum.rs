use pallas::{
    codec::utils::{CborWrap, KeepRaw},
    ledger::{
        primitives::{conway::DatumOption, Fragment, PlutusData as PallasPlutusData},
        traverse::ComputeHash,
    },
};
use std::str::FromStr;
use whisky_common::WError;

#[derive(Debug, PartialEq, Clone)]
pub enum DatumKind {
    Hash { datum_hash: String },
    Data { plutus_data_hex: String },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Datum<'a> {
    pub inner: DatumOption<'a>,
}

impl<'a> Datum<'a> {
    pub fn new(datum_option_kind: DatumKind) -> Result<Self, WError> {
        let pallas_datum_option = match datum_option_kind {
            DatumKind::Hash { datum_hash } => {
                let datum_hash = datum_hash.parse().map_err(|e| {
                    WError::new(
                        "WhiskyPallas - Creating datum:",
                        &format!("Invalid datum hash length: {}", e),
                    )
                })?;
                DatumOption::Hash(datum_hash)
            }

            DatumKind::Data { plutus_data_hex } => {
                let bytes = hex::decode(plutus_data_hex).map_err(|e| {
                    WError::new(
                        "WhiskyPallas - Creating datum:",
                        &format!("Hex decode error: {}", e),
                    )
                })?;
                let plutus_data = PallasPlutusData::decode_fragment(&bytes).map_err(|e| {
                    WError::new(
                        "WhiskyPallas - Creating datum:",
                        &format!("Fragment decode error: {}", e),
                    )
                })?;
                DatumOption::Data(CborWrap(KeepRaw::from(plutus_data)))
            }
        };

        Ok(Self {
            inner: pallas_datum_option,
        })
    }

    pub fn hash(&self) -> Result<String, WError> {
        Ok(DatumOption::compute_hash(&self.inner).to_string())
    }

    pub fn encode(&self) -> Result<String, WError> {
        self.inner
            .encode_fragment()
            .map(|bytes| hex::encode(bytes))
            .map_err(|_| {
                WError::new(
                    "WhiskyPallas - Encoding datum:",
                    "Failed to encode fragment",
                )
            })
    }

    pub fn decode(bytes: &'a [u8]) -> Result<Self, WError> {
        let pallas_datum_option = DatumOption::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Decoding datum:",
                &format!("Fragment decode error: {}", e),
            )
        })?;

        Ok(Self {
            inner: pallas_datum_option,
        })
    }
}
