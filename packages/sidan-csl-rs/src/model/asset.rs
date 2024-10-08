use crate::*;
use schemars::JsonSchema;
use serde;

#[wasm_bindgen]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_asset() {
        let asset = Asset::new("lovelace".to_string(), "1000000".to_string());
        assert_eq!(asset.unit(), "lovelace".to_string());
        assert_eq!(asset.policy(), "lovelace".to_string());
        assert_eq!(asset.name(), "".to_string());
        assert_eq!(asset.quantity(), "1000000".to_string());
        assert_eq!(asset.quantity_i128(), 1000000);
    }

    #[test]
    fn test_asset2() {
        let asset = Asset::new(
            "fc0e0323b254c0eb7275349d1e32eb6cc7ecfd03f3b71408eb46d75168696e736f6e2e616461"
                .to_string(),
            "89346934".to_string(),
        );
        assert_eq!(
            asset.unit(),
            "fc0e0323b254c0eb7275349d1e32eb6cc7ecfd03f3b71408eb46d75168696e736f6e2e616461"
                .to_string()
        );
        assert_eq!(
            asset.policy(),
            "fc0e0323b254c0eb7275349d1e32eb6cc7ecfd03f3b71408eb46d751".to_string()
        );
        assert_eq!(asset.name(), "68696e736f6e2e616461".to_string());
        assert_eq!(asset.quantity(), "89346934".to_string());
        assert_eq!(asset.quantity_i128(), 89346934);
    }
}
