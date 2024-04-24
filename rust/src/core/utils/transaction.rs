use crate::{csl, model::*, *};
use cryptoxide::blake2b::Blake2b;

pub(crate) fn blake2b256(data: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

#[wasm_bindgen]
pub fn calculate_tx_hash(tx_hex: &str) -> String {
    let csl_tx = csl::protocol_types::fixed_tx::FixedTransaction::from_hex(tx_hex).unwrap();
    csl::crypto::TransactionHash::from(blake2b256(&csl_tx.raw_body())).to_hex()
}

#[wasm_bindgen]
pub fn sign_transaction(tx_hex: String, signing_keys: JsVecString) -> String {
    let unsigned_transaction: csl::Transaction = csl::Transaction::from_hex(&tx_hex).unwrap();
    let tx_body = unsigned_transaction.body();
    let mut witness_set = unsigned_transaction.witness_set();
    let mut vkey_witnesses = witness_set.vkeys().unwrap().clone();
    for key in signing_keys {
        let clean_hex = if &key[0..4] == "5820" {
            key[4..].to_string()
        } else {
            key.to_string()
        };
        let skey = csl::crypto::PrivateKey::from_hex(&clean_hex).unwrap();
        let vkey_witness =
            csl::utils::make_vkey_witness(&csl::utils::hash_transaction(&tx_body), &skey);
        vkey_witnesses.add(&vkey_witness);
    }
    witness_set.set_vkeys(&vkey_witnesses);
    let signed_transaction = csl::Transaction::new(
        &tx_body,
        &witness_set,
        unsigned_transaction.auxiliary_data(),
    );
    signed_transaction.to_hex()
}

pub fn remove_witness_set(tx_hex: String) -> String {
    let signed_transaction = csl::Transaction::from_hex(&tx_hex).unwrap();
    csl::Transaction::new(
        &signed_transaction.body(),
        &csl::TransactionWitnessSet::new(),
        signed_transaction.auxiliary_data().clone(),
    )
    .to_hex()
}