use pallas::{
    codec::utils::{CborWrap, KeepRaw},
    ledger::primitives::{conway::DatumOption, Fragment, PlutusData as PallasPlutusData},
};

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
    pub fn new(datum_option_kind: DatumKind) -> Result<Self, String> {
        let pallas_datum_option = match datum_option_kind {
            DatumKind::Hash { datum_hash } => {
                let datum_hash = datum_hash.parse().expect("Invalid datum hash length");
                DatumOption::Hash(datum_hash)
            }

            DatumKind::Data { plutus_data_hex } => {
                let bytes =
                    hex::decode(plutus_data_hex).map_err(|e| format!("Hex decode error: {}", e))?;
                PallasPlutusData::decode_fragment(&bytes)
                    .map(|e| DatumOption::Data(CborWrap(KeepRaw::from(e))))
                    .map_err(|e| format!("Fragment decode error: {}", e))?
            }
        };

        Ok(Self {
            inner: pallas_datum_option,
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.inner.encode_fragment().unwrap())
    }

    pub fn decode(bytes: &'a [u8]) -> Result<Self, String> {
        let pallas_datum_option = DatumOption::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e))?;

        Ok(Self {
            inner: pallas_datum_option,
        })
    }
}
