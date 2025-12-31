use std::str::FromStr;

use pallas::codec::utils::{Bytes, CborWrap, KeepRaw};
use pallas::ledger::primitives::babbage::{GenPostAlonzoTransactionOutput, GenTransactionOutput};
use pallas::ledger::primitives::conway::TransactionOutput as PallasTransactionOutput;
use pallas::ledger::primitives::Fragment;
use whisky_common::WError;

use crate::wrapper::transaction_body::{Datum, ScriptRef, Value};

#[derive(Debug, PartialEq, Clone)]
pub struct TransactionOutput<'a> {
    pub inner: PallasTransactionOutput<'a>,
}

impl<'a> TransactionOutput<'a> {
    pub fn new(
        address: &str,
        value: Value,
        datum: Option<Datum<'a>>,
        script_ref: Option<ScriptRef<'a>>,
    ) -> Result<Self, WError> {
        let address = Bytes::from_str(&address).map_err(|e| {
            WError::new(
                "WhiskyPallas - Creating transaction output:",
                &format!("Invalid address bytes: {}", e),
            )
        })?;

        let pallas_transaction_output =
            GenTransactionOutput::PostAlonzo(KeepRaw::from(GenPostAlonzoTransactionOutput {
                address,
                value: value.inner,
                datum_option: datum.map(|d| KeepRaw::from(d.inner)),
                script_ref: script_ref.map(|s| CborWrap(s.inner)),
            }));

        Ok(Self {
            inner: pallas_transaction_output,
        })
    }

    pub fn encode(&self) -> Result<String, WError> {
        self.inner
            .encode_fragment()
            .map(|bytes| hex::encode(bytes))
            .map_err(|_| {
                WError::new(
                    "WhiskyPallas - Encoding transaction output:",
                    "Failed to encode fragment",
                )
            })
    }

    pub fn decode_bytes(bytes: &'a [u8]) -> Result<Self, WError> {
        let inner = PallasTransactionOutput::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Decoding transaction output:",
                &format!("Fragment decode error: {}", e.to_string()),
            )
        })?;
        Ok(Self { inner })
    }
}
