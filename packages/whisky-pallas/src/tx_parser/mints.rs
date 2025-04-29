use whisky_common::{MintItem, WError};

use super::TxParser;

impl TxParser {
    pub fn mints(&mut self) -> Result<Vec<MintItem>, WError> {
        Ok(vec![])
    }
}
