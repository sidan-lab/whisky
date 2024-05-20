#[derive(Clone, Debug, PartialEq)]
pub struct Asset {
    pub unit: String,
    pub quantity: String,
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
}
