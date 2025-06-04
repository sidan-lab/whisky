use crate::{TxBuilderBody, TxTester, UTxO, WError};

pub trait TxParsable {
    fn parse(&mut self, tx_hex: &str, resolved_utxos: &[UTxO]) -> Result<(), WError>;
    fn get_builder_body(&self) -> TxBuilderBody;
    fn get_builder_body_without_change(&self) -> TxBuilderBody;
    fn to_tester(&self) -> TxTester;
}
