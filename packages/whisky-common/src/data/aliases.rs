use crate::{
    data::{ByteString, Constr0, Int},
    impl_constr_type,
};
use serde_json::Value;

use super::{
    constr0,
    primitives::{byte_string, integer},
};

pub fn currency_symbol(policy_id: &str) -> Value {
    byte_string(policy_id)
}

pub fn token_name(token_name: &str) -> Value {
    byte_string(token_name)
}

pub fn asset_class(policy_id: &str, asset_name: &str) -> Value {
    constr0(vec![currency_symbol(policy_id), token_name(asset_name)])
}

pub fn tx_out_ref(tx_hash: &str, index: i128) -> Value {
    constr0(vec![constr0(vec![byte_string(tx_hash)]), integer(index)])
}

pub fn output_reference(tx_hash: &str, index: i128) -> Value {
    constr0(vec![byte_string(tx_hash), integer(index)])
}

// Type alias
pub type OutputReference = Constr0<Box<(ByteString, Int)>>;
impl_constr_type!(OutputReference, 0, [(tx_hash: ByteString, &str), (index: Int, i128)]);
