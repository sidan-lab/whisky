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
