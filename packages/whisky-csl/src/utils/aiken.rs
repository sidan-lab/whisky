use cardano_serialization_lib::{self as csl};

use whisky_common::*;

pub fn apply_double_cbor_encoding(script: &str) -> Result<String, WError> {
    let bytes: Vec<u8> = hex_to_bytes(script).unwrap();

    let single_encoded_script = csl::PlutusScript::from_bytes(bytes.clone()).map_err(
        WError::from_err("apply_double_cbor_encoding - invalid script bytes"),
    )?;

    match csl::PlutusScript::from_bytes(single_encoded_script.bytes()) {
        Ok(_) => Ok(script.to_string()),
        Err(_) => {
            let bytes = csl::PlutusScript::new(bytes).to_bytes();
            let new_script = bytes_to_hex(&bytes);
            Ok(new_script)
        }
    }
}

pub fn apply_params_to_script(
    plutus_script: &str,
    params_to_apply: &[&str],
    param_type: BuilderDataType,
) -> Result<String, WError> {
    let double_encoded_script = apply_double_cbor_encoding(plutus_script).unwrap();
    let plutus_script =
        csl::PlutusScript::from_bytes(hex_to_bytes(&double_encoded_script).unwrap()).unwrap();
    let mut plutus_list = csl::PlutusList::new();
    for param in params_to_apply {
        match param_type {
            BuilderDataType::JSON => {
                let plutus_data =
                    csl::PlutusData::from_json(param, csl::PlutusDatumSchema::DetailedSchema)
                        .unwrap();
                plutus_list.add(&plutus_data);
            }
            BuilderDataType::CBOR => {
                let plutus_data = csl::PlutusData::from_hex(param).unwrap();
                plutus_list.add(&plutus_data);
            }
        }
    }
    let bytes = apply_params_to_plutus_script(&plutus_list, plutus_script)?.to_bytes();
    Ok(bytes_to_hex(&bytes))
}

pub fn apply_params_to_plutus_script(
    params: &csl::PlutusList,
    plutus_script: csl::PlutusScript,
) -> Result<csl::PlutusScript, WError> {
    let bytes = uplc::tx::apply_params_to_script(&params.to_bytes(), &plutus_script.bytes())
        .map_err(WError::from_err(
            "apply_params_to_plutus_script - invalid script bytes",
        ))?;
    Ok(csl::PlutusScript::new(bytes))
}
