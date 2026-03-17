use crate::converter::{bytes_from_bech32, convert_value};
use whisky_common::{LanguageVersion, Output, WError};

use crate::wrapper::transaction_body::{
    Datum, DatumKind, ScriptRef, ScriptRefKind, TransactionOutput,
};

pub fn convert_output(output: &Output) -> Result<TransactionOutput, WError> {
    let datum: Option<Datum> = match &output.datum {
        Some(datum_source) => match datum_source {
            whisky_common::Datum::Inline(datum_str) => Some(Datum::new(DatumKind::Data {
                plutus_data_hex: datum_str.to_string(),
            })?),
            whisky_common::Datum::Hash(datum_str) => {
                let datum = Datum::new(DatumKind::Data {
                    plutus_data_hex: datum_str.to_string(),
                })?;

                let datum_hash_str = datum.hash()?;
                Some(Datum::new(DatumKind::Hash {
                    datum_hash: datum_hash_str,
                })?)
            }
            whisky_common::Datum::Embedded(datum_str) => None,
        },
        None => None,
    };

    let script_ref = match &output.reference_script {
        Some(script_source) => match script_source {
            whisky_common::OutputScriptSource::ProvidedScriptSource(provided_script_source) => {
                let plutus_script = match provided_script_source.language_version {
                    LanguageVersion::V1 => ScriptRef::new(ScriptRefKind::PlutusV1Script {
                        plutus_v1_script_hex: provided_script_source.script_cbor.clone(),
                    })?,
                    LanguageVersion::V2 => ScriptRef::new(ScriptRefKind::PlutusV2Script {
                        plutus_v2_script_hex: provided_script_source.script_cbor.clone(),
                    })?,
                    LanguageVersion::V3 => ScriptRef::new(ScriptRefKind::PlutusV3Script {
                        plutus_v3_script_hex: provided_script_source.script_cbor.clone(),
                    })?,
                };
                Some(plutus_script)
            }
            whisky_common::OutputScriptSource::ProvidedSimpleScriptSource(
                provided_simple_script_source,
            ) => {
                let native_script = ScriptRef::new(ScriptRefKind::NativeScript {
                    native_script_hex: provided_simple_script_source.script_cbor.clone(),
                })?;
                Some(native_script)
            }
        },
        None => None,
    };

    let tx_output = TransactionOutput::new(
        &bytes_from_bech32(&output.address)?,
        convert_value(&output.amount.clone())?,
        datum,
        script_ref,
    )?;

    Ok(tx_output)
}
