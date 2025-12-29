use std::{collections::BTreeMap, str::FromStr};

use crate::wrapper::witness_set::{native_script::NativeScript, plutus_script::PlutusScript};

use pallas::{
    codec::utils::{Bytes, Int},
    ledger::primitives::{
        alonzo::{
            PostAlonzoAuxiliaryData as PallasPostAlonzoAuxiliaryData,
            ShelleyMaAuxiliaryData as PallasShelleyMaAuxiliaryData,
        },
        conway::{
            AuxiliaryData as PallasAuxiliaryData, Metadata as PallasMetadata,
            Metadatum as PallasMetadum,
        },
        Fragment, KeyValuePairs,
    },
};

pub type MetadatumLabel = u64;

pub type Metadata = BTreeMap<MetadatumLabel, Metadatum>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Metadatum {
    Int(i64),
    Bytes(String),
    Text(String),
    Array(Vec<Metadatum>),
    Map(Vec<(Metadatum, Metadatum)>),
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct PostAlonzoAuxiliaryData {
    pub metadata: Option<Metadata>,
    pub native_scripts: Option<Vec<NativeScript>>,
    pub plutus_scripts: Option<Vec<PlutusScript<1>>>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct ShelleyMaAuxiliaryData {
    pub transaction_metadata: Metadata,
    pub auxiliary_scripts: Option<Vec<NativeScript>>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum AuxiliaryDataKind {
    Shelley(Metadata),
    ShelleyMa(ShelleyMaAuxiliaryData),
    PostAlonzo(PostAlonzoAuxiliaryData),
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct AuxiliaryData {
    pub inner: PallasAuxiliaryData,
}

impl AuxiliaryData {
    pub fn new(kind: AuxiliaryDataKind) -> Result<Self, String> {
        let inner: PallasAuxiliaryData = match kind {
            AuxiliaryDataKind::Shelley(metadata) => {
                PallasAuxiliaryData::Shelley(Self::to_pallas_metadatum_map(metadata)?)
            }
            AuxiliaryDataKind::ShelleyMa(shelley_ma_aux_data) => {
                PallasAuxiliaryData::ShelleyMa(PallasShelleyMaAuxiliaryData {
                    transaction_metadata: Self::to_pallas_metadatum_map(
                        shelley_ma_aux_data.transaction_metadata,
                    )?,
                    auxiliary_scripts: match shelley_ma_aux_data.auxiliary_scripts {
                        Some(scripts) => Some(scripts.into_iter().map(|s| s.inner).collect()),
                        None => None,
                    },
                })
            }
            AuxiliaryDataKind::PostAlonzo(post_alonzo_aux_data) => {
                PallasAuxiliaryData::PostAlonzo(PallasPostAlonzoAuxiliaryData {
                    metadata: match post_alonzo_aux_data.metadata {
                        Some(meta) => Some(Self::to_pallas_metadatum_map(meta)?),
                        None => None,
                    },
                    native_scripts: match post_alonzo_aux_data.native_scripts {
                        Some(scripts) => Some(scripts.into_iter().map(|s| s.inner).collect()),
                        None => None,
                    },
                    plutus_scripts: match post_alonzo_aux_data.plutus_scripts {
                        Some(scripts) => Some(scripts.into_iter().map(|s| s.inner).collect()),
                        None => None,
                    },
                })
            }
        };

        Ok(Self { inner })
    }

    fn to_pallas_metadatum_map(metadata: Metadata) -> Result<PallasMetadata, String> {
        let mut pallas_metadata_map: BTreeMap<u64, PallasMetadum> = BTreeMap::new();
        for (key, value) in metadata {
            let pallas_value = Self::to_pallas_metadatum(value)
                .map_err(|e| format!("Invalid metadatum during serialization: {}", e))?;
            pallas_metadata_map.insert(key, pallas_value);
        }
        Ok(PallasMetadata::from(pallas_metadata_map))
    }

    fn to_pallas_metadatum(metadatum: Metadatum) -> Result<PallasMetadum, String> {
        match metadatum {
            Metadatum::Int(i) => Ok(PallasMetadum::Int(Int::from(i))),
            Metadatum::Bytes(b) => {
                Ok(PallasMetadum::Bytes(Bytes::from_str(&b).map_err(|e| {
                    format!("Invalid bytes in metadatum: {}", e)
                })?))
            }
            Metadatum::Text(t) => Ok(PallasMetadum::Text(t)),
            Metadatum::Array(arr) => {
                let mut pallas_array = Vec::new();
                for item in arr {
                    pallas_array.push(Self::to_pallas_metadatum(item)?);
                }
                Ok(PallasMetadum::Array(pallas_array))
            }
            Metadatum::Map(map) => {
                let mut pallas_key_values = Vec::new();
                for (key, value) in map {
                    let pallas_key = Self::to_pallas_metadatum(key)?;
                    let pallas_value = Self::to_pallas_metadatum(value)?;
                    pallas_key_values.push((pallas_key, pallas_value));
                }
                Ok(PallasMetadum::Map(KeyValuePairs::from(pallas_key_values)))
            }
        }
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at AuxiliaryData"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasAuxiliaryData::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
