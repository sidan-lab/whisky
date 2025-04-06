use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Network {
    Mainnet,
    Preprod,
    Preview,
    Custom(Vec<Vec<i64>>),
}

impl TryFrom<String> for Network {
    type Error = serde_json::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "mainnet" => Ok(Network::Mainnet),
            "preprod" => Ok(Network::Preprod),
            "preview" => Ok(Network::Preview),
            _ => serde_json::from_str(&s),
        }
    }
}