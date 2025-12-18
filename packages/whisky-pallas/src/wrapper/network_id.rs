use pallas::ledger::primitives::{Fragment, NetworkId as PallasNetworkId};

pub enum NetworkIdKind {
    Mainnet,
    Testnet,
}

pub struct NetworkId {
    pub inner: PallasNetworkId,
}

impl NetworkId {
    pub fn new(network_id: NetworkIdKind) -> Self {
        let pallas_network_id = match network_id {
            NetworkIdKind::Mainnet => PallasNetworkId::Mainnet,
            NetworkIdKind::Testnet => PallasNetworkId::Testnet,
        };

        Self {
            inner: pallas_network_id,
        }
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at NetworkId"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasNetworkId::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
