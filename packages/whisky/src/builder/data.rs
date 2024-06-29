use crate::*;
use cardano_serialization_lib::JsError;
use model::Budget;

pub enum WData {
    JSON(String),
    CBOR(String),
}

impl WData {
    pub fn to_cbor(&self) -> Result<String, JsError> {
        match self {
            WData::CBOR(data) => Ok(data.clone()),
            WData::JSON(data) => {
                let data_cbor =
                    &csl::PlutusData::from_json(data, csl::PlutusDatumSchema::DetailedSchema)?
                        .to_hex();
                Ok(data_cbor.clone())
            }
        }
    }
}

pub struct WRedeemer {
    pub data: WData,
    pub ex_units: Budget,
}

pub struct WDatum {
    pub type_: String,
    pub data: WData,
}
