use crate::data::{constr0, constr1, PlutusDataJson};
use serde_json::{json, Value};

#[derive(Clone, Debug)]
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
}

pub fn bool(b: bool) -> Value {
    if b {
        constr1(json!([]))
    } else {
        constr0(json!([]))
    }
}
