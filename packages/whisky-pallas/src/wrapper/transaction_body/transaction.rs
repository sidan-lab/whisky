// use pallas::ledger::primitives::{conway::Tx as PallasTx, Fragment};

// use crate::wrapper::TransactionBody;

// #[derive(Clone, Debug)]
// pub struct Transaction<'a> {
//     pub inner: PallasTx<'a>,
// }

// impl<'a> Transaction<'a> {
//     pub fn new(transaction_body: TransactionBody<'a>)

//     pub fn encode(&self) -> String {
//         hex::encode(
//             self.inner
//                 .encode_fragment()
//                 .expect("encoding failed at Transaction"),
//         )
//     }

//     pub fn decode_bytes(bytes: &'a [u8]) -> Result<Self, String> {
//         let inner = PallasTx::decode_fragment(&bytes)
//             .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
//         Ok(Self { inner })
//     }
// }
