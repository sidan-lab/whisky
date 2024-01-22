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

pub fn integer(int: i32) -> Value {
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

pub fn tx_out_ref(tx_hash: &str, index: i32) -> Value {
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

pub fn posix_time(posix_time: i32) -> Value {
    integer(posix_time)
}

#[test]
fn test_con_str() {
    let correct_con_str = "{\"constructor\":10,\"fields\":[{\"bytes\":\"hello\"}]}";
    assert_eq!(
        con_str(10, json!([builtin_byte_string("hello")])).to_string(),
        correct_con_str
    );
}

#[test]
fn test_con_str0() {
    let correct_con_str0 = "{\"constructor\":0,\"fields\":{\"bytes\":\"hello\"}}";
    assert_eq!(
        con_str0(builtin_byte_string("hello")).to_string(),
        correct_con_str0
    );
}

#[test]
fn test_con_str1() {
    let correct_con_str1 = "{\"constructor\":1,\"fields\":{\"bytes\":\"hello\"}}";
    assert_eq!(
        con_str1(builtin_byte_string("hello")).to_string(),
        correct_con_str1
    );
}

#[test]
fn test_con_str2() {
    let correct_con_str2 = "{\"constructor\":2,\"fields\":{\"bytes\":\"hello\"}}";
    assert_eq!(
        con_str2(builtin_byte_string("hello")).to_string(),
        correct_con_str2
    );
}

#[test]
fn test_bool() {
    let correct_bool = "{\"constructor\":1,\"fields\":[]}";
    assert_eq!(bool(true).to_string(), correct_bool);
}

#[test]
fn test_builtin_byte_string() {
    let correct_builtin_byte_string = "{\"bytes\":\"hello\"}";
    assert_eq!(
        builtin_byte_string("hello").to_string(),
        correct_builtin_byte_string
    );
}

#[test]
fn test_integer() {
    let correct_integer = "{\"int\":1}";
    assert_eq!(integer(1).to_string(), correct_integer);
}

#[test]
fn test_list() {
    let correct_list = "{\"list\":[1,2,3]}";
    assert_eq!(list(vec![1, 2, 3]).to_string(), correct_list);
}

#[test]
fn test_maybe_staking_hash() {
    let correct_maybe_staking_hash = "{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"}]}]}]}";
    assert_eq!(
        maybe_staking_hash("hello").to_string(),
        correct_maybe_staking_hash
    );
}

#[test]
fn test_pub_key_address() {
    let correct_pub_key_address = "{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"bytes\":\"8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73\"}]},{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"bytes\":\"039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f\"}]}]}]}]}";
    assert_eq!(
        pub_key_address(
            "8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73",
            Some("039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f")
        )
        .to_string(),
        correct_pub_key_address
    );
}

#[test]
fn test_script_address() {
    let correct_script_address = "{\"constructor\":0,\"fields\":[{\"constructor\":1,\"fields\":[{\"bytes\":\"hello\"}]},{\"constructor\":1,\"fields\":[]}]}";
    assert_eq!(
        script_address("hello", None).to_string(),
        correct_script_address
    );
}

#[test]
fn test_asset_class() {
    let correct_asset_class =
        "{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"},{\"bytes\":\"world\"}]}";
    assert_eq!(
        asset_class("hello", "world").to_string(),
        correct_asset_class
    );
}

#[test]
fn test_tx_out_ref() {
    let correct_tx_out_ref = "{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"}]},{\"int\":12}]}";
    assert_eq!(tx_out_ref("hello", 12).to_string(), correct_tx_out_ref);
}

#[test]
fn test_assoc_map() {
    let correct_assoc_map =
    "{\"map\":[{\"k\":{\"bytes\":\"hello\"},\"v\":{\"bytes\":\"world\"}},{\"k\":{\"bytes\":\"123\"},\"v\":{\"bytes\":\"456\"}}]}";
    assert_eq!(
        assoc_map(vec![
            (builtin_byte_string("hello"), builtin_byte_string("world")),
            (builtin_byte_string("123"), builtin_byte_string("456"))
        ])
        .to_string(),
        correct_assoc_map
    );
}

#[test]
fn test_tuple() {
    let correct_tuple =
        "{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"},{\"bytes\":\"world\"}]}";
    assert_eq!(
        tuple(builtin_byte_string("hello"), builtin_byte_string("world")).to_string(),
        correct_tuple
    );
}
