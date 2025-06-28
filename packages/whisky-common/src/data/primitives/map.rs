use std::collections::HashMap;
use std::iter::FromIterator;

use serde_json::{json, Value};

use crate::data::PlutusDataJson;

#[derive(Clone, Debug)]
pub struct Map<K, V>
where
    K: Clone + PlutusDataJson,
    V: Clone + PlutusDataJson,
{
    pub map: Vec<(K, V)>,
}

impl<K, V> Map<K, V>
where
    K: Clone + PlutusDataJson,
    V: Clone + PlutusDataJson,
{
    pub fn new(map_items: &[(K, V)]) -> Self {
        Map {
            map: map_items.to_vec(),
        }
    }

    pub fn from_map(hash_map: HashMap<K, V>) -> Self {
        Map {
            map: hash_map.into_iter().collect(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.map.push((key, value));
    }
}

// Implement FromIterator for Map to allow .collect() to work
impl<K, V> FromIterator<(K, V)> for Map<K, V>
where
    K: Clone + PlutusDataJson,
    V: Clone + PlutusDataJson,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = Map { map: Vec::new() };
        for (key, value) in iter {
            map.insert(key, value);
        }
        map
    }
}

impl<K, V> PlutusDataJson for Map<K, V>
where
    K: Clone + PlutusDataJson,
    V: Clone + PlutusDataJson,
{
    fn to_json(&self) -> Value {
        let map_items_json: Vec<(Value, Value)> = self
            .map
            .iter()
            .map(|(k, v)| (k.clone().to_json(), v.clone().to_json()))
            .collect();
        pairs(map_items_json)
    }
}

pub fn pairs<K: Into<Value>, V: Into<Value>>(items_map: Vec<(K, V)>) -> Value {
    let map: Vec<Value> = items_map
        .into_iter()
        .map(|(k, v)| json!({"k": k.into(), "v": v.into()}))
        .collect();
    json!({ "map": map })
}

pub fn assoc_map<K: Into<Value>, V: Into<Value>>(items_map: Vec<(K, V)>) -> Value {
    pairs(items_map)
}
