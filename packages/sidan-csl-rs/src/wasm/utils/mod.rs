use crate::{core::serializer::script_to_address, *};

#[wasm_bindgen]
pub fn wasm_script_to_address(
    network_id: u8,
    script_hash: String,
    stake_hash: Option<String>,
    is_script_stake_key: bool,
) -> String {
    match stake_hash {
        Some(stake) => script_to_address(
            network_id,
            &script_hash,
            Some((&stake, is_script_stake_key)),
        ),
        None => script_to_address(network_id, &script_hash, None),
    }
}
