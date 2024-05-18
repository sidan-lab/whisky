use crate::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SerializedAddress {
    pub_key_hash: String,
    script_hash: String,
    stake_key_hash: String,
}

#[wasm_bindgen]
impl SerializedAddress {
    pub fn new(pub_key_hash: String, script_hash: String, stake_key_hash: String) -> Self {
        Self {
            pub_key_hash,
            script_hash,
            stake_key_hash,
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
}
