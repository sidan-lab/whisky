use whisky_common::{Datum, WError};

use super::TxParser;

impl TxParser {
    pub fn change_datum(&mut self) -> Result<Option<Datum>, WError> {
        Ok(None)
    }
}
