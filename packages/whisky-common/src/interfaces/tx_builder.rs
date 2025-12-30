use crate::{errors::*, Protocol, PubKeyTxIn, TxBuilderBody};
use std::fmt::Debug;

pub trait TxBuildable: Debug {
    fn set_protocol_params(&mut self, protocol_params: Protocol);
    fn set_tx_builder_body(&mut self, tx_builder: TxBuilderBody);
    fn reset_builder(&mut self);

    fn serialize_tx_body(&mut self) -> Result<String, WError>;
    fn unbalanced_serialize_tx_body(&mut self) -> Result<String, WError>;
    fn complete_signing(&mut self) -> Result<String, WError>;
    fn set_tx_hex(&mut self, tx_hex: String);
    fn tx_hex(&mut self) -> String;
    fn tx_evaluation_multiplier_percentage(&self) -> u64;

    fn add_tx_in(&mut self, input: PubKeyTxIn) -> Result<(), WError>;
}
