use serde_json::{json, Value};

use crate::PlutusDataToJson;

#[derive(Clone, Debug)]
pub struct Map<K, V>
where
    K: Clone + PlutusDataToJson,
    V: Clone + PlutusDataToJson,
{
    pub map: Vec<(K, V)>,
}

impl<K, V> Map<K, V>
where
    K: Clone + PlutusDataToJson,
    V: Clone + PlutusDataToJson,
{
    pub fn new(map_items: &[(K, V)]) -> Self {
        Map {
            map: map_items.to_vec(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.map.push((key, value));
    }
}

impl<K, V> PlutusDataToJson for Map<K, V>
where
    K: Clone + PlutusDataToJson,
    V: Clone + PlutusDataToJson,
{
    fn to_json(&self) -> Value {
        let map_items_json: Vec<(Value, Value)> = self
            .map
            .iter()
            .map(|(k, v)| (k.clone().to_json(), v.clone().to_json()))
            .collect();
        pairs(map_items_json)
    }

    fn to_json_string(&self) -> String {
        self.to_json().to_string()
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
