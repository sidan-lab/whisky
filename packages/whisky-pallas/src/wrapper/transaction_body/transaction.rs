use crate::wrapper::{
    auxiliary_data::auxiliary_data::AuxiliaryData,
    transaction_body::transaction_body::TransactionBody, witness_set::witness_set::WitnessSet,
};
use pallas::{
    codec::utils::{KeepRaw, Nullable},
    ledger::primitives::{conway::Tx as PallasTx, Fragment},
};
use whisky_common::WError;

#[derive(Clone, Debug)]
pub struct Transaction<'a> {
    pub inner: PallasTx<'a>,
}

impl<'a> Transaction<'a> {
    pub fn new(
        transaction_body: TransactionBody<'a>,
        transaction_witness_set: WitnessSet<'a>,
        success: bool,
        auxiliary_data: Option<AuxiliaryData>,
    ) -> Result<Self, WError> {
        let inner = PallasTx {
            transaction_body: KeepRaw::from(transaction_body.inner),
            transaction_witness_set: KeepRaw::from(transaction_witness_set.inner),
            success,
            auxiliary_data: match auxiliary_data {
                Some(aux_data) => Nullable::Some(KeepRaw::from(aux_data.inner)),
                None => Nullable::Null,
            },
        };

        Ok(Self { inner })
    }

    pub fn encode(&self) -> Result<String, WError> {
        self.inner
            .encode_fragment()
            .map(|bytes| hex::encode(bytes))
            .map_err(|_| {
                WError::new(
                    "WhiskyPallas - Encoding transaction:",
                    "Failed to encode fragment",
                )
            })
    }

    pub fn decode_bytes(bytes: &'a [u8]) -> Result<Self, WError> {
        let inner = PallasTx::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Decoding transaction:",
                &format!("Fragment decode error: {}", e.to_string()),
            )
        })?;
        Ok(Self { inner })
    }
}
