use cardano_serialization_lib::{self as csl};

use super::{to_bignum, to_value};
use whisky_common::*;

pub fn get_min_utxo_value(output: &Output, coins_per_utxo_size: &u64) -> Result<String, WError> {
    let mut tx_output_builder = csl::TransactionOutputBuilder::new().with_address(
        &csl::Address::from_bech32(&output.address).map_err(WError::from_err(
            "get_min_utxo_value - invalid address bech32",
        ))?,
    );
    if let Some(datum) = &output.datum {
        match datum {
            Datum::Inline(str_data) => {
                tx_output_builder = tx_output_builder.with_plutus_data(
                    &csl::PlutusData::from_hex(str_data).map_err(WError::from_err(
                        "get_min_utxo_value - invalid inline datum hex",
                    ))?,
                );
            }
            Datum::Hash(str_data) => {
                tx_output_builder = tx_output_builder.with_data_hash(&csl::hash_plutus_data(
                    &csl::PlutusData::from_hex(str_data).map_err(WError::from_err(
                        "get_min_utxo_value - invalid hash datum hex",
                    ))?,
                ));
            }
            Datum::Embedded(str_data) => {
                tx_output_builder = tx_output_builder.with_data_hash(&csl::hash_plutus_data(
                    &csl::PlutusData::from_hex(str_data).map_err(WError::from_err(
                        "get_min_utxo_value - invalid embedded datum hex",
                    ))?,
                ));
            }
        }
    }
    if let Some(output_script_source) = &output.reference_script {
        match output_script_source {
            OutputScriptSource::ProvidedSimpleScriptSource(simple_script) => {
                tx_output_builder =
                    tx_output_builder.with_script_ref(&csl::ScriptRef::new_native_script(
                        &csl::NativeScript::from_hex(&simple_script.script_cbor).map_err(
                            WError::from_err(
                                "get_min_utxo_value - invalid provided simple script hex",
                            ),
                        )?,
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
                        &csl::PlutusScript::from_hex_with_version(&script.script_cbor, &version)
                            .map_err(WError::from_err(
                                "get_min_utxo_value - invalid provided script hex",
                            ))?,
                    ));
            }
        }
    }
    let multi_asset = match to_value(&output.amount)?.multiasset() {
        Some(multi_asset) => multi_asset,
        None => csl::MultiAsset::new(),
    };
    let final_output = tx_output_builder
        .next()
        .map_err(WError::from_err(
            "get_min_utxo_value - invalid next output builder",
        ))?
        .with_asset_and_min_required_coin_by_utxo_cost(
            &multi_asset,
            &csl::DataCost::new_coins_per_byte(&to_bignum(*coins_per_utxo_size).map_err(
                WError::add_err_trace("get_min_utxo_value - invalid coins per utxo size"),
            )?),
        )
        .map_err(WError::from_err(
            "get_min_utxo_value - invalid min required coin by utxo cost",
        ))?
        .build()
        .map_err(WError::from_err(
            "get_min_utxo_value - invalid build tx output",
        ))?;
    Ok(final_output.amount().coin().to_str())
}
