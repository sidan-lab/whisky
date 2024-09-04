use cardano_serialization_lib::{self as csl, JsError};

use crate::model::{Datum, LanguageVersion, Output, OutputScriptSource};

use super::{to_bignum, to_value};

pub fn get_min_utxo_value(output: &Output, coins_per_utxo_size: &u64) -> Result<String, JsError> {
    let mut tx_output_builder = csl::TransactionOutputBuilder::new()
        .with_address(&csl::Address::from_bech32(&output.address)?);
    match &output.datum {
        Some(datum) => match datum {
            Datum::Inline(str_data) => {
                tx_output_builder =
                    tx_output_builder.with_plutus_data(&csl::PlutusData::from_hex(str_data)?);
            }
            Datum::Hash(str_data) => {
                tx_output_builder = tx_output_builder.with_data_hash(&csl::hash_plutus_data(
                    &csl::PlutusData::from_hex(str_data)?,
                ));
            }
            Datum::Embedded(str_data) => {
                tx_output_builder = tx_output_builder.with_data_hash(&csl::hash_plutus_data(
                    &csl::PlutusData::from_hex(str_data)?,
                ));
            }
        },
        None => {}
    }
    match &output.reference_script {
        Some(output_script_source) => match output_script_source {
            OutputScriptSource::ProvidedSimpleScriptSource(simple_script) => {
                tx_output_builder =
                    tx_output_builder.with_script_ref(&csl::ScriptRef::new_native_script(
                        &csl::NativeScript::from_hex(&simple_script.script_cbor)?,
                    ));
            }
            OutputScriptSource::ProvidedScriptSource(script) => {
                let version = match script.language_version {
                    LanguageVersion::V1 => csl::Language::new_plutus_v1(),
                    LanguageVersion::V2 => csl::Language::new_plutus_v2(),
                    LanguageVersion::V3 => csl::Language::new_plutus_v3(),
                };
                tx_output_builder =
                    tx_output_builder.with_script_ref(&csl::ScriptRef::new_plutus_script(
                        &csl::PlutusScript::from_hex_with_version(&script.script_cbor, &version)?,
                    ));
            }
        },
        None => {}
    }
    let multi_asset = match to_value(&output.amount).multiasset() {
        Some(multi_asset) => multi_asset,
        None => csl::MultiAsset::new(),
    };
    let final_output = tx_output_builder
        .next()?
        .with_asset_and_min_required_coin_by_utxo_cost(
            &multi_asset,
            &csl::DataCost::new_coins_per_byte(&to_bignum(*coins_per_utxo_size)),
        )?
        .build()?;
    Ok(final_output.amount().coin().to_str())
}
