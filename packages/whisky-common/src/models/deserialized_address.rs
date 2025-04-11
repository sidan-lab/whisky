#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DeserializedAddress {
    pub pub_key_hash: String,
    pub script_hash: String,
    pub stake_key_hash: String,
    pub stake_key_script_hash: String,
}

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
}
