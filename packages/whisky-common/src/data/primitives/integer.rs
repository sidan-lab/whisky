use serde_json::{json, Value};

use crate::{data::PlutusDataJson, WError};

#[derive(Clone, Debug, PartialEq)]
pub struct Int {
    pub int: i128,
}

impl Int {
    pub fn new(int: i128) -> Self {
        Int { int }
    }
}

impl PlutusDataJson for Int {
    fn to_json(&self) -> Value {
        integer(self.int)
    }

    fn from_json(value: &Value) -> Result<Self, WError> {
        let int = value
            .get("int")
            .ok_or_else(|| WError::new("Int::from_json", "missing 'int' field"))?
            .as_i64()
            .map(|v| v as i128)
            .or_else(|| {
                // Try to parse as string for large numbers
                value.get("int").and_then(|v| {
                    v.as_str()
                        .and_then(|s| s.parse::<i128>().ok())
                        .or_else(|| v.as_u64().map(|u| u as i128))
                })
            })
            .ok_or_else(|| WError::new("Int::from_json", "invalid 'int' value"))?;
        Ok(Int { int })
    }
}

pub fn integer(int: i128) -> Value {
    json!({ "int": int })
}

pub fn posix_time(posix_time: i128) -> Value {
    integer(posix_time)
}
