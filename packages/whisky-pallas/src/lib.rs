pub mod converter;
pub mod tx_builder;
pub mod utils;
pub mod wrapper;

use crate::tx_builder::core_pallas::CorePallas;
use whisky_common::{Protocol, TxBuilderBody};

#[derive(Clone, Debug)]
pub struct WhiskyPallas {
    pub core: CorePallas,
    pub tx_builder_body: TxBuilderBody,
    pub tx_evaluation_multiplier_percentage: u64,
    pub tx_hex: String,
}

impl WhiskyPallas {
    pub fn new(protocol_params: Option<Protocol>) -> Self {
        Self {
            core: CorePallas::new(match protocol_params {
                Some(params) => params,
                None => Protocol::default(),
            }),
            tx_builder_body: TxBuilderBody::new(),
            tx_evaluation_multiplier_percentage: 110,
            tx_hex: String::new(),
        }
    }
}
