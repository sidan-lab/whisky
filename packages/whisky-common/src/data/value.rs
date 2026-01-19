use serde::{Deserialize, Serialize};

use crate::{
    data::{ByteString, Int, Map, PlutusDataJson},
    models::Asset,
    WError,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Value(pub BTreeMap<String, u64>);

impl Value {
    pub fn new() -> Self {
        Value(BTreeMap::new())
    }

    pub fn from_asset(asset: &Asset) -> Self {
        let mut asset_map = BTreeMap::new();
        asset_map.insert(
            Value::sanitize_unit(&asset.unit()),
            asset.quantity().parse::<u64>().unwrap(),
        );
        Value(asset_map)
    }

    pub fn from_asset_vec(assets: &[Asset]) -> Self {
        let mut asset_map = BTreeMap::new();
        for asset in assets {
            let current_value = asset_map
                .entry(Value::sanitize_unit(&asset.unit()))
                .or_insert(0);
            *current_value += asset.quantity().parse::<u64>().unwrap();
        }
        Value(asset_map)
    }

    pub fn add_asset(&mut self, unit: &str, quantity: u64) -> &mut Self {
        let current_value = self.0.entry(Value::sanitize_unit(unit)).or_insert(0);
        *current_value += quantity;
        self
    }

    pub fn add_assets(&mut self, assets: &[Asset]) -> &mut Self {
        for asset in assets {
            self.add_asset(&asset.unit(), asset.quantity().parse::<u64>().unwrap());
        }
        self
    }

    pub fn merge(&mut self, other: &Value) -> &mut Self {
        for (key, value) in other.0.clone() {
            let current_value = self.0.entry(Value::sanitize_unit(&key)).or_insert(0);
            *current_value += value;
        }
        self
    }

    pub fn negate_asset(&mut self, unit: &str, quantity: u64) -> &mut Self {
        let current_value = self.0.entry(Value::sanitize_unit(unit)).or_insert(0);
        if *current_value <= quantity {
            self.0.remove(unit);
        } else {
            *current_value -= quantity;
        };
        self
    }

    pub fn negate_assets(&mut self, other: &[Asset]) -> &mut Self {
        for asset in other {
            self.negate_asset(&asset.unit(), asset.quantity().parse::<u64>().unwrap());
        }
        self
    }

    pub fn negate_value(&mut self, other: &Value) -> &mut Self {
        for (key, value) in other.0.clone() {
            let unit = Value::sanitize_unit(&key);
            let current_value = self.0.entry(unit.clone()).or_insert(0);
            if *current_value <= value {
                self.0.remove(&unit);
            } else {
                *current_value -= value;
            }
        }
        self
    }

    pub fn to_asset_vec(&self) -> Vec<Asset> {
        let mut assets = vec![];
        for (unit, quantity) in &self.0 {
            assets.push(Asset::new(Value::sanitize_unit(unit), quantity.to_string()));
        }
        assets
    }

    // Accessor
    pub fn get(&self, key: &str) -> u64 {
        let key = if key.is_empty() { "lovelace" } else { key };
        match self.0.get(key) {
            Some(value) => *value,
            None => 0,
        }
    }

    pub fn get_policy_assets(&self, policy_id: &str) -> Vec<Asset> {
        let mut assets = vec![];
        for (unit, quantity) in &self.0 {
            if unit.starts_with(policy_id) {
                assets.push(Asset::new(unit.clone(), quantity.to_string()));
            }
        }
        assets
    }

    pub fn keys(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    // Comparison function
    pub fn geq(&self, other: &Value) -> bool {
        for (key, value) in &other.0 {
            if self
                .0
                .get(&Value::sanitize_unit(key))
                .is_some_and(|v| v < value)
            {
                return false;
            }
        }
        true
    }

    pub fn leq(&self, other: &Value) -> bool {
        for (key, value) in &other.0 {
            if self
                .0
                .get(&Value::sanitize_unit(key))
                .is_some_and(|v| v > value)
            {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn sanitize_unit(unit: &str) -> String {
        if unit.is_empty() {
            "lovelace".to_string()
        } else {
            unit.to_string()
        }
    }
}

impl PlutusDataJson for Value {
    fn to_json(&self) -> serde_json::Value {
        let mut value_map: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();

        self.0.iter().for_each(|(unit, quantity)| {
            let sanitized_name = unit.replace("lovelace", "");
            let policy = &sanitized_name[..56.min(sanitized_name.len())];
            let token = &sanitized_name[56.min(sanitized_name.len())..];

            value_map
                .entry(policy.to_string())
                .or_insert_with(BTreeMap::new)
                .entry(token.to_string())
                .and_modify(|q| *q += quantity)
                .or_insert(*quantity);
        });

        let json_map = value_map
            .into_iter() // Keys will already be sorted
            .map(|(policy, tokens)| {
                (
                    ByteString::new(&policy),
                    tokens
                        .into_iter() // Token keys will already be sorted
                        .map(|(token, quantity)| {
                            (ByteString::new(&token), Int::new(quantity as i128))
                        })
                        .collect(),
                )
            })
            .collect::<Map<ByteString, Map<ByteString, Int>>>();
        json_map.to_json()
    }

    fn from_json(value: &serde_json::Value) -> Result<Self, WError> {
        // Parse as Map<ByteString, Map<ByteString, Int>>
        let outer_map = Map::<ByteString, Map<ByteString, Int>>::from_json(value)
            .map_err(WError::add_err_trace("Value::from_json"))?;

        let mut asset_map = BTreeMap::new();

        for (policy_bytes, tokens_map) in outer_map.map {
            let policy = policy_bytes.bytes;

            for (token_bytes, quantity) in tokens_map.map {
                let token = token_bytes.bytes;
                let unit = if policy.is_empty() {
                    "lovelace".to_string()
                } else {
                    format!("{}{}", policy, token)
                };

                // quantity.int is i128, convert to u64
                let qty = quantity.int as u64;
                asset_map
                    .entry(unit)
                    .and_modify(|q| *q += qty)
                    .or_insert(qty);
            }
        }

        Ok(Value(asset_map))
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::new()
    }
}
