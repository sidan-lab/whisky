use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Network {
    Mainnet,
    Preprod,
    Preview,
    Custom(Vec<Vec<i64>>),
}
