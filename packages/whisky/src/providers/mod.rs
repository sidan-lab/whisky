pub mod blockfrost;
pub mod maestro;
pub use blockfrost::BlockfrostProvider;
pub use maestro::MaestroProvider;

use serde::Serialize;

#[derive(Serialize)]
pub struct AdditionalUtxo {
    pub tx_hash: String,
    pub index: u32,
    pub txout_cbor: String,
}
