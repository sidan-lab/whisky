use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Datum {
    Inline(String),
    Hash(String),
}

impl Datum {
    pub fn get_inner(&self) -> &str {
        match self {
            Datum::Inline(s) => s,
            Datum::Hash(s) => s,
        }
    }
}