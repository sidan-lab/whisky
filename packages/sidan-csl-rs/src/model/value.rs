use serde::{Deserialize, Serialize};

use super::Asset;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Value(pub HashMap<String, u64>);

impl Value {
    pub fn new() -> Self {
        Value(HashMap::new())
    }

    pub fn from_asset(asset: Asset) -> Self {
        let mut asset_map = HashMap::new();
        asset_map.insert(
            Value::santitize_unit(&asset.unit()),
            asset.quantity().parse::<u64>().unwrap(),
        );
        Value(asset_map)
    }

    pub fn from_asset_vec(assets: Vec<Asset>) -> Self {
        let mut asset_map = HashMap::new();
        for asset in assets {
            let current_value = asset_map
                .entry(Value::santitize_unit(&asset.unit()))
                .or_insert(0);
            *current_value += asset.quantity().parse::<u64>().unwrap();
        }
        Value(asset_map)
    }

    pub fn add_asset(&mut self, asset: Asset) -> &mut Self {
        let current_value = self
            .0
            .entry(Value::santitize_unit(&asset.unit()))
            .or_insert(0);
        *current_value += asset.quantity().parse::<u64>().unwrap();
        self
    }

    pub fn merge(&mut self, other: Value) -> &mut Self {
        for (key, value) in other.0 {
            let current_value = self.0.entry(Value::santitize_unit(&key)).or_insert(0);
            *current_value += value;
        }
        self
    }

    pub fn negate_asset(&mut self, other: Asset) -> &mut Self {
        let current_value = self
            .0
            .entry(Value::santitize_unit(&other.unit()))
            .or_insert(0);
        let negate_quantity = other.quantity().parse::<u64>().unwrap();
        if *current_value <= negate_quantity {
            self.0.remove(&other.unit());
        } else {
            *current_value -= negate_quantity;
        };
        self
    }

    pub fn negate_assets(&mut self, other: Value) -> &mut Self {
        for (key, value) in other.0 {
            let unit = Value::santitize_unit(&key);
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
            assets.push(Asset::new(
                Value::santitize_unit(unit),
                quantity.to_string(),
            ));
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

    pub fn keys(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }

    // Comparison function
    pub fn geq(&self, other: &Value) -> bool {
        for (key, value) in &other.0 {
            if self
                .0
                .get(&Value::santitize_unit(key))
                .map_or(false, |v| v < value)
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
                .get(&Value::santitize_unit(key))
                .map_or(false, |v| v > value)
            {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn santitize_unit(unit: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_assets() {
        let mut assets = Value::new();
        let mut other = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        other.0.insert("lovelace".to_string(), 100);
        assets.merge(other);
        assert_eq!(assets.0.get("lovelace").unwrap(), &200);
    }

    #[test]
    fn test_merge_multiple_assets() {
        let mut assets = Value::new();
        let mut other = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        other.0.insert("lovelace".to_string(), 100);
        assets.0.insert("asset1".to_string(), 100);
        other.0.insert("asset2".to_string(), 50);
        assets.merge(other);
        assert_eq!(assets.0.get("lovelace").unwrap(), &200);
        assert_eq!(assets.0.get("asset1").unwrap(), &100);
        assert_eq!(assets.0.get("asset2").unwrap(), &50);
    }

    #[test]
    fn test_negate_asset() {
        let mut assets = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        assets.negate_asset(Asset::new_from_str("lovelace", "65"));
        assert_eq!(assets.0.get("lovelace").unwrap(), &35);
    }

    #[test]
    fn test_negate_asset_to_zero() {
        let mut assets = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        assets.negate_asset(Asset::new_from_str("lovelace", "101"));
        assert_eq!(assets.0.get("lovelace"), None);
    }

    #[test]
    fn test_negate_assets() {
        let mut assets = Value::new();
        let mut other = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        other.0.insert("lovelace".to_string(), 65);
        assets.negate_assets(other);
        assert_eq!(assets.0.get("lovelace").unwrap(), &35);
    }

    #[test]
    fn test_negate_assets_to_zero() {
        let mut assets = Value::new();
        let mut other = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        other.0.insert("lovelace".to_string(), 101);
        assets.negate_assets(other);
        assert_eq!(assets.0.get("lovelace"), None);
    }

    #[test]
    fn test_negate_multiple_assets() {
        let mut assets = Value::new();
        let mut other = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        other.0.insert("lovelace".to_string(), 65);
        assets.0.insert("asset1".to_string(), 100);
        other.0.insert("asset2".to_string(), 50);
        assets.negate_assets(other);
        assert_eq!(assets.0.get("lovelace").unwrap(), &35);
        assert_eq!(assets.0.get("asset1").unwrap(), &100);
        assert_eq!(assets.0.get("asset2"), None);
    }

    #[test]
    fn test_geq() {
        let mut first_assets = Value::new();
        first_assets
            .add_asset(Asset::new_from_str("lovelace", "1012760"))
            .add_asset(Asset::new_from_str("asset1", "100"));

        let mut second_assets = Value::new();
        second_assets
            .add_asset(Asset::new_from_str("lovelace", "1000000"))
            .add_asset(Asset::new_from_str("asset1", "100"));

        assert!(first_assets.geq(&second_assets));
    }

    #[test]
    fn test_leq() {
        let mut first_assets = Value::new();
        first_assets
            .add_asset(Asset::new_from_str("lovelace", "1000000"))
            .add_asset(Asset::new_from_str("asset1", "100"));

        let mut second_assets = Value::new();
        second_assets
            .add_asset(Asset::new_from_str("lovelace", "1012760"))
            .add_asset(Asset::new_from_str("asset1", "100"));

        assert!(first_assets.leq(&second_assets));
    }

    #[test]
    fn test_is_empty() {
        let assets = Value::new();
        assert!(assets.is_empty());
    }
}
