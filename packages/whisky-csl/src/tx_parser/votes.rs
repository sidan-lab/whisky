use whisky_common::{Vote, WError};

use super::TxParser;

impl TxParser {
    pub fn votes(&mut self) -> Result<Vec<Vote>, WError> {
        Ok(vec![])
    }
}
