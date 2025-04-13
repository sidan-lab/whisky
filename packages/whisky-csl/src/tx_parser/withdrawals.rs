use whisky_common::{WError, Withdrawal};

use super::TxParser;

impl TxParser {
    pub fn withdrawals(&mut self) -> Result<Vec<Withdrawal>, WError> {
        Ok(vec![])
    }
}
