use serde_json::{json, Value};

use super::{con_str0, con_str1};

pub fn bool(b: bool) -> Value {
    if b {
        con_str1(json!([]))
    } else {
        con_str0(json!([]))
    }
}

pub fn byte_string(bytes: &str) -> Value {
    json!({ "bytes": bytes })
}

pub fn builtin_byte_string(bytes: &str) -> Value {
    json!({ "bytes": bytes })
}

pub fn integer(int: i64) -> Value {
    json!({ "int": int })
}

pub fn list<T: Into<Value>>(p_list: Vec<T>) -> Value {
    let list: Vec<Value> = p_list.into_iter().map(|item| item.into()).collect();
    json!({ "list": list })
}

pub fn assoc_map<K: Into<Value>, V: Into<Value>>(items_map: Vec<(K, V)>) -> Value {
    let map: Vec<Value> = items_map
        .into_iter()
        .map(|(k, v)| json!({"k": k.into(), "v": v.into()}))
        .collect();
    json!({ "map": map })
}

pub fn tuple<K: Into<Value>, V: Into<Value>>(key: K, value: V) -> Value {
    con_str0(vec![key.into(), value.into()])
}

pub fn pairs<K: Into<Value>, V: Into<Value>>(items_map: Vec<(K, V)>) -> Value {
    let map: Vec<Value> = items_map
        .into_iter()
        .map(|(k, v)| json!({"k": k.into(), "v": v.into()}))
        .collect();
    json!({ "map": map })
}
