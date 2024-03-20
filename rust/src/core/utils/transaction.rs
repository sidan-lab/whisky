use crate::*;
use cardano_serialization_lib as csl;
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

#[test]
fn test_calculate_tx_hash() {
    let tx_hex = "84a30081825820cc24e6f228e04d98c80088c830a363fff80a2437959f826e1a5b4c01ec912d0f010182a200581d605ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa011a001c0242a200581d601fd5bab167338971d92b4d8f0bdf57d889903e6e934e7ea38c7dadf1011b0000000252fe47ac021a00028759a100818258201557f444f3ae6e61dfed593ae15ec8dbd57b8138972bf16fde5b4c559f41549b5840b8317b840d4e908cd6a69bad0d294a593a40812749ccacdea993c660952a57cdf89428934973848a1437820b9f0e5784ddc01eb049415d4189977fdc32fda904f5f6";
    let tx_hash = calculate_tx_hash(tx_hex);
    assert_eq!(
        tx_hash,
        "c162f8abf8405b1d7f8f7677bc391b2d8f1911e73035cb97634b2dede72404cf"
    )
}

#[test]
fn test_calculate_tx_hash_2() {
    let tx_hex = "84a400828258200f88c351c8afb3494b70dc2128e61289ea279fee7516db2c58e1562ce8576bbd028258208bbb363df8e0bcadf6b4ac473a06d94d75be243e0772ffbfc34571ea39873a5c000182a3005839008f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f01821a0012593aa1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a00989680028201d81843d87980825839008f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f1ab51927b1021a0002a04509a0a0f5f6";
    let signed_tx_hex = "84a400828258200f88c351c8afb3494b70dc2128e61289ea279fee7516db2c58e1562ce8576bbd028258208bbb363df8e0bcadf6b4ac473a06d94d75be243e0772ffbfc34571ea39873a5c000182a3005839008f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f01821a0012593aa1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a00989680028201d81843d87980825839008f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f1ab51927b1021a0002a04509a0a10081825820eb125c9530b870bab17f5f30dcbf029929c4d8743e4eaaf71a5e883d41a236ce5840bc63f18abf97e386743b00baf8e829a73e19bf600c8bdfc0e53d14c5171c7f6d62adc5a7081b06465d7003641ec2406421316424d216e06605323ebc68c1600cf5f6";
    let tx_hash_from_unsigned_hex = calculate_tx_hash(tx_hex);
    let tx_hash_from_signed_hex = calculate_tx_hash(signed_tx_hex);
    assert_eq!(
        tx_hash_from_unsigned_hex,
        "e8b7aefcee2953cf55a01c97565cfe9d414a21e17064d8fcef1f632f7311f933"
    );
    assert_eq!(
        tx_hash_from_signed_hex,
        "e8b7aefcee2953cf55a01c97565cfe9d414a21e17064d8fcef1f632f7311f933"
    )
}

#[wasm_bindgen]
pub fn sign_transaction(tx_hex: String, signing_keys: Vec<String>) -> String {
    let mut vkey_witnesses = csl::crypto::Vkeywitnesses::new();
    let unsigned_transaction: csl::Transaction = csl::Transaction::from_hex(&tx_hex).unwrap();
    let tx_body = unsigned_transaction.body();
    for key in signing_keys {
        let clean_hex = if &key[0..4] == "5820" {
            key[4..].to_string()
        } else {
            key
        };
        let skey = csl::crypto::PrivateKey::from_hex(&clean_hex).unwrap();
        let vkey_witness =
            csl::utils::make_vkey_witness(&csl::utils::hash_transaction(&tx_body), &skey);
        vkey_witnesses.add(&vkey_witness);
    }
    let mut witness_set = unsigned_transaction.witness_set();
    witness_set.set_vkeys(&vkey_witnesses);
    let signed_transaction = csl::Transaction::new(
        &tx_body,
        &witness_set,
        unsigned_transaction.auxiliary_data(),
    );
    signed_transaction.to_hex()
}
