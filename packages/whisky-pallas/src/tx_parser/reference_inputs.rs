use whisky_common::{RefTxIn, WError};

use super::TxParser;

impl TxParser {
    pub fn reference_inputs(&mut self) -> Result<Vec<RefTxIn>, WError> {
        Ok(vec![])
    }
}
