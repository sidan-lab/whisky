use whisky_common::{PubKeyTxIn, WError};

use super::TxParser;

impl TxParser {
    pub fn collaterals(&mut self) -> Result<Vec<PubKeyTxIn>, WError> {
        Ok(vec![])
    }
}
