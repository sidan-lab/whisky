use crate::{csl, *};
use cardano_serialization_lib::JsError;
use cryptoxide::blake2b::Blake2b;

pub(crate) fn blake2b256(data: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

pub fn calculate_tx_hash(tx_hex: &str) -> Result<String, JsError> {
    let csl_tx = csl::FixedTransaction::from_hex(tx_hex)?;
    Ok(csl::TransactionHash::from(blake2b256(&csl_tx.raw_body())).to_hex())
}

pub fn sign_transaction(tx_hex: &str, signing_keys: &[&str]) -> Result<String, JsError> {
    let unsigned_transaction: csl::FixedTransaction = csl::FixedTransaction::from_hex(tx_hex)?;
    let mut witness_set = unsigned_transaction.witness_set();
    let mut vkey_witnesses = witness_set
        .vkeys()
        .unwrap_or_else(csl::Vkeywitnesses::new)
        .clone();
    for key in signing_keys {
        let clean_hex = if &key[0..4] == "5820" && key.len() == 68 {
            key[4..].to_string()
        } else {
            key.to_string()
        };
        let skey = csl::PrivateKey::from_hex(&clean_hex)?;
        let vkey_witness = csl::make_vkey_witness(
            &csl::TransactionHash::from(blake2b256(&unsigned_transaction.raw_body())),
            &skey,
        );
        vkey_witnesses.add(&vkey_witness);
    }
    witness_set.set_vkeys(&vkey_witnesses);
    let signed_transaction: csl::FixedTransaction = match &unsigned_transaction.raw_auxiliary_data()
    {
        Some(raw_auxiliary_data) => csl::FixedTransaction::new_with_auxiliary(
            &unsigned_transaction.raw_body(),
            &unsigned_transaction.raw_witness_set(),
            raw_auxiliary_data,
            true,
        )?,
        None => csl::FixedTransaction::new(
            &unsigned_transaction.raw_body(),
            &witness_set.to_bytes(),
            true,
        )?,
    };
    Ok(signed_transaction.to_hex())
}

// #[test]
// fn test_private_key_parsing() {
//     let key = "f89081fcf711b55722c26d2734b6a324dce4976849e2feb03a3f5ab595bd987b";
//     let skey = csl::PrivateKey::from_hex(key).unwrap();
//     println!("Pub key {:?}", skey.to_public().to_hex());
//     println!("Pub key hash {:?}", skey.to_public().hash().to_hex());
// }

#[wasm_bindgen]
pub fn remove_witness_set(tx_hex: String) -> String {
    let signed_transaction = csl::FixedTransaction::from_hex(&tx_hex).unwrap();
    let unsigned_transaction: csl::FixedTransaction = match &signed_transaction.raw_auxiliary_data()
    {
        Some(raw_auxiliary_data) => csl::FixedTransaction::new_with_auxiliary(
            &signed_transaction.raw_body(),
            &csl::TransactionWitnessSet::new().to_bytes(),
            raw_auxiliary_data,
            true,
        )
        .unwrap(),
        None => csl::FixedTransaction::new(
            &signed_transaction.raw_body(),
            &csl::TransactionWitnessSet::new().to_bytes(),
            true,
        )
        .unwrap(),
    };
    unsigned_transaction.to_hex()
}

#[wasm_bindgen]
pub fn merge_vkey_witnesses_to_transaction(
    tx_hex: String,
    added_witness_set_hex: String,
) -> String {
    let unsigned_transaction: csl::FixedTransaction =
        csl::FixedTransaction::from_hex(&tx_hex).unwrap();
    let mut witness_set = unsigned_transaction.witness_set();
    let mut vkey_witnesses = witness_set
        .vkeys()
        .unwrap_or_else(csl::Vkeywitnesses::new)
        .clone();
    let added_vkey_witnesses = csl::TransactionWitnessSet::from_hex(&added_witness_set_hex)
        .unwrap()
        .vkeys()
        .expect("Expected vkeys to add to transaction");

    for index in 0..added_vkey_witnesses.len() {
        vkey_witnesses.add(&added_vkey_witnesses.get(index));
    }
    witness_set.set_vkeys(&vkey_witnesses);
    let signed_transaction: csl::FixedTransaction = match &unsigned_transaction.raw_auxiliary_data()
    {
        Some(raw_auxiliary_data) => csl::FixedTransaction::new_with_auxiliary(
            &unsigned_transaction.raw_body(),
            &unsigned_transaction.raw_witness_set(),
            raw_auxiliary_data,
            true,
        )
        .unwrap(),
        None => csl::FixedTransaction::new(
            &unsigned_transaction.raw_body(),
            &witness_set.to_bytes(),
            true,
        )
        .unwrap(),
    };
    signed_transaction.to_hex()
}
