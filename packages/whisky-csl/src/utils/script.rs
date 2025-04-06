use cardano_serialization_lib::{self as csl};
use whisky_common::*;

pub fn get_native_script_hash(script: &str) -> Result<String, WError> {
    let script_hash = csl::NativeScript::from_hex(script)
        .map_err(WError::from_err(
            "get_native_script_hash - invalid native script hex",
        ))?
        .hash()
        .to_hex();
    Ok(script_hash)
}

pub fn get_script_hash(script: &str, version: LanguageVersion) -> Result<String, WError> {
    let language_version = match version {
        LanguageVersion::V1 => csl::Language::new_plutus_v1(),
        LanguageVersion::V2 => csl::Language::new_plutus_v2(),
        LanguageVersion::V3 => csl::Language::new_plutus_v3(),
    };
    let script_hash = csl::PlutusScript::from_hex_with_version(script, &language_version)
        .map_err(WError::from_err("get_script_hash - invalid script hex"))?
        .hash()
        .to_hex();
    Ok(script_hash)
}

pub fn to_csl_script_source(
    script_source: ScriptSource,
) -> Result<csl::PlutusScriptSource, WError> {
    match script_source {
        ScriptSource::InlineScriptSource(script) => {
            let language_version: csl::Language = match script.language_version {
                LanguageVersion::V1 => csl::Language::new_plutus_v1(),
                LanguageVersion::V2 => csl::Language::new_plutus_v2(),
                LanguageVersion::V3 => csl::Language::new_plutus_v3(),
            };
            Ok(csl::PlutusScriptSource::new_ref_input(
                &csl::ScriptHash::from_hex(&script.script_hash).map_err(WError::from_err(
                    "to_csl_script_source - invalid script hash",
                ))?,
                &csl::TransactionInput::new(
                    &csl::TransactionHash::from_hex(&script.ref_tx_in.tx_hash).map_err(
                        WError::from_err("to_csl_script_source - invalid transaction hash"),
                    )?,
                    script.ref_tx_in.tx_index,
                ),
                &language_version,
                script.script_size,
            ))
        }
        ScriptSource::ProvidedScriptSource(script) => {
            let language_version: csl::Language = match script.language_version {
                LanguageVersion::V1 => csl::Language::new_plutus_v1(),
                LanguageVersion::V2 => csl::Language::new_plutus_v2(),
                LanguageVersion::V3 => csl::Language::new_plutus_v3(),
            };
            Ok(csl::PlutusScriptSource::new(
                &csl::PlutusScript::from_hex_with_version(
                    script.script_cbor.as_str(),
                    &language_version,
                )
                .map_err(WError::from_err(
                    "to_csl_script_source - invalid script cbor",
                ))?,
            ))
        }
    }
}

pub fn to_csl_simple_script_source(
    simple_script_source: SimpleScriptSource,
) -> Result<csl::NativeScriptSource, WError> {
    match simple_script_source {
        SimpleScriptSource::ProvidedSimpleScriptSource(script) => Ok(csl::NativeScriptSource::new(
            &csl::NativeScript::from_hex(&script.script_cbor).map_err(WError::from_err(
                "to_csl_simple_script_source - invalid script cbor",
            ))?,
        )),

        SimpleScriptSource::InlineSimpleScriptSource(script) => {
            Ok(csl::NativeScriptSource::new_ref_input(
                &csl::ScriptHash::from_hex(&script.simple_script_hash).map_err(
                    WError::from_err("to_csl_simple_script_source - invalid script hash"),
                )?,
                &csl::TransactionInput::new(
                    &csl::TransactionHash::from_hex(&script.ref_tx_in.tx_hash).map_err(
                        WError::from_err("to_csl_simple_script_source - invalid transaction hash"),
                    )?,
                    script.ref_tx_in.tx_index,
                ),
                script.script_size,
            ))
        }
    }
}
