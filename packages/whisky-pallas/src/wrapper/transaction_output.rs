use std::str::FromStr;

use pallas::codec::utils::{Bytes, CborWrap, KeepRaw};
use pallas::ledger::primitives::babbage::{GenPostAlonzoTransactionOutput, GenTransactionOutput};
use pallas::ledger::primitives::conway::TransactionOutput as PallasTransactionOutput;
use pallas::ledger::primitives::Fragment;

use crate::wrapper::{Datum, ScriptRef, Value};

#[derive(Debug, PartialEq, Clone)]
pub struct TransactionOutput<'a> {
    pub inner: PallasTransactionOutput<'a>,
}

impl<'a> TransactionOutput<'a> {
    pub fn new(
        address: String,
        value: Value,
        datum: Option<Datum<'a>>,
        script_ref: Option<ScriptRef<'a>>,
    ) -> Result<Self, String> {
        let address =
            Bytes::from_str(&address).map_err(|e| format!("Invalid address bytes: {}", e))?;

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

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at TransactionOutput"),
        )
    }

    pub fn decode_bytes(bytes: &'a [u8]) -> Result<Self, String> {
        let inner = PallasTransactionOutput::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
