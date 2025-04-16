mod bip39;

use ::bip39::{Language, Mnemonic};
pub use bip39::*;
use whisky_common::WError;
use whisky_csl::csl::{self, Bip32PrivateKey};

pub enum WalletType {
    Mnemonic(String),
    Root(String),
    Cli(String),
}

pub struct Wallet {
    pub wallet_type: WalletType,
    pub account_index: u32,
    pub key_index: u32,
}

impl Wallet {
    pub fn new(wallet_type: WalletType, account_index: u32, key_index: u32) -> Self {
        Self {
            wallet_type,
            account_index,
            key_index,
        }
    }

    pub fn sign_tx(&self, tx_hex: &str) -> Result<String, WError> {
        let root_key = self
            .get_root_key()
            .map_err(WError::from_err("Wallet - sign_tx"))?;
        let account = self
            .get_account(root_key, self.account_index, self.key_index)
            .map_err(WError::from_err("Wallet - sign_tx"))?;
        let signed_tx = account
            .sign_transaction(tx_hex)
            .map_err(WError::from_err("Wallet - sign_tx"))?;
        Ok(signed_tx.to_string())
    }

    pub fn get_root_key(&self) -> Result<Bip32PrivateKey, WError> {
        match &self.wallet_type {
            WalletType::Mnemonic(mnemonic_phrase) => {
                let mnemonic = Mnemonic::from_phrase(mnemonic_phrase, Language::English).map_err(
                    WError::from_err("Wallet - get_root_key - failed to create mnemonic"),
                )?;
                let entropy = mnemonic.entropy();
                let root_key = csl::Bip32PrivateKey::from_bip39_entropy(entropy, &[]);
                Ok(root_key)
            }
            WalletType::Root(root_key) => {
                let root_key = csl::Bip32PrivateKey::from_bech32(root_key).map_err(
                    WError::from_err("Wallet - from_root_key - failed to create root key"),
                )?;
                Ok(root_key)
            }
            WalletType::Cli(_) => Err(WError::new(
                "Wallet - get_root_key",
                "CLI wallet type not supported",
            )),
        }
    }

    pub fn get_account(
        &self,
        root_key: Bip32PrivateKey,
        account_index: u32,
        key_index: u32,
    ) -> Result<Bip32KeyGenerator, WError> {
        let hardened_key_start = 2147483648;
        let account_key = root_key
            .derive(hardened_key_start + 1852)
            .derive(hardened_key_start + 1815)
            .derive(hardened_key_start);

        let private_key = account_key
            .derive(account_index)
            .derive(key_index)
            .to_raw_key();
        let public_key = private_key.to_public();
        Ok(Bip32KeyGenerator {
            private_key,
            public_key,
        })
    }
}
