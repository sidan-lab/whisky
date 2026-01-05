use cardano_serialization_lib as csl;
use whisky_common::WError;

use crate::tx_prototype::types::*;

/// Convert PlutusDataVariant to CSL PlutusData
/// This handles both CBOR hex and manual construction
pub fn proto_to_plutus_data_from_variant(
    variant: &PlutusDataVariant,
) -> Result<csl::PlutusData, WError> {
    match variant {
        PlutusDataVariant::Cbor { hex } => csl::PlutusData::from_hex(hex).map_err(
            WError::from_err("proto_to_plutus_data_from_variant - invalid CBOR hex"),
        ),
        PlutusDataVariant::Manual { data } => proto_to_plutus_data(data),
    }
}

/// Convert PlutusData to CSL PlutusData (recursive)
pub fn proto_to_plutus_data(data: &PlutusData) -> Result<csl::PlutusData, WError> {
    match data {
        PlutusData::Integer { value } => {
            let big_int = csl::BigInt::from_str(&value.to_string())
                .map_err(WError::from_err("proto_to_plutus_data - invalid integer"))?;
            Ok(csl::PlutusData::new_integer(&big_int))
        }
        PlutusData::Bytes { value } => {
            let bytes = hex::decode(value)
                .map_err(|_| WError::new("proto_to_plutus_data", "invalid bytes hex string"))?;
            Ok(csl::PlutusData::new_bytes(bytes))
        }
        PlutusData::List { value } => {
            let mut list = csl::PlutusList::new();
            for item in value {
                list.add(&proto_to_plutus_data(item)?);
            }
            Ok(csl::PlutusData::new_list(&list))
        }
        PlutusData::Map { value } => {
            let mut map = csl::PlutusMap::new();
            for (key, val) in value {
                let key_data = proto_to_plutus_data(key)?;
                let value_data = proto_to_plutus_data(val)?;
                // PlutusMap uses PlutusMapValues for the value side
                let mut values = csl::PlutusMapValues::new();
                values.add(&value_data);
                map.insert(&key_data, &values);
            }
            Ok(csl::PlutusData::new_map(&map))
        }
        PlutusData::Constr {
            alternative,
            fields,
        } => {
            let alt = csl::BigNum::from_str(&alternative.to_string()).map_err(WError::from_err(
                "proto_to_plutus_data - invalid alternative",
            ))?;
            let mut plutus_fields = csl::PlutusList::new();
            for field in fields {
                plutus_fields.add(&proto_to_plutus_data(field)?);
            }
            Ok(csl::PlutusData::new_constr_plutus_data(
                &csl::ConstrPlutusData::new(&alt, &plutus_fields),
            ))
        }
    }
}

/// Convert DataOptionPrototype to CSL data representation
/// Returns (Option<DataHash>, Option<PlutusData>) for output usage
pub fn proto_to_data_option(
    data_option: &DataOptionPrototype,
) -> Result<(Option<csl::DataHash>, Option<csl::PlutusData>), WError> {
    match data_option {
        DataOptionPrototype::DataHash { value: hash } => {
            let data_hash = csl::DataHash::from_hex(hash)
                .map_err(WError::from_err("proto_to_data_option - invalid data hash"))?;
            Ok((Some(data_hash), None))
        }
        DataOptionPrototype::Data { value: variant } => {
            let plutus_data = proto_to_plutus_data_from_variant(variant)?;
            Ok((None, Some(plutus_data)))
        }
    }
}
