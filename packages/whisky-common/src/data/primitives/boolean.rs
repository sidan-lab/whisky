use crate::{data::{constr0, constr1, PlutusDataJson}, WError};
use serde_json::{json, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum Bool {
    True,
    False,
}

impl Bool {
    pub fn new(b: bool) -> Self {
        if b {
            Bool::True
        } else {
            Bool::False
        }
    }
}

impl PlutusDataJson for Bool {
    fn to_json(&self) -> serde_json::Value {
        match self {
            Bool::True => constr1(json!([])),
            Bool::False => constr0(json!([])),
        }
    }

    fn from_json(value: &Value) -> Result<Self, WError> {
        let constructor = value
            .get("constructor")
            .ok_or_else(|| WError::new("Bool::from_json", "missing 'constructor' field"))?
            .as_u64()
            .ok_or_else(|| WError::new("Bool::from_json", "invalid 'constructor' value"))?;

        match constructor {
            0 => Ok(Bool::False),
            1 => Ok(Bool::True),
            _ => Err(WError::new(
                "Bool::from_json",
                &format!("invalid constructor tag for Bool: {}", constructor),
            )),
        }
    }
}

pub fn bool(b: bool) -> Value {
    if b {
        constr1(json!([]))
    } else {
        constr0(json!([]))
    }
}
