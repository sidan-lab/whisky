use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidityRange {
    pub invalid_before: Option<u64>,
    pub invalid_hereafter: Option<u64>,
}
