use std::str::FromStr;

use pallas::{
    codec::utils::{Bytes, NonEmptySet},
    ledger::{primitives::conway::VKeyWitness, traverse::ComputeHash},
};
use pallas_crypto::key::ed25519::SecretKey;
use whisky_common::{TxBuildable, TxBuilderBody, WError};

use crate::{
    wrapper::{transaction_body::Transaction, witness_set::vkey_witness},
    WhiskyPallas,
};

impl TxBuildable for WhiskyPallas {
    fn set_protocol_params(&mut self, protocol_params: whisky_common::Protocol) {
        self.core.protocol_params = protocol_params;
    }

    fn set_tx_builder_body(&mut self, tx_builder: whisky_common::TxBuilderBody) {
        self.tx_builder_body = tx_builder;
    }

    fn reset_builder(&mut self) {
        self.tx_builder_body = TxBuilderBody::default();
    }

    fn serialize_tx_body(&mut self) -> Result<String, whisky_common::WError> {
        let tx_hex = self.core.build_tx(self.tx_builder_body.clone(), true)?;
        self.tx_hex = tx_hex.clone();
        Ok(tx_hex)
    }

    fn unbalanced_serialize_tx_body(&mut self) -> Result<String, whisky_common::WError> {
        let tx_hex = self.core.build_tx(self.tx_builder_body.clone(), false)?;
        self.tx_hex = tx_hex.clone();
        Ok(tx_hex)
    }

    fn complete_signing(&mut self) -> Result<String, whisky_common::WError> {
        let mut vkey_witnesses: Vec<VKeyWitness> = vec![];
        let transaction_bytes = hex::decode(self.tx_hex.clone()).unwrap();
        let mut transaction = Transaction::decode_bytes(&transaction_bytes)?;
        for signer in &self.tx_builder_body.signing_key {
            let data = hex::decode(signer).map_err(|e| {
                WError::new(
                    "WhiskyPallas CompleteSigning - ",
                    &format!("Failed to decode signing key hex: {}", e.to_string()),
                )
            })?;
            let data_bytes: [u8; 32] = data.try_into().map_err(|_| {
                WError::new("WhiskyPallas CompleteSigning - ", "Key must be 32 bytes")
            })?;
            let secret_key = SecretKey::from(data_bytes);
            let signature = secret_key.sign(transaction.inner.transaction_body.compute_hash());

            let vkey_witness: VKeyWitness = VKeyWitness {
                vkey: Bytes::from_str(&secret_key.public_key().to_string()).unwrap(),
                signature: Bytes::from_str(&signature.to_string()).unwrap(),
            };
            vkey_witnesses.push(vkey_witness);
        }
        if !vkey_witnesses.is_empty() {
            transaction.inner.transaction_witness_set.vkeywitness =
                Some(NonEmptySet::from_vec(vkey_witnesses).unwrap());
        }
        Ok(transaction.encode()?)
    }

    fn set_tx_hex(&mut self, tx_hex: String) {
        self.tx_hex = tx_hex;
    }

    fn tx_hex(&mut self) -> String {
        self.tx_hex.clone()
    }

    fn tx_evaluation_multiplier_percentage(&self) -> u64 {
        self.tx_evaluation_multiplier_percentage
    }

    fn add_tx_in(&mut self, input: whisky_common::PubKeyTxIn) -> Result<(), whisky_common::WError> {
        self.tx_builder_body
            .inputs
            .push(whisky_common::TxIn::PubKeyTxIn(input));
        Ok(())
    }
}
