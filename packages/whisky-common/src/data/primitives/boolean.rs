use serde_json::{json, Value};

use crate::{constr0, constr1, PlutusDataToJson};

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

impl PlutusDataToJson for Bool {
    fn to_json(&self) -> Value {
        match self {
            Bool::True => constr1(json!([])),
            Bool::False => constr0(json!([])),
        }
    }
    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }
}

pub fn bool(b: bool) -> Value {
    if b {
        constr1(json!([]))
    } else {
        constr0(json!([]))
    }
}
