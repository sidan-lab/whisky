use crate::*;
use schemars::JsonSchema;
use serde;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Asset {
    unit: String,
    quantity: String,
}

impl Asset {
    pub fn new(unit: String, quantity: String) -> Self {
        Asset { unit, quantity }
    }
    pub fn new_from_str(unit: &str, quantity: &str) -> Self {
        Asset {
            unit: unit.to_string(),
            quantity: quantity.to_string(),
        }
    }
    pub fn unit(&self) -> String {
        self.unit.clone()
    }
    pub fn quantity(&self) -> String {
        self.quantity.clone()
    }
}
