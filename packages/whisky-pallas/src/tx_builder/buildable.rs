use whisky_common::TxBuildable;

use crate::WhiskyPallas;

impl TxBuildable for WhiskyPallas {
    fn set_protocol_params(&mut self, protocol_params: whisky_common::Protocol) {
        todo!()
    }

    fn set_tx_builder_body(&mut self, tx_builder: whisky_common::TxBuilderBody) {
        todo!()
    }

    fn reset_builder(&mut self) {
        todo!()
    }

    fn serialize_tx_body(&mut self) -> Result<String, whisky_common::WError> {
        // let tx_builder_body = self.tx_builder_body;
        todo!()
    }

    fn unbalanced_serialize_tx_body(&mut self) -> Result<String, whisky_common::WError> {
        todo!()
    }

    fn complete_signing(&mut self) -> Result<String, whisky_common::WError> {
        todo!()
    }

    fn set_tx_hex(&mut self, tx_hex: String) {
        todo!()
    }

    fn tx_hex(&mut self) -> String {
        todo!()
    }

    fn tx_evaluation_multiplier_percentage(&self) -> u64 {
        todo!()
    }

    fn add_tx_in(&mut self, input: whisky_common::PubKeyTxIn) -> Result<(), whisky_common::WError> {
        todo!()
    }
}
