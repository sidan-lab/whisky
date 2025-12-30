use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::conway::TransactionInput as PallasTransactionInput;
use pallas::ledger::primitives::Fragment;
use whisky_common::WError;

#[derive(Debug, PartialEq, Clone)]
pub struct TransactionInput {
    pub inner: PallasTransactionInput,
}

impl TransactionInput {
    pub fn new(transaction_id: &str, index: u64) -> Result<Self, WError> {
        let digest: Hash<32> = transaction_id.parse().map_err(|_| {
            WError::new(
                "WhiskyPallas - Serializing transaction input:",
                "Invalid transaction id length",
            )
        })?;

        let inner = PallasTransactionInput {
            transaction_id: digest,
            index,
        };

        Ok(Self { inner })
    }

    pub fn encode(&self) -> Result<String, WError> {
        self.inner
            .encode_fragment()
            .map(|bytes| hex::encode(bytes))
            .map_err(|_| {
                WError::new(
                    "WhiskyPallas - Encoding transaction input:",
                    "Failed to encode fragment",
                )
            })
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasTransactionInput::decode_fragment(bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Decoding transaction input:",
                &format!("Fragment decode error: {}", e.to_string()),
            )
        })?;
        Ok(Self { inner })
    }
}
