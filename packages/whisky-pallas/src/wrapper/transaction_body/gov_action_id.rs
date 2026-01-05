use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::conway::GovActionId as PallasGovActionId;
use pallas::ledger::primitives::Fragment;
use whisky_common::WError;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct GovActionId {
    pub inner: PallasGovActionId,
}

impl GovActionId {
    pub fn new(transaction_id: &str, index: u32) -> Result<Self, WError> {
        let digest: Hash<32> = transaction_id
            .parse()
            .map_err(|_| WError::new("GovActionId::new", "Invalid transaction id length"))?;

        let inner = PallasGovActionId {
            transaction_id: digest,
            action_index: index,
        };
        Ok(Self { inner })
    }

    pub fn encode(&self) -> Result<String, WError> {
        let encoded = self.inner.encode_fragment().map_err(|e| {
            WError::new(
                "GovActionId::encode",
                &format!("Fragment encode error: {}", e),
            )
        })?;
        Ok(hex::encode(encoded))
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasGovActionId::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "GovActionId::decode_bytes",
                &format!("Fragment decode error: {}", e),
            )
        })?;
        Ok(Self { inner })
    }
}
