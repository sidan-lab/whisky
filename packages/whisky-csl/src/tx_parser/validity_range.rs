use whisky_common::{ValidityRange, WError};

use super::TxParser;

impl TxParser {
    pub fn validity_range(&mut self) -> Result<ValidityRange, WError> {
        Ok(ValidityRange {
            invalid_before: None,
            invalid_hereafter: None,
        })
    }
}
