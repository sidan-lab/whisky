use super::wallet_constants::HARDENED_KEY_START;
use bip39::{Language, Mnemonic};
use whisky_common::WError;
use whisky_csl::csl::{Bip32PrivateKey, FixedTransaction, PrivateKey, PublicKey};

pub enum WalletType {
    MnemonicWallet(MnemonicWallet),
    RootKeyWallet(RootKeyWallet),
    Cli(String),
}

pub struct MnemonicWallet {
    pub mnemonic_phrase: String,
    pub derivation_indices: DerivationIndices,
}

pub struct RootKeyWallet {
    pub root_key: String,
    pub derivation_indices: DerivationIndices,
}

pub struct DerivationIndices(pub Vec<u32>);

impl Default for DerivationIndices {
    fn default() -> Self {
        DerivationIndices(vec![
            HARDENED_KEY_START + 1852, // purpose
            HARDENED_KEY_START + 1815, // coin type
            HARDENED_KEY_START,        // account
            0,                         // payment
            0,                         // key index
        ])
    }
}
pub struct Wallet {
    pub wallet_type: WalletType,
}

pub struct Account {
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
}

impl Account {
    pub fn sign_transaction(&self, tx_hex: &str) -> Result<String, WError> {
        let mut tx = FixedTransaction::from_hex(tx_hex)
            .map_err(WError::from_err("Account - failed to deserialize tx hex"))?;
        tx.sign_and_add_vkey_signature(&self.private_key)
            .map_err(WError::from_err("Account - failed to sign transaction"))?;
        Ok(tx.to_hex())
    }
}

impl Wallet {
    pub fn new(wallet_type: WalletType) -> Self {
        Self { wallet_type }
    }

    pub fn sign_tx(&self, tx_hex: &str) -> Result<String, WError> {
        let account = self
            .get_account()
            .map_err(WError::from_err("Wallet - sign_tx"))?;
        let signed_tx = account
            .sign_transaction(tx_hex)
            .map_err(WError::from_err("Wallet - sign_tx"))?;
        Ok(signed_tx.to_string())
    }

    pub fn get_account(&self) -> Result<Account, WError> {
        let private_key: PrivateKey = match &self.wallet_type {
            WalletType::MnemonicWallet(mnemonic_wallet) => {
                let mnemonic =
                    Mnemonic::from_phrase(&mnemonic_wallet.mnemonic_phrase, Language::English)
                        .map_err(WError::from_err(
                            "Wallet - get_account - failed to create mnemonic",
                        ))?;
                let entropy = mnemonic.entropy();
                let mut root_key = Bip32PrivateKey::from_bip39_entropy(entropy, &[]);
                for index in &mnemonic_wallet.derivation_indices.0 {
                    root_key = root_key.derive(index.clone());
                }
                root_key.to_raw_key()
            }
            WalletType::RootKeyWallet(root_key_wallet) => {
                let mut root_key = Bip32PrivateKey::from_bech32(&root_key_wallet.root_key)
                    .map_err(WError::from_err(
                        "Wallet - get_account - invalid root key hex",
                    ))?;
                for index in &root_key_wallet.derivation_indices.0 {
                    root_key = root_key.derive(index.clone());
                }
                root_key.to_raw_key()
            }
            WalletType::Cli(private_key) => PrivateKey::from_hex(&private_key).map_err(
                WError::from_err("Wallet - get_account - invalid private key hex"),
            )?,
        };
        let public_key = private_key.to_public();
        Ok(Account {
            private_key,
            public_key,
        })
    }
}
