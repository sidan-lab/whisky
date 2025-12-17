use pallas::codec::utils::Set;
use pallas::ledger::primitives::conway::{
    TransactionBody as PallasTransactionBody, TransactionInput as PallasTransactionInput,
    TransactionOutput as PallasTransactionOutput,
};
use pallas::ledger::primitives::Fragment;

use crate::wrapper::transaction_input::TransactionInput;
use crate::wrapper::transaction_output::TransactionOutput;

#[derive(Debug, PartialEq, Clone)]
pub struct TransactionBody<'a> {
    pub inner: PallasTransactionBody<'a>,
}

impl<'a> TransactionBody<'a> {
    pub fn new(
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput<'a>>,
    ) -> Result<Self, String> {
        Ok(Self {
            inner: PallasTransactionBody {
                inputs: Self::parse_inputs(inputs),
                outputs: Self::parse_outputs(outputs),
                fee: 0,                        // Placeholder implementation
                ttl: None,                     // Placeholder implementation
                certificates: None,            // Placeholder implementation
                withdrawals: None,             // Placeholder implementation
                auxiliary_data_hash: None,     // Placeholder implementation
                validity_interval_start: None, // Placeholder implementation
                mint: None,                    // Placeholder implementation
                script_data_hash: None,        // Placeholder implementation
                collateral: None,              // Placeholder implementation
                required_signers: None,        // Placeholder implementation
                network_id: None,              // Placeholder implementation
                collateral_return: None,       // Placeholder implementation
                total_collateral: None,        // Placeholder implementation
                reference_inputs: None,        // Placeholder implementation
                voting_procedures: None,       // Placeholder implementation
                proposal_procedures: None,     // Placeholder implementation
                treasury_value: None,          // Placeholder implementation
                donation: None,                // Placeholder implementation
            },
        }) // Placeholder implementation
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at TransactionBody"),
        )
    }

    pub fn decode_bytes(bytes: &'a [u8]) -> Result<Self, String> {
        let inner = PallasTransactionBody::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }

    fn parse_inputs(inputs: Vec<TransactionInput>) -> Set<PallasTransactionInput> {
        let pallas_inputs: Vec<PallasTransactionInput> =
            inputs.into_iter().map(|input| input.inner).collect();
        Set::from(pallas_inputs)
    }

    fn parse_outputs(outputs: Vec<TransactionOutput<'a>>) -> Vec<PallasTransactionOutput<'a>> {
        outputs.into_iter().map(|output| output.inner).collect()
    }
}
