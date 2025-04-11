use serde::{Deserialize, Serialize};
use super::Budget;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub index: u32,
    pub budget: Budget,
    pub tag: RedeemerTag,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvalError {
    pub index: u32,
    pub budget: Budget,
    pub tag: RedeemerTag,
    pub error_message: String,
    pub logs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EvalResult {
    Success(Action),
    Error(EvalError),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RedeemerTag {
    Spend,
    Mint,
    Cert,
    Reward,
    Vote,
    Propose,
}
