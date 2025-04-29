use std::collections::HashMap;

use pallas_addresses::Address;
use pallas_codec::utils::CborWrap;
use pallas_primitives::{
    conway::{DatumOption, PostAlonzoTransactionOutput, ScriptRef, TransactionOutput, Value},
    AssetName, Coin, NonEmptyKeyValuePairs, PolicyId, PositiveCoin,
};
use uplc::{tx::ResolvedInput, Fragment, Hash, PlutusData, TransactionInput};
use whisky_common::{hex_to_bytes, Asset, UTxO, UtxoOutput, WError};

pub fn to_uplc_utxos(utxos: &Vec<UTxO>) -> Result<Vec<ResolvedInput>, WError> {
    let mut resolved_inputs = Vec::new();
    for utxo in utxos {
        let tx_hash: [u8; 32] = hex::decode(&utxo.input.tx_hash)
            .map_err(|err| {
                WError::new("to_uplc_utxos", &format!("Invalid tx hash found: {}", err))
            })?
            .try_into()
            .map_err(|_e| WError::new("to_uplc_utxos", "Invalid tx hash length found"))?;

        let address_hex = Address::from_bech32(&utxo.output.address)
            .map_err(|err| {
                WError::new(
                    "to_uplc_utxos",
                    &format!("Invalid address found: {:?}", err),
                )
            })?
            .to_hex();

        let output = TransactionOutput::PostAlonzo(PostAlonzoTransactionOutput {
            address: hex_to_bytes(&address_hex)
                .map_err(WError::from_err("to_uplc_utxos"))?
                .to_vec()
                .try_into()
                .map_err(WError::from_err("to_uplc_utxos - into address"))?,
            value: to_uplc_value(&utxo.output.amount).map_err(WError::from_err("to_uplc_utxos"))?,
            datum_option: to_uplc_datum(&utxo.output).map_err(WError::from_err("to_uplc_utxos"))?,
            script_ref: to_uplc_script_ref(&utxo.output.script_ref)
                .map_err(WError::from_err("to_uplc_utxos"))?,
        });

        let resolved_input = ResolvedInput {
            input: TransactionInput {
                transaction_id: Hash::from(tx_hash),
                index: utxo.input.output_index.into(),
            },
            output,
        };
        resolved_inputs.push(resolved_input);
    }
    Ok(resolved_inputs)
}
pub fn to_uplc_value(assets: &Vec<Asset>) -> Result<Value, WError> {
    if assets.len() == 1 {
        match assets[0].unit().as_str() {
            "lovelace" => Ok(Value::Coin(assets[0].quantity().parse::<u64>().unwrap())),
            _ => Err(WError::new("to_uplc_value", "Invalid value")),
        }
    } else {
        to_uplc_multi_asset_value(assets)
    }
}

pub fn to_uplc_multi_asset_value(assets: &Vec<Asset>) -> Result<Value, WError> {
    let mut coins: Coin = 0;
    let mut asset_mapping: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for asset in assets {
        if asset.unit() == "lovelace" || asset.unit().is_empty() {
            coins = asset.quantity().parse::<u64>().unwrap();
        } else {
            let asset_unit = asset.unit();
            let (policy_id, asset_name) = asset_unit.split_at(56);
            asset_mapping
                .entry(policy_id.to_string())
                .or_default()
                .push((asset_name.to_string(), asset.quantity().clone()))
        }
    }

    let mut multi_asset = Vec::new();
    for (policy_id, asset_list) in &asset_mapping {
        let policy_id_bytes: [u8; 28] = hex::decode(policy_id)
            .map_err(WError::from_err(
                "to_uplc_multi_asset_value - Invalid policy id hex",
            ))?
            .try_into()
            .map_err(|_e| {
                WError::new("to_uplc_multi_asset_vale", "Invalid length policy id found")
            })?;

        let policy_id = PolicyId::from(policy_id_bytes);
        let mut mapped_assets = Vec::new();
        for asset in asset_list {
            let (asset_name, asset_quantity) = asset;
            let asset_name_bytes = AssetName::from(hex::decode(asset_name).map_err(
                WError::from_err("to_uplc_multi_asset_value - Invalid asset name hex"),
            )?);
            mapped_assets.push((
                asset_name_bytes,
                PositiveCoin::try_from(asset_quantity.parse::<u64>().unwrap()).unwrap(),
            ));
        }
        multi_asset.push((policy_id, NonEmptyKeyValuePairs::Def(mapped_assets)));
    }
    let uplc_multi_asset = NonEmptyKeyValuePairs::Def(multi_asset);
    Ok(Value::Multiasset(coins, uplc_multi_asset))
}

pub fn to_uplc_script_ref(
    script_ref: &Option<String>,
) -> Result<Option<CborWrap<ScriptRef>>, WError> {
    if let Some(script_ref) = script_ref {
        let script_bytes = hex::decode(script_ref).map_err(WError::from_err(
            "to_uplc_script_ref - Invalid script ref hex",
        ))?;
        let pallas_script = ScriptRef::decode_fragment(&script_bytes).map_err(WError::from_err(
            "to_uplc_script_ref - Invalid script ref bytes",
        ))?;

        Ok(Some(CborWrap(pallas_script)))
    } else {
        Ok(None)
    }
}

pub fn to_uplc_datum(utxo_output: &UtxoOutput) -> Result<Option<DatumOption>, WError> {
    if let Some(inline_datum) = &utxo_output.plutus_data {
        //hex to bytes
        let plutus_data_bytes = hex::decode(inline_datum)
            .map_err(WError::from_err("to_uplc_datum - Invalid plutus data hex"))?;
        let datum = CborWrap(PlutusData::decode_fragment(&plutus_data_bytes).map_err(
            WError::from_err("to_uplc_datum - Invalid plutus data bytes"),
        )?);
        Ok(Some(DatumOption::Data(datum)))
    } else if let Some(datum_hash) = &utxo_output.data_hash {
        let datum_hash_bytes: [u8; 32] = hex::decode(datum_hash)
            .map_err(WError::from_err("to_uplc_datum - Invalid datum hash hex"))?
            .try_into()
            .map_err(|_e| {
                WError::new("to_uplc_datum", "Invalid byte length of datum hash found")
            })?;
        Ok(Some(DatumOption::Hash(Hash::from(datum_hash_bytes))))
    } else {
        Ok(None)
    }
}
