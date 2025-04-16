use bip39::{Language, Mnemonic};
use whisky_common::*;
use whisky_csl::csl;

pub struct Bip32KeyGenerator {
    pub private_key: csl::PrivateKey,
    pub public_key: csl::PublicKey,
}

impl Bip32KeyGenerator {
    pub fn from_mnemonic(mnemonic_phrase: &str) -> Result<Self, WError> {
        let mnemonic = Mnemonic::from_phrase(mnemonic_phrase, Language::English)
            .expect("Invalid mnemonic phrase");
        let entropy = mnemonic.entropy();
        let root_key = csl::Bip32PrivateKey::from_bip39_entropy(entropy, &[]);

        let hardened_key_start = 2147483648;
        let account_key = root_key
            .derive(hardened_key_start + 1852)
            .derive(hardened_key_start + 1815)
            .derive(hardened_key_start);

        let private_key = account_key.derive(0).derive(0).to_raw_key();
        let public_key = private_key.to_public();
        Ok(Bip32KeyGenerator {
            private_key,
            public_key,
        })
    }

    pub fn from_root_key(root_key: &str) -> Result<Self, WError> {
        let root_key = csl::Bip32PrivateKey::from_hex(root_key).map_err(WError::from_err(
            "from_root_key - failed to create root key",
        ))?;

        let hardened_key_start = 2147483648;
        let account_key = root_key
            .derive(hardened_key_start + 1852)
            .derive(hardened_key_start + 1815)
            .derive(hardened_key_start);

        let private_key = account_key.derive(0).derive(0).to_raw_key();
        let public_key = private_key.to_public();
        Ok(Bip32KeyGenerator {
            private_key,
            public_key,
        })
    }

    pub fn sign_transaction(&self, tx_hex: &str) -> Result<String, WError> {
        let mut tx = csl::FixedTransaction::from_hex(tx_hex).map_err(WError::from_err(
            "sign_transaction - failed to create transaction",
        ))?;
        tx.sign_and_add_vkey_signature(&self.private_key)
            .map_err(WError::from_err(
                "sign_transaction - failed to sign transaction",
            ))?;
        Ok(tx.to_hex())
    }

    pub fn get_public_key(&self) -> String {
        self.public_key.clone().to_hex()
    }
}
