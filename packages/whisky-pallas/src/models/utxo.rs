use pallas_codec::utils::{CborWrap, PositiveCoin};
use pallas_primitives::conway::Multiasset;
use pallas_primitives::{
    conway::{AssetName, Coin, DatumOption, PlutusData, PolicyId, ScriptRef, Value},
    Fragment,
};
use pallas_primitives::{Hash, KeepRaw};
use std::collections::{BTreeMap, HashMap};
use whisky_common::{self as wc, *};

pub fn to_pallas_script_ref<'a>(
    script_ref: &Option<String>,
) -> Result<Option<CborWrap<ScriptRef<'a>>>, WError> {
    if let Some(script_ref) = script_ref {
        let script_bytes = hex::decode(script_ref).map_err(WError::from_err(
            "to_pallas_script_ref - Invalid script ref hex",
        ))?;
        let script_bytes_ref: &'a [u8] = Box::leak(script_bytes.into_boxed_slice());
        let pallas_script = ScriptRef::decode_fragment(script_bytes_ref).map_err(
            WError::from_err("to_pallas_script_ref - Invalid script ref bytes"),
        )?;

        Ok(Some(CborWrap(pallas_script)))
    } else {
        Ok(None)
    }
}

pub fn to_pallas_datum(utxo_output: &UtxoOutput) -> Result<Option<DatumOption>, WError> {
    if let Some(inline_datum) = &utxo_output.plutus_data {
        //hex to bytes
        let plutus_data_bytes = hex::decode(inline_datum).map_err(WError::from_err(
            "to_pallas_datum - Invalid plutus data hex",
        ))?;
        let datum = CborWrap(KeepRaw::from(
            PlutusData::decode_fragment(&plutus_data_bytes).map_err(WError::from_err(
                "to_pallas_datum - Invalid plutus data bytes",
            ))?,
        ));
        Ok(Some(DatumOption::Data(datum)))
    } else if let Some(datum_hash) = &utxo_output.data_hash {
        let datum_hash_bytes: [u8; 32] = hex::decode(datum_hash)
            .map_err(WError::from_err("to_pallas_datum - Invalid datum hash hex"))?
            .try_into()
            .map_err(|_e| {
                WError::new("to_pallas_datum", "Invalid byte length of datum hash found")
            })?;
        Ok(Some(DatumOption::Hash(Hash::from(datum_hash_bytes))))
    } else {
        Ok(None)
    }
}

pub fn to_pallas_value(assets: &Vec<Asset>) -> Result<Value, WError> {
    if assets.len() == 1 {
        match assets[0].unit().as_str() {
            "lovelace" => Ok(Value::Coin(assets[0].quantity().parse::<u64>().unwrap())),
            _ => Err(WError::new("to_pallas_value", "Invalid value")),
        }
    } else {
        to_pallas_multi_asset_value(assets)
    }
}

pub fn to_pallas_multi_asset_value(assets: &Vec<Asset>) -> Result<Value, WError> {
    let assets = wc::Value::from_asset_vec(assets).to_asset_vec();
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

    let mut multi_asset: Multiasset<PositiveCoin> = BTreeMap::new();
    for (policy_id, asset_list) in &asset_mapping {
        let policy_id_bytes: [u8; 28] = hex::decode(policy_id)
            .map_err(WError::from_err(
                "to_pallas_multi_asset_value - Invalid policy id hex",
            ))?
            .try_into()
            .map_err(|_e| {
                WError::new(
                    "to_pallas_multi_asset_vale",
                    "Invalid length policy id found",
                )
            })?;

        let policy_id = PolicyId::from(policy_id_bytes);
        let mut mapped_assets = BTreeMap::new();
        for asset in asset_list {
            let (asset_name, asset_quantity) = asset;
            let asset_name_bytes = AssetName::from(hex_to_bytes(asset_name).map_err(
                WError::from_err("to_pallas_multi_asset_value - Invalid asset name hex"),
            )?);
            mapped_assets.insert(
                asset_name_bytes,
                PositiveCoin::try_from(asset_quantity.parse::<u64>().unwrap()).unwrap(),
            );
        }
        multi_asset.insert(policy_id, mapped_assets);
    }
    Ok(Value::Multiasset(coins, multi_asset))
}
