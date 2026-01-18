use serde_json::{json, Value};

use crate::{data::PlutusDataJson, WError};

#[derive(Clone, Debug, PartialEq)]
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

    fn from_json(value: &Value) -> Result<Self, WError> {
        let bytes = value
            .get("bytes")
            .ok_or_else(|| WError::new("ByteString::from_json", "missing 'bytes' field"))?
            .as_str()
            .ok_or_else(|| WError::new("ByteString::from_json", "invalid 'bytes' value"))?;
        Ok(ByteString {
            bytes: bytes.to_string(),
        })
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
