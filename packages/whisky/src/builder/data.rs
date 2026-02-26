use pallas_primitives::{
    conway::{DatumOption, PseudoDatumOption},
    MaybeIndefArray, PlutusData,
};
use pallas_traverse::ComputeHash;
use serde_json::json;
use uplc::{Constr, Fragment};
use whisky_pallas::utils::encode_json_str_to_plutus_datum;

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
                let data_cbor = bytes_to_hex(
                    &encode_json_str_to_plutus_datum(data)?
                        .encode_fragment()
                        .map_err(WError::from_err("WData - to_cbor"))?,
                );
                Ok(data_cbor.clone())
            }
        }
    }

    pub fn to_hash(&self) -> Result<String, WError> {
        let cbor = self.to_cbor()?;
        let decoded_hex =
            hex::decode(cbor).map_err(WError::from_err("WData - to_hash - hex decode"))?;
        let plutus_data = PlutusData::decode_fragment(&decoded_hex)
            .map_err(|_| WError::new("WData to_hash", "error decoding cbor"))?;
        let hash = DatumOption::compute_hash(&PseudoDatumOption::Data(
            pallas_codec::utils::CborWrap(plutus_data),
        ));
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

#[test]
fn test_wdata_to_cbor() {
    let cbor = &WData::JSON(
        json!({
            "constructor": 0,
            "fields": []
        })
        .to_string(),
    )
    .to_cbor()
    .unwrap();

    let cbor2 = PlutusData::Constr(Constr {
        tag: 121,
        any_constructor: None,
        fields: MaybeIndefArray::Def(vec![]),
    });
    println!("CBOR: {}", cbor);
    println!("CBOR2: {}", hex::encode(cbor2.encode_fragment().unwrap()));
}
