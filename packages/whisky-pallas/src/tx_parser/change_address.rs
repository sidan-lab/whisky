use whisky_common::WError;

use super::TxParser;

impl TxParser {
    pub fn change_address(&mut self) -> Result<String, WError> {
        Ok("".to_string())
    }
}
