use whisky_common::{TxIn, WError};

use super::TxParser;

impl TxParser {
    pub fn inputs(&mut self) -> Result<Vec<TxIn>, WError> {
        Ok(vec![])
    }
}
