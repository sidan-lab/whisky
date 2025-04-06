use crate::errors::*;
use std::fmt::Debug;

pub trait TxBuildable: Clone + Debug {
    fn serialize_tx_body(&mut self) -> Result<String, WError>;
}
