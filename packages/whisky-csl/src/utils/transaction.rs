use cardano_serialization_lib::{self as csl};
use cryptoxide::blake2b::Blake2b;
use whisky_common::WError;

pub(crate) fn blake2b256(data: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    Blake2b::blake2b(&mut out, data, &[]);
    out
}

pub fn calculate_tx_hash(tx_hex: &str) -> Result<String, WError> {
    let csl_tx = csl::FixedTransaction::from_hex(tx_hex).map_err(WError::from_err(
        "calculate_tx_hash - invalid transaction hex",
    ))?;
    Ok(csl_tx.transaction_hash().to_hex())
}

pub fn sign_transaction(tx_hex: &str, signing_keys: &[&str]) -> Result<String, WError> {
    let unsigned_transaction: csl::FixedTransaction = csl::FixedTransaction::from_hex(tx_hex)
        .map_err(WError::from_err(
            "sign_transaction - invalid transaction hex",
        ))?;
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
        let skey = csl::PrivateKey::from_hex(&clean_hex).map_err(WError::from_err(
            "sign_transaction - invalid signing key hex",
        ))?;
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
        )
        .map_err(WError::from_err(
            "sign_transaction - failed to create transaction with auxiliary data",
        ))?,
        None => csl::FixedTransaction::new(
            &unsigned_transaction.raw_body(),
            &witness_set.to_bytes(),
            true,
        )
        .map_err(WError::from_err(
            "sign_transaction - failed to create transaction",
        ))?,
    };
    Ok(signed_transaction.to_hex())
}
