use whisky_common::{Metadata, WError};

use super::TxParser;

impl TxParser {
    pub fn metadata(&mut self) -> Result<Vec<Metadata>, WError> {
        Ok(vec![])
    }
}
