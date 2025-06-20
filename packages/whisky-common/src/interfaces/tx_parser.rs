use crate::{TxBuilderBody, TxTester, UTxO, UtxoInput, WError};

pub trait TxParsable {
    fn parse(&mut self, tx_hex: &str, resolved_utxos: &[UTxO]) -> Result<(), WError>;
    fn get_required_inputs(&mut self, tx_hex: &str) -> Result<Vec<UtxoInput>, WError>;
    fn get_builder_body(&self) -> TxBuilderBody;
    fn get_builder_body_without_change(&self) -> TxBuilderBody;
    fn to_tester(&self) -> TxTester;
}
