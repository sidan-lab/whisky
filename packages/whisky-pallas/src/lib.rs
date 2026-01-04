pub mod converter;
pub mod tx_builder;
pub mod utils;
pub mod wrapper;

use crate::tx_builder::core_pallas::CorePallas;
use whisky_common::TxBuilderBody;

#[derive(Clone, Debug)]
pub struct WhiskyPallas {
    pub core: CorePallas,
    pub tx_builder_body: TxBuilderBody,
    pub tx_evaluation_multiplier_percentage: u64,
    pub tx_hex: String,
}
