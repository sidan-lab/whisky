use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DeserializedAddress {
    pub_key_hash: String,
    script_hash: String,
    stake_key_hash: String,
    stake_key_script_hash: String,
}

#[wasm_bindgen]
impl DeserializedAddress {
    pub fn new(
        pub_key_hash: &str,
        script_hash: &str,
        stake_key_hash: &str,
        stake_key_script_hash: &str,
    ) -> Self {
        Self {
            pub_key_hash: pub_key_hash.to_string(),
            script_hash: script_hash.to_string(),
            stake_key_hash: stake_key_hash.to_string(),
            stake_key_script_hash: stake_key_script_hash.to_string(),
        }
    }

    pub fn get_pub_key_hash(&self) -> String {
        self.pub_key_hash.clone()
    }

    pub fn get_script_hash(&self) -> String {
        self.script_hash.clone()
    }

    pub fn get_stake_key_hash(&self) -> String {
        self.stake_key_hash.clone()
    }

    pub fn get_stake_key_script_hash(&self) -> String {
        self.stake_key_script_hash.clone()
    }
}
