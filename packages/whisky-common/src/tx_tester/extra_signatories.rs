use super::TxTester;

impl TxTester {
    /// ## Testing methods for required signers
    ///
    /// Checks if a specific key is signed in the transaction.
    pub fn key_signed(&mut self, key_hash: &str) -> &mut Self {
        let is_key_signed = self.key_signed_logic(key_hash);
        if !is_key_signed {
            self.add_trace(
                "key_signed",
                &format!("tx does not have key {} signed", key_hash),
            )
        };
        self
    }

    /// ## Testing methods for required signers
    ///
    /// Checks if any one of the specified keys is signed in the transaction.
    pub fn one_of_keys_signed(&mut self, key_hashes: &[String]) -> &mut Self {
        let is_one_of_keys_signed = key_hashes
            .iter()
            .any(|key_hash| self.key_signed_logic(key_hash));
        if !is_one_of_keys_signed {
            self.add_trace(
                "one_of_keys_signed",
                &format!(
                    "tx does not have any of the keys signed:
                        {}",
                    key_hashes.join(", ")
                ),
            );
        }
        self
    }

    /// ## Testing methods for required signers
    ///
    /// Checks if all specified keys are signed in the transaction.
    pub fn all_keys_signed(&mut self, key_hashes: &[String]) -> &mut Self {
        let mut missing_keys: Vec<String> = vec![];
        let is_all_keys_signed = key_hashes.iter().all(|key_hash| {
            let is_key_signed = self.key_signed_logic(key_hash);
            if !is_key_signed {
                missing_keys.push(key_hash.clone());
            }
            is_key_signed
        });

        if !is_all_keys_signed {
            self.add_trace(
                "all_keys_signed",
                &format!(
                    "tx does not have all keys signed: {}",
                    missing_keys.join(", ")
                ),
            );
        }
        self
    }

    fn key_signed_logic(&mut self, key_hash: &str) -> bool {
        self.tx_body
            .required_signatures
            .iter()
            .any(|signatory| signatory == key_hash)
    }
}
