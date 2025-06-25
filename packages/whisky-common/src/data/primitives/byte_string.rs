use serde_json::{json, Value};

use crate::PlutusDataToJson;

#[derive(Clone, Debug)]
pub struct ByteString {
    pub bytes: String,
}

impl ByteString {
    pub fn new(bytes: &str) -> Self {
        ByteString {
            bytes: bytes.to_string(),
        }
    }
}

impl PlutusDataToJson for ByteString {
    fn to_json(&self) -> Value {
        byte_string(&self.bytes)
    }
    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }
}

pub fn byte_string(bytes: &str) -> Value {
    json!({ "bytes": bytes })
}

pub fn builtin_byte_string(bytes: &str) -> Value {
    json!({ "bytes": bytes })
}
