pub mod derivation_indices;
pub mod mnemonic;
pub mod root_key;
use bip39::{Language, Mnemonic};
use derivation_indices::DerivationIndices;
pub use mnemonic::MnemonicWallet;
pub use root_key::RootKeyWallet;
use whisky_common::{Fetcher, Submitter, WError};
use whisky_csl::{
    csl::{
        BaseAddress, Bip32PrivateKey, Credential, Ed25519KeyHash, FixedTransaction, PrivateKey,
        PublicKey,
    },
    sign_transaction,
};

#[derive(Copy, Clone)]
pub enum NetworkId {
    Preprod = 0, // Default
    Mainnet = 1,
}

pub enum AddressType {
    Enterprise,
    Payment,
}

pub enum WalletType {
    MnemonicWallet(MnemonicWallet),
    RootKeyWallet(RootKeyWallet),
    Cli(String),
}

pub struct Wallet {
    pub wallet_type: WalletType,
    pub network_id: NetworkId,
    pub fetcher: Option<Box<dyn Fetcher>>,
    pub submitter: Option<Box<dyn Submitter>>,
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
        Self {
            wallet_type,
            network_id: NetworkId::Preprod,
            fetcher: None,
            submitter: None,
        }
    }

    pub fn new_cli(cli_skey: &str) -> Self {
        Self {
            wallet_type: WalletType::Cli(cli_skey.to_string()),
            network_id: NetworkId::Preprod,
            fetcher: None,
            submitter: None,
        }
    }

    pub fn new_mnemonic(mnemonic_phrase: &str) -> Self {
        Self {
            wallet_type: WalletType::MnemonicWallet(MnemonicWallet {
                mnemonic_phrase: mnemonic_phrase.to_string(),
                derivation_indices: DerivationIndices::default(),
            }),
            network_id: NetworkId::Preprod,
            fetcher: None,
            submitter: None,
        }
    }

    pub fn new_root_key(root_key: &str) -> Self {
        Self {
            wallet_type: WalletType::RootKeyWallet(RootKeyWallet {
                root_key: root_key.to_string(),
                derivation_indices: DerivationIndices::default(),
            }),
            network_id: NetworkId::Preprod,
            fetcher: None,
            submitter: None,
        }
    }

    pub fn with_network_id(mut self, network_id: NetworkId) -> Self {
        self.network_id = network_id;
        self
    }

    pub fn with_fetcher<F: Fetcher + 'static>(mut self, fetcher: F) -> Self {
        self.fetcher = Some(Box::new(fetcher));
        self
    }

    pub fn with_submitter<S: Submitter + 'static>(mut self, submitter: S) -> Self {
        self.submitter = Some(Box::new(submitter));
        self
    }

    pub fn payment_account(&mut self, account_index: u32, key_index: u32) -> &mut Self {
        match &mut self.wallet_type {
            WalletType::MnemonicWallet(mnemonic_wallet) => {
                mnemonic_wallet.payment_account(account_index, key_index);
            }
            WalletType::RootKeyWallet(root_key_wallet) => {
                root_key_wallet.payment_account(account_index, key_index);
            }
            _ => {}
        }
        self
    }

    pub fn stake_account(&mut self, account_index: u32, key_index: u32) -> &mut Self {
        match &mut self.wallet_type {
            WalletType::MnemonicWallet(mnemonic_wallet) => {
                mnemonic_wallet.stake_account(account_index, key_index);
            }
            WalletType::RootKeyWallet(root_key_wallet) => {
                root_key_wallet.stake_account(account_index, key_index);
            }
            _ => {}
        }
        self
    }

    pub fn drep_account(&mut self, account_index: u32, key_index: u32) -> &mut Self {
        match &mut self.wallet_type {
            WalletType::MnemonicWallet(mnemonic_wallet) => {
                mnemonic_wallet.drep_account(account_index, key_index);
            }
            WalletType::RootKeyWallet(root_key_wallet) => {
                root_key_wallet.drep_account(account_index, key_index);
            }
            _ => {}
        }
        self
    }

    pub fn sign_tx(&self, tx_hex: &str) -> Result<String, WError> {
        match &self.wallet_type {
            WalletType::Cli(cli_skey) => {
                let signed_tx = sign_transaction(tx_hex, &[cli_skey])
                    .map_err(WError::from_err("Wallet - sign_tx"))?;
                Ok(signed_tx)
            }
            _ => {
                let account = self
                    .get_account()
                    .map_err(WError::from_err("Wallet - sign_tx"))?;
                let signed_tx = account
                    .sign_transaction(tx_hex)
                    .map_err(WError::from_err("Wallet - sign_tx"))?;
                Ok(signed_tx.to_string())
            }
        }
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

    pub fn get_address_with_params(
        &mut self,
        account_index: u32,
        key_index: u32,
        address_type: AddressType,
        stake_credential: Option<&str>,
    ) -> Result<String, WError> {
        let payment_account = self
            .payment_account(account_index, key_index)
            .get_account()?;
        let payment_credential = Credential::from_keyhash(&payment_account.public_key.hash());

        let network_id_value = self.network_id as u8;

        match address_type {
            AddressType::Payment => {
                let stake_credential = if let Some(stake_cred_str) = stake_credential {
                    let stake_key_hash = Ed25519KeyHash::from_hex(stake_cred_str)
                        .map_err(WError::from_err("Invalid stake credential format"))?;
                    Credential::from_keyhash(&stake_key_hash)
                } else {
                    let stake_account =
                        self.stake_account(account_index, key_index).get_account()?;

                    Credential::from_keyhash(&stake_account.public_key.hash())
                };
                let base_address =
                    BaseAddress::new(network_id_value, &payment_credential, &stake_credential);
                let address = base_address.to_address();
                address.to_bech32(None).map_err(WError::from_err(
                    "Failed to convert payment address to bech32",
                ))
            }
            AddressType::Enterprise => {
                let enterprise_address =
                    whisky_csl::csl::EnterpriseAddress::new(network_id_value, &payment_credential);
                let address = enterprise_address.to_address();
                address.to_bech32(None).map_err(WError::from_err(
                    "Failed to convert enterprise address to bech32",
                ))
            }
        }
    }
}
