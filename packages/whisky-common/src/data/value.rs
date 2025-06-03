use serde::{Deserialize, Serialize};

use crate::models::Asset;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Value(pub HashMap<String, u64>);

impl Value {
    pub fn new() -> Self {
        Value(HashMap::new())
    }

    pub fn from_asset(asset: &Asset) -> Self {
        let mut asset_map = HashMap::new();
        asset_map.insert(
            Value::sanitize_unit(&asset.unit()),
            asset.quantity().parse::<u64>().unwrap(),
        );
        Value(asset_map)
    }

    pub fn from_asset_vec(assets: &[Asset]) -> Self {
        let mut asset_map = HashMap::new();
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

impl Default for Value {
    fn default() -> Self {
        Value::new()
    }
}
