use serde_json::{json, Value};

pub fn con_str<N: Into<Value>, T: Into<Value>>(constructor: N, fields: T) -> Value {
    json!({ "constructor": constructor.into(), "fields": fields.into() })
}

pub fn con_str0<T: Into<Value>>(fields: T) -> Value {
    con_str(0, fields)
}

pub fn con_str1<T: Into<Value>>(fields: T) -> Value {
    con_str(1, fields)
}

pub fn con_str2<T: Into<Value>>(fields: T) -> Value {
    con_str(2, fields)
}

pub fn bool(b: bool) -> Value {
    if b {
        con_str1(json!([]))
    } else {
        con_str0(json!([]))
    }
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

// Other functions like currencySymbol, tokenName, etc., would create JSON objects
pub fn currency_symbol(policy_id: &str) -> Value {
    builtin_byte_string(policy_id)
}

pub fn token_name(token_name: &str) -> Value {
    builtin_byte_string(token_name)
}

pub fn maybe_staking_hash(stake_credential: &str) -> Value {
    if stake_credential.is_empty() {
        con_str1(json!([]))
    } else {
        con_str0(vec![con_str0(vec![con_str0(vec![builtin_byte_string(
            stake_credential,
        )])])])
    }
}

pub fn pub_key_address(bytes: &str, stake_credential: Option<&str>) -> Value {
    con_str0(vec![
        con_str0(vec![builtin_byte_string(bytes)]),
        maybe_staking_hash(stake_credential.unwrap_or("")),
    ])
}

pub fn script_address(bytes: &str, stake_credential: Option<&str>) -> Value {
    con_str0(vec![
        con_str1(vec![builtin_byte_string(bytes)]),
        maybe_staking_hash(stake_credential.unwrap_or("")),
    ])
}

pub fn asset_class(policy_id: &str, asset_name: &str) -> Value {
    con_str0(vec![currency_symbol(policy_id), token_name(asset_name)])
}

pub fn tx_out_ref(tx_hash: &str, index: i64) -> Value {
    con_str0(vec![
        con_str0(vec![builtin_byte_string(tx_hash)]),
        integer(index),
    ])
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

pub fn payment_pub_key_hash(pub_key_hash: &str) -> Value {
    builtin_byte_string(pub_key_hash)
}

pub fn pub_key_hash(pub_key_hash: &str) -> Value {
    builtin_byte_string(pub_key_hash)
}

pub fn posix_time(posix_time: i64) -> Value {
    integer(posix_time)
}
