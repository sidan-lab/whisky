use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::conway::GovActionId as PallasGovActionId;
use pallas::ledger::primitives::Fragment;

pub struct GovActionId {
    pub inner: PallasGovActionId,
}

impl GovActionId {
    pub fn new(transaction_id: &str, index: u32) -> Result<Self, String> {
        let digest: Hash<32> = transaction_id
            .parse()
            .map_err(|_| "Invalid transaction id length".to_string())?;

        let inner = PallasGovActionId {
            transaction_id: digest,
            action_index: index,
        };
        Ok(Self { inner })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at GovActionId"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasGovActionId::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
