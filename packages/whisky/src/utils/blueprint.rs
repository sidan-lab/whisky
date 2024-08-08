use sidan_csl_rs::{
    core::utils::{apply_params_to_script, get_v2_script_hash, script_to_address},
    model::BuilderDataType,
};

use derive_more::Deref;

pub struct MintingBlueprint {
    pub cbor: String,
    pub hash: String,
}

impl MintingBlueprint {
    pub fn new(compiled_code: &str, params: &[&str], params_type: BuilderDataType) -> Self {
        let cbor = apply_params_to_script(compiled_code, params, params_type).unwrap();
        let hash = get_v2_script_hash(&cbor);

        Self { cbor, hash }
    }

    pub fn new_no_params(compiled_code: &str) -> Self {
        let cbor = apply_params_to_script(compiled_code, &[], BuilderDataType::CBOR).unwrap();
        let hash = get_v2_script_hash(&cbor);
        Self { cbor, hash }
    }
}

#[derive(Deref)]
pub struct WithdrawalBlueprint(MintingBlueprint);

impl WithdrawalBlueprint {
    pub fn new(compiled_code: &str, params: &[&str], params_type: BuilderDataType) -> Self {
        WithdrawalBlueprint(MintingBlueprint::new(compiled_code, params, params_type))
    }

    pub fn new_no_params(compiled_code: &str) -> Self {
        WithdrawalBlueprint(MintingBlueprint::new_no_params(compiled_code))
    }
}

pub struct SpendingBlueprint {
    pub cbor: String,
    pub hash: String,
    pub address: String,
}

impl SpendingBlueprint {
    pub fn new(
        network_id: u8,
        compiled_code: &str,
        params: &[&str],
        params_type: BuilderDataType,
        stake_hash: Option<(&str, bool)>,
    ) -> Self {
        let cbor = apply_params_to_script(compiled_code, params, params_type).unwrap();
        let hash = get_v2_script_hash(&cbor);
        let address = script_to_address(network_id, &hash, stake_hash);

        Self {
            cbor,
            hash,
            address,
        }
    }

    pub fn new_no_params(
        network_id: u8,
        compiled_code: &str,
        stake_hash: Option<(&str, bool)>,
    ) -> Self {
        let cbor = apply_params_to_script(compiled_code, &[], BuilderDataType::CBOR).unwrap();
        let hash = get_v2_script_hash(&cbor);
        let address = script_to_address(network_id, &hash, stake_hash);
        Self {
            cbor,
            hash,
            address,
        }
    }
}
