use whisky_common::{Certificate, WError};

use super::TxParser;

impl TxParser {
    pub fn certificates(&mut self) -> Result<Vec<Certificate>, WError> {
        Ok(vec![])
    }
}
