use schemars::JsonSchema;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    unit: String,
    quantity: String,
}

impl Asset {
    pub fn unit_to_tuple(unit: &str) -> (String, String) {
        let unit = if unit == "lovelace" {
            "".to_string()
        } else {
            unit.to_string()
        };
        let policy = unit.chars().take(56).collect();
        let name = unit.chars().skip(56).collect();
        (policy, name)
    }
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
    pub fn policy(&self) -> String {
        self.unit.chars().take(56).collect()
    }
    pub fn name(&self) -> String {
        self.unit.chars().skip(56).collect()
    }
    pub fn quantity(&self) -> String {
        self.quantity.clone()
    }
    pub fn quantity_i128(&self) -> i128 {
        self.quantity.parse().unwrap()
    }
}
