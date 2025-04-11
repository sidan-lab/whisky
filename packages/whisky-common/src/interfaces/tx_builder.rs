use crate::{errors::*, Protocol, TxBuilderBody};
use std::fmt::Debug;

pub trait TxBuildable: Clone + Debug {
    fn set_protocol_params(&mut self, protocol_params: Protocol) -> &mut Self;
    fn set_tx_builder_body(&mut self, tx_builder: TxBuilderBody) -> &mut Self;
    fn reset_builder(&mut self) -> &mut Self;

    fn serialize_tx_body(&mut self) -> Result<String, WError>;
    fn unbalanced_serialize_tx_body(&mut self) -> Result<String, WError>;
    fn complete_signing(&mut self) -> Result<String, WError>;
    fn tx_hex(&mut self) -> String;
}
