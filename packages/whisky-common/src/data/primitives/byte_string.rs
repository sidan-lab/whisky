use serde_json::{json, Value};

use crate::data::PlutusDataJson;

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

impl PlutusDataJson for ByteString {
    fn to_json(&self) -> Value {
        byte_string(&self.bytes)
    }
}

pub fn byte_string(bytes: &str) -> Value {
    json!({ "bytes": bytes })
}

pub fn builtin_byte_string(bytes: &str) -> Value {
    json!({ "bytes": bytes })
}

pub type ByteArray = ByteString;

pub type ScriptHash = ByteString;

pub type PolicyId = ByteString;

pub type AssetName = ByteString;

pub type PubKeyHash = ByteString;

pub type VerificationKeyHash = ByteString;
