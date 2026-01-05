use pallas::{
    codec::utils::{Bytes, NonEmptySet},
    ledger::primitives::conway::VKeyWitness,
};

pub fn required_signatures_to_mock_witnesses(
    required_signatures: Vec<String>,
) -> Option<NonEmptySet<VKeyWitness>> {
    if required_signatures.is_empty() {
        return None;
    }
    let required_signature_set: NonEmptySet<String> =
        NonEmptySet::from_vec(required_signatures).unwrap();
    let mut vkey_witnesses = Vec::new();

    for (i, _signer_hash) in required_signature_set.into_iter().enumerate() {
        let mock_vkey = vec![i as u8; 32];
        let mock_signature = vec![i as u8; 64];
        let vkey_witness = VKeyWitness {
            vkey: Bytes::from(mock_vkey),           // Mock public key
            signature: Bytes::from(mock_signature), // Mock signature
        };
        vkey_witnesses.push(vkey_witness);
    }

    Some(NonEmptySet::from_vec(vkey_witnesses).unwrap())
}
