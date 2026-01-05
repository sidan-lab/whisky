use cardano_serialization_lib as csl;
use whisky_common::WError;

use super::native_script::proto_to_native_script;
use super::primitives::proto_to_bignum;
use crate::tx_prototype::types::*;

/// Convert AuxiliaryDataPrototype to CSL AuxiliaryData
pub fn proto_to_auxiliary_data(
    aux_data: &AuxiliaryDataPrototype,
) -> Result<csl::AuxiliaryData, WError> {
    let mut result = csl::AuxiliaryData::new();

    // Metadata
    if let Some(metadata) = &aux_data.metadata {
        let general_metadata = proto_to_general_tx_metadata(metadata)?;
        result.set_metadata(&general_metadata);
    }

    // Native scripts
    if let Some(native_scripts) = &aux_data.native_scripts {
        let mut scripts = csl::NativeScripts::new();
        for script in native_scripts {
            scripts.add(&proto_to_native_script(script)?);
        }
        result.set_native_scripts(&scripts);
    }

    // Plutus scripts
    if let Some(plutus_scripts) = &aux_data.plutus_scripts {
        let mut scripts = csl::PlutusScripts::new();
        for script_hex in plutus_scripts {
            let script = csl::PlutusScript::from_hex(script_hex).map_err(WError::from_err(
                "proto_to_auxiliary_data - invalid plutus script hex",
            ))?;
            scripts.add(&script);
        }
        result.set_plutus_scripts(&scripts);
    }

    // Set prefer_alonzo_format
    result.set_prefer_alonzo_format(aux_data.prefer_alonzo_format);

    Ok(result)
}

/// Convert TxMetadataPrototype to CSL GeneralTransactionMetadata
fn proto_to_general_tx_metadata(
    metadata: &TxMetadataPrototype,
) -> Result<csl::GeneralTransactionMetadata, WError> {
    let mut result = csl::GeneralTransactionMetadata::new();

    for (label_str, metadatum) in metadata {
        let label = proto_to_bignum(label_str)?;
        let tx_metadatum = proto_to_tx_metadatum(metadatum)?;
        result.insert(&label, &tx_metadatum);
    }

    Ok(result)
}

/// Convert MetadatumPrototype to CSL TransactionMetadatum
fn proto_to_tx_metadatum(
    metadatum: &MetadatumPrototype,
) -> Result<csl::TransactionMetadatum, WError> {
    match metadatum {
        MetadatumPrototype::Int { value } => {
            let int = csl::Int::from_str(&value.to_string())
                .map_err(WError::from_err("proto_to_tx_metadatum - invalid int"))?;
            Ok(csl::TransactionMetadatum::new_int(&int))
        }
        MetadatumPrototype::Bytes { value } => {
            Ok(csl::TransactionMetadatum::new_bytes(value.clone())
                .map_err(WError::from_err("proto_to_tx_metadatum - invalid bytes"))?)
        }
        MetadatumPrototype::String { value } => {
            Ok(csl::TransactionMetadatum::new_text(value.clone())
                .map_err(WError::from_err("proto_to_tx_metadatum - invalid text"))?)
        }
        MetadatumPrototype::List { value } => {
            let mut list = csl::MetadataList::new();
            for item in value {
                list.add(&proto_to_tx_metadatum(item)?);
            }
            Ok(csl::TransactionMetadatum::new_list(&list))
        }
        MetadatumPrototype::Map { value } => {
            let mut map = csl::MetadataMap::new();
            for (key, val) in value {
                map.insert(&proto_to_tx_metadatum(key)?, &proto_to_tx_metadatum(val)?);
            }
            Ok(csl::TransactionMetadatum::new_map(&map))
        }
    }
}
