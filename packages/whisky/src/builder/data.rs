use crate::*;

#[derive(Clone, Debug)]
pub enum WData {
    JSON(String),
    CBOR(String),
}

impl WData {
    pub fn to_cbor(&self) -> Result<String, WError> {
        match self {
            WData::CBOR(data) => Ok(data.clone()),
            WData::JSON(data) => {
                let data_cbor =
                    &csl::PlutusData::from_json(data, csl::PlutusDatumSchema::DetailedSchema)
                        .map_err(WError::from_err("WData - to_cbor"))?
                        .to_hex();
                Ok(data_cbor.clone())
            }
        }
    }

    pub fn to_hash(&self) -> Result<String, WError> {
        let cbor = self.to_cbor()?;
        let hash = &csl::hash_plutus_data(
            &csl::PlutusData::from_hex(&cbor).map_err(WError::from_err("WData - to_hash"))?,
        )
        .to_hex();
        Ok(hash.to_string())
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
