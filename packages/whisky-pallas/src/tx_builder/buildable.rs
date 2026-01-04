use whisky_common::{TxBuildable, TxBuilderBody};

use crate::WhiskyPallas;

impl TxBuildable for WhiskyPallas {
    fn set_protocol_params(&mut self, protocol_params: whisky_common::Protocol) {
        self.core.protocol_params = protocol_params;
    }

    fn set_tx_builder_body(&mut self, tx_builder: whisky_common::TxBuilderBody) {
        self.tx_builder_body = tx_builder;
    }

    fn reset_builder(&mut self) {
        self.tx_builder_body = TxBuilderBody::default();
    }

    fn serialize_tx_body(&mut self) -> Result<String, whisky_common::WError> {
        let tx_hex = self.core.build_tx(self.tx_builder_body.clone())?;
        self.tx_hex = tx_hex.clone();
        Ok(tx_hex)
    }

    fn unbalanced_serialize_tx_body(&mut self) -> Result<String, whisky_common::WError> {
        todo!()
    }

    fn complete_signing(&mut self) -> Result<String, whisky_common::WError> {
        todo!()
    }

    fn set_tx_hex(&mut self, tx_hex: String) {
        self.tx_hex = tx_hex;
    }

    fn tx_hex(&mut self) -> String {
        self.tx_hex.clone()
    }

    fn tx_evaluation_multiplier_percentage(&self) -> u64 {
        self.core.tx_evaluation_multiplier_percentage
    }

    fn add_tx_in(&mut self, input: whisky_common::PubKeyTxIn) -> Result<(), whisky_common::WError> {
        self.tx_builder_body
            .inputs
            .push(whisky_common::TxIn::PubKeyTxIn(input));
        Ok(())
    }
}
