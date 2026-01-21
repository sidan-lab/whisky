use std::collections::BTreeMap;

use pallas::ledger::primitives::{conway::Tx, Metadatum};
use whisky_common::{Metadata, WError};

pub fn extract_metadata(pallas_tx: &Tx) -> Result<Vec<Metadata>, WError> {
    let mut metadata_list: Vec<Metadata> = vec![];
    match &pallas_tx.auxiliary_data {
        pallas::codec::utils::Nullable::Some(aux_data) => match aux_data.clone().unwrap() {
            pallas::ledger::primitives::conway::AuxiliaryData::Shelley(metadata_map) => {
                for (key, metadatum) in metadata_map.iter() {
                    metadata_list.push(Metadata {
                        tag: key.to_string(),
                        metadata: serde_json::to_string(&metadata_to_json_value(metadatum)?)
                            .map_err(|e| {
                                WError::new(
                                    "WhiskyPallas Parser - ",
                                    &format!("metadata to json string error: {:?}", e),
                                )
                            })?,
                    })
                }
            }
            pallas::ledger::primitives::conway::AuxiliaryData::ShelleyMa(
                shelley_ma_auxiliary_data,
            ) => {
                let metadata_map: &BTreeMap<u64, Metadatum> =
                    &shelley_ma_auxiliary_data.transaction_metadata;
                for (key, metadatum) in metadata_map.iter() {
                    metadata_list.push(Metadata {
                        tag: key.to_string(),
                        metadata: serde_json::to_string(&metadata_to_json_value(metadatum)?)
                            .map_err(|e| {
                                WError::new(
                                    "WhiskyPallas Parser - ",
                                    &format!("metadata to json string error: {:?}", e),
                                )
                            })?,
                    })
                }
            }
            pallas::ledger::primitives::conway::AuxiliaryData::PostAlonzo(
                post_alonzo_auxiliary_data,
            ) => {
                if let Some(metadata_map) = &post_alonzo_auxiliary_data.metadata {
                    for (key, metadatum) in metadata_map.iter() {
                        metadata_list.push(Metadata {
                            tag: key.to_string(),
                            metadata: serde_json::to_string(&metadata_to_json_value(metadatum)?)
                                .map_err(|e| {
                                    WError::new(
                                        "WhiskyPallas Parser - ",
                                        &format!("metadata to json string error: {:?}", e),
                                    )
                                })?,
                        })
                    }
                }
            }
        },
        pallas::codec::utils::Nullable::Null => {}
        pallas::codec::utils::Nullable::Undefined => {}
    }
    Ok(metadata_list)
}

fn metadata_to_json_value(metadatum: &Metadatum) -> Result<serde_json::Value, WError> {
    match metadatum {
        Metadatum::Int(int) => {
            let int_value = i128::try_from(int.clone()).map_err(|_| {
                WError::new(
                    "WhiskyPallas Parser - ",
                    "metadata to json value, invalid tag",
                )
            })?;
            let int_i64 = i64::try_from(int_value).map_err(|_| {
                WError::new(
                    "WhiskyPallas Parser - ",
                    "metadata to json value, i128 out of i64 range",
                )
            })?;
            Ok(serde_json::Value::Number(serde_json::Number::from(int_i64)))
        }
        Metadatum::Bytes(bytes) => Ok(serde_json::Value::String(bytes.to_string())),
        Metadatum::Text(string) => Ok(serde_json::Value::String(string.clone())),
        Metadatum::Array(metadatums) => {
            let mut json_array: Vec<serde_json::Value> = Vec::new();
            for metadatum in metadatums {
                let json_value = metadata_to_json_value(metadatum)?;
                json_array.push(json_value);
            }
            Ok(serde_json::Value::Array(json_array))
        }
        Metadatum::Map(key_value_pairs) => {
            let mut json_map = serde_json::Map::new();
            for (key, value) in key_value_pairs.iter() {
                let json_key = match metadata_to_json_value(key)? {
                    serde_json::Value::String(s) => s,
                    other => other.to_string(),
                };
                let json_value = metadata_to_json_value(value)?;
                json_map.insert(json_key, json_value);
            }
            Ok(serde_json::Value::Object(json_map))
        }
    }
}
