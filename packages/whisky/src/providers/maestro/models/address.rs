use serde::Deserialize;

// use crate::provider::maestro::utils::last_updated::LastUpdated;

use super::utxo::Utxo;

#[derive(Deserialize, Debug, Clone)]
pub struct UtxosAtAddress {
    pub data: Vec<Utxo>,
    // pub last_updated: LastUpdated,
    pub next_cursor: Option<String>,
}
