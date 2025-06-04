use whisky_common::{Fetcher, TxBuilderBody, TxParsable, TxTester, UTxO, WError};
use whisky_csl::WhiskyCSL;

pub struct TxParser {
    pub fetcher: Option<Box<dyn Fetcher>>,
    pub serializer: Box<dyn TxParsable>,
}

impl TxParser {
    pub fn new() -> Self {
        TxParser {
            fetcher: None,
            serializer: Box::new(WhiskyCSL::new(None).unwrap()),
        }
    }

    pub fn parse(&mut self, tx_hex: &str, resolved_utxos: &[UTxO]) -> Result<&mut Self, WError> {
        self.serializer
            .parse(tx_hex, resolved_utxos)
            .map_err(WError::from_err("TxParser - parse"))?;
        Ok(self)
    }

    pub fn to_tester(&self) -> TxTester {
        self.serializer.to_tester()
    }

    pub fn get_builder_body(&self) -> TxBuilderBody {
        self.serializer.get_builder_body()
    }

    pub fn get_builder_body_without_change(&self) -> TxBuilderBody {
        self.serializer.get_builder_body_without_change()
    }
}
