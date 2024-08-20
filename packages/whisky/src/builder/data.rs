use crate::*;
use csl::JsError;
use model::Budget;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct WRedeemer {
    pub data: WData,
    pub ex_units: Budget,
}

#[derive(Clone, Debug)]
pub struct WDatum {
    pub type_: String,
    pub data: WData,
}
