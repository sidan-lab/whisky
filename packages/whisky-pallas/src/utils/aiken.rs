use pallas_primitives::PlutusScript;
use uplc::{Fragment, PlutusData};
use whisky_common::{BuilderDataType, WError};

use crate::utils::encode_json_str_to_plutus_datum;

pub fn apply_double_cbor_encoding(script: &str) -> Result<String, WError> {
    let bytes: Vec<u8> = hex::decode(script).map_err(|e| {
        WError::new(
            "apply_double_cbor_encoding - invalid script bytes",
            &format!("Hex decode error: {}", e.to_string()),
        )
    })?;

    let single_encoded_script =
        PlutusScript::<3>::decode_fragment(&bytes.clone()).map_err(|e| {
            WError::new(
                "apply_double_cbor_encoding - invalid script bytes",
                &format!("PlutusScript decode error: {}", e.to_string()),
            )
        })?;

    let encoded_bytes = single_encoded_script.encode_fragment().map_err(|e| {
        WError::new(
            "apply_double_cbor_encoding - invalid script bytes",
            &format!("PlutusScript encode error: {}", e.to_string()),
        )
    })?;

    Ok(hex::encode(encoded_bytes))
}

pub fn apply_params_to_script(
    plutus_script: &str,
    params_to_apply: &[&str],
    param_type: BuilderDataType,
) -> Result<String, WError> {
    let double_encoded_script = apply_double_cbor_encoding(plutus_script)?;
    let plutus_script =
        PlutusScript::<3>::decode_fragment(&hex::decode(&double_encoded_script).map_err(|e| {
            WError::new(
                "apply_params_to_script - invalid script bytes",
                &format!("Hex decode error: {}", e.to_string()),
            )
        })?)
        .map_err(|e| {
            WError::new(
                "apply_params_to_script - invalid script bytes",
                &format!("PlutusScript decode error: {}", e.to_string()),
            )
        })?;

    let mut plutus_list: Vec<PlutusData> = vec![];
    for param in params_to_apply {
        match param_type {
            BuilderDataType::JSON => {
                let plutus_data = encode_json_str_to_plutus_datum(param).map_err(|e| {
                    WError::new(
                        "apply_params_to_script - invalid parameter",
                        &format!("JSON to PlutusData error: {}", e.to_string()),
                    )
                })?;
                plutus_list.push(plutus_data);
            }
            BuilderDataType::CBOR => {
                let plutus_data =
                    PlutusData::decode_fragment(&hex::decode(param).map_err(|e| {
                        WError::new(
                            "apply_params_to_script - invalid parameter",
                            &format!("Hex decode error: {}", e.to_string()),
                        )
                    })?)
                    .map_err(|e| {
                        WError::new(
                            "apply_params_to_script - invalid parameter",
                            &format!("PlutusData decode error: {}", e.to_string()),
                        )
                    })?;
                plutus_list.push(plutus_data);
            }
        }
    }

    let bytes = uplc::tx::apply_params_to_script(
        &plutus_list.encode_fragment().map_err(|e| {
            WError::new(
                "apply_params_to_script - invalid parameters",
                &format!("PlutusData encode error: {}", e.to_string()),
            )
        })?,
        &plutus_script.encode_fragment().map_err(|e| {
            WError::new(
                "apply_params_to_script - invalid script bytes",
                &format!("PlutusScript encode error: {}", e.to_string()),
            )
        })?,
    )
    .map_err(|e| {
        WError::new(
            "apply_params_to_script - applying parameters to script failed",
            &format!("Apply params error: {}", e.to_string()),
        )
    })?;

    Ok(hex::encode(bytes))
}
