use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::conway::TransactionInput as PallasTransactionInput;
use pallas::ledger::primitives::Fragment;

#[derive(Debug, PartialEq, Clone)]
pub struct TransactionInput {
    pub inner: PallasTransactionInput,
}

impl TransactionInput {
    pub fn new(transaction_id: &str, index: u64) -> Result<Self, String> {
        let digest: Hash<32> = transaction_id
            .parse()
            .map_err(|_| "Invalid transaction id length".to_string())?;

        let inner = PallasTransactionInput {
            transaction_id: digest,
            index,
        };

        Ok(Self { inner })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at TransactionInput"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasTransactionInput::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
