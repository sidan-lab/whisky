use std::{collections::BTreeMap, str::FromStr};

use pallas::{
    codec::utils::{NonZeroInt, PositiveCoin},
    ledger::primitives::{conway::Multiasset as PallasMultiasset, AssetName, PolicyId},
};

#[derive(Debug, PartialEq, Clone)]
pub struct MultiassetPositiveCoin {
    pub inner: PallasMultiasset<PositiveCoin>,
}

impl MultiassetPositiveCoin {
    pub fn new(multiasset: Vec<(String, Vec<(String, u64)>)>) -> Self {
        let mut policy_map: BTreeMap<PolicyId, BTreeMap<AssetName, PositiveCoin>> = BTreeMap::new();

        for (policy_id_str, assets) in multiasset {
            let policy_id = PolicyId::from_str(&policy_id_str).expect("Invalid PolicyId");

            let mut asset_map: BTreeMap<AssetName, PositiveCoin> = BTreeMap::new();
            for (asset_name_str, amount) in assets {
                let asset_name = AssetName::from_str(&asset_name_str).expect("Invalid AssetName");
                asset_map.insert(
                    asset_name,
                    PositiveCoin::try_from(amount).expect("Invalid amount"),
                );
            }

            policy_map.insert(policy_id, asset_map);
        }
        MultiassetPositiveCoin { inner: policy_map }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MultiassetNonZeroInt {
    pub inner: PallasMultiasset<NonZeroInt>,
}

impl MultiassetNonZeroInt {
    pub fn new(multiasset: Vec<(String, Vec<(String, i64)>)>) -> Self {
        let mut policy_map: BTreeMap<PolicyId, BTreeMap<AssetName, NonZeroInt>> = BTreeMap::new();

        for (policy_id_str, assets) in multiasset {
            let policy_id = PolicyId::from_str(&policy_id_str).expect("Invalid PolicyId");

            let mut asset_map: BTreeMap<AssetName, NonZeroInt> = BTreeMap::new();
            for (asset_name_str, amount) in assets {
                let asset_name = AssetName::from_str(&asset_name_str).expect("Invalid AssetName");
                asset_map.insert(
                    asset_name,
                    NonZeroInt::try_from(amount).expect("Invalid amount"),
                );
            }

            policy_map.insert(policy_id, asset_map);
        }
        MultiassetNonZeroInt { inner: policy_map }
    }
}
