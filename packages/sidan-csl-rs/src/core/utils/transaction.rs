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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_tx_hash() {
        let tx_hex = "84a800828258205bc0b1cbf366f26110a5535437559907f3d638b7e52ebe4d3e2b4461a6fe419c07825820ce762730834d729b3c094a33da9fc95ae101d34282929b2f4944505e2bee9b77050181a3005839100d1e1d4ffa6006438e33769d25de0ece14797e6f84cacae3040d58ea5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a00abe5d5a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a05f5e100028201d8185862d8799fd8799fd87a9f581cde3dd9cde94d805da0cdce78bc6374a6a518a84c875d260f5062fa8dffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd87a801a00061a801a0ee6b28000ff021a0003940b05a1581df012660ed0257fd0b0786cf64d032232502b6b39616bdf095ba2c38d9e000b58200124157b58b5ff78c3c68d46ad889d1735d7cdb7ff614f610143debf65389b160d818258203fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814070e82581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604ae581c5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa1283825820ce762730834d729b3c094a33da9fc95ae101d34282929b2f4944505e2bee9b7700825820ce762730834d729b3c094a33da9fc95ae101d34282929b2f4944505e2bee9b77018258209bafe5234c4de6fedd980b977e76ece47901f41fa7084d47d25c26e1bbe3035700a20082825820aa8ce9e908f525c3b700a65669430ec68ca19615e7309e25bb6fa883964cfa9f58405f06fb680ed1822383d988e185929a8bd268a025ac2b2e8a18acb02bff5785ff76abbd23f881d8fb4107ae92c53b7e7712e7c3d11cfd34f0cfef44206556370e8258207f4747ca0c20a1e5c28716c4a10fffbcbe8fe6253cb427ae2f0e24d231a9808458404754e7ed68fa85ac14dd8fc19d562b2bce6e86bac398e681013d3713238d25afc168ed59ee4eacef7d677e11d86b8ba9cb8401b2c390701710c763ab189063040583840000d879808219836b1a00a18165840001d879808219836b1a00a18165840300d87980821a0001e4681a02392dd5f5f6";
        let tx_hash = calculate_tx_hash(tx_hex).unwrap();
        assert_eq!(
            tx_hash,
            "174ed92a1c005606b8d814b8e921a8f03666ffb16cfcb82459770f918ddafc25"
        );
    }
}
