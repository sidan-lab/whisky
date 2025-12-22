use cardano_serialization_lib as csl;
use whisky_common::WError;

use super::primitives::proto_to_bignum;
use crate::tx_prototype::types::*;

/// Convert ValuePrototype to CSL Value
pub fn proto_to_value(value: &ValuePrototype) -> Result<csl::Value, WError> {
    let coin = proto_to_bignum(&value.coin)?;
    let mut result = csl::Value::new(&coin);

    if let Some(multiasset) = &value.multiasset {
        let csl_multiasset = proto_to_multiasset(multiasset)?;
        result.set_multiasset(&csl_multiasset);
    }

    Ok(result)
}

/// Convert MultiAssetPrototype to CSL MultiAsset
pub fn proto_to_multiasset(multiasset: &MultiAssetPrototype) -> Result<csl::MultiAsset, WError> {
    let mut result = csl::MultiAsset::new();

    for (policy_id_hex, assets) in multiasset {
        let policy_id = csl::ScriptHash::from_hex(policy_id_hex)
            .map_err(WError::from_err("proto_to_multiasset - invalid policy id"))?;
        let csl_assets = proto_to_assets(assets)?;
        result.insert(&policy_id, &csl_assets);
    }

    Ok(result)
}

/// Convert AssetsPrototype to CSL Assets
pub fn proto_to_assets(assets: &AssetsPrototype) -> Result<csl::Assets, WError> {
    let mut result = csl::Assets::new();

    for (asset_name_hex, amount_str) in assets {
        let asset_name_bytes = hex::decode(asset_name_hex)
            .map_err(|_| WError::new("proto_to_assets", "invalid asset name hex"))?;
        let asset_name = csl::AssetName::new(asset_name_bytes)
            .map_err(WError::from_err("proto_to_assets - invalid asset name"))?;
        let amount = proto_to_bignum(amount_str)?;
        result.insert(&asset_name, &amount);
    }

    Ok(result)
}

/// Convert MintPrototype to CSL Mint
/// MintPrototype is MultiAssetPrototype matching CSL's Mint structure
/// Note: Mint uses Int (can be negative for burning) while MultiAsset uses BigNum
pub fn proto_to_mint(mint: &MintPrototype) -> Result<csl::Mint, WError> {
    let mut result = csl::Mint::new();

    for (policy_id_hex, assets) in mint {
        let policy_id = csl::ScriptHash::from_hex(policy_id_hex)
            .map_err(WError::from_err("proto_to_mint - invalid policy id"))?;

        let mut mint_assets = csl::MintAssets::new();
        for (asset_name_hex, amount_str) in assets {
            let asset_name_bytes = hex::decode(asset_name_hex)
                .map_err(|_| WError::new("proto_to_mint", "invalid asset name hex"))?;
            let asset_name = csl::AssetName::new(asset_name_bytes)
                .map_err(WError::from_err("proto_to_mint - invalid asset name"))?;
            // Mint uses Int (can be negative) instead of BigNum
            let amount = csl::Int::from_str(amount_str)
                .map_err(WError::from_err("proto_to_mint - invalid mint amount"))?;
            mint_assets
                .insert(&asset_name, &amount)
                .map_err(WError::from_err("proto_to_mint - failed to insert"))?;
        }
        result.insert(&policy_id, &mint_assets);
    }

    Ok(result)
}
