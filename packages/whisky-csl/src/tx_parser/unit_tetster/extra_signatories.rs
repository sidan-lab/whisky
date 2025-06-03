use super::TxTester;

impl TxTester {
    pub fn key_signed(&self, key_hash: &str) -> bool {
        self.tx_body
            .required_signatures
            .iter()
            .any(|signatory| signatory == key_hash)
    }

    pub fn one_of_keys_signed(&self, key_hashes: &[String]) -> bool {
        key_hashes.iter().any(|key_hash| self.key_signed(key_hash))
    }

    pub fn all_keys_signed(&self, key_hashes: &[String]) -> bool {
        key_hashes.iter().all(|key_hash| self.key_signed(key_hash))
    }
}
