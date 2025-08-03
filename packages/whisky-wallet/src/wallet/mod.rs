pub mod derivation_indices;
pub mod mnemonic;
pub mod root_key;

use bip39::{Language, Mnemonic};
use derivation_indices::DerivationIndices;
pub use mnemonic::MnemonicWallet;
pub use root_key::RootKeyWallet;
use whisky_common::{Fetcher, Submitter, UTxO, WError};
use whisky_csl::{
    csl::{
        BaseAddress, Bip32PrivateKey, Credential, EnterpriseAddress, FixedTransaction, PrivateKey,
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

/// Represents a Cardano wallet.
///
/// A wallet manages addresses and cryptographic keys needed for transaction
/// signing and verification. It supports different wallet types, including
/// mnemonic-based, root key-based, and CLI-based wallets.
pub struct Wallet {
    pub wallet_type: WalletType,
    pub network_id: NetworkId,
    pub addresses: Addresses,
    pub fetcher: Option<Box<dyn Fetcher>>,
    pub submitter: Option<Box<dyn Submitter>>,
}
pub struct Addresses {
    pub base_address: Option<BaseAddress>,
    pub enterprise_address: Option<EnterpriseAddress>,
}

pub struct Account {
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
}

impl Account {
    /// Signs a transaction using the account's private key.
    ///
    /// # Arguments
    ///
    /// * `tx_hex` - The transaction to sign in hexadecimal format
    ///
    /// # Returns
    ///
    /// A Result containing either the signed transaction in hexadecimal format or an error
    pub fn sign_transaction(&self, tx_hex: &str) -> Result<String, WError> {
        let mut tx = FixedTransaction::from_hex(tx_hex)
            .map_err(WError::from_err("Account - failed to deserialize tx hex"))?;
        tx.sign_and_add_vkey_signature(&self.private_key)
            .map_err(WError::from_err("Account - failed to sign transaction"))?;
        Ok(tx.to_hex())
    }
}

impl Wallet {
    // Private helper method for basic wallet initialization
    fn empty(wallet_type: WalletType) -> Self {
        Self {
            wallet_type,
            network_id: NetworkId::Preprod,
            addresses: Addresses {
                base_address: None,
                enterprise_address: None,
            },
            fetcher: None,
            submitter: None,
        }
    }

    /// Creates a new wallet with the specified wallet type.
    ///
    /// This is a generic constructor that initializes addresses based on the wallet type.
    ///
    /// # Arguments
    ///
    /// * `wallet_type` - The type of wallet to create
    ///
    /// # Returns
    ///
    /// A new `Wallet` instance with initialized addresses
    pub fn new(wallet_type: WalletType) -> Self {
        let mut wallet = Self::empty(wallet_type);
        wallet.init_addresses();
        wallet
    }

    /// Creates a new CLI-based wallet using the provided signing key.
    ///
    /// # Arguments
    ///
    /// * `cli_skey` - The signing key string in hex format
    ///
    /// # Returns
    ///
    /// A new `Wallet` instance
    pub fn new_cli(cli_skey: &str) -> Self {
        let mut wallet = Self::empty(WalletType::Cli(cli_skey.to_string()));
        wallet.init_addresses();
        wallet
    }

    /// Creates a new mnemonic-based wallet using the provided mnemonic phrase.
    ///
    /// # Arguments
    ///
    /// * `mnemonic_phrase` - The BIP39 mnemonic phrase
    ///
    /// # Returns
    ///
    /// A new `Wallet` instance with initialized addresses
    pub fn new_mnemonic(mnemonic_phrase: &str) -> Self {
        let mut wallet = Self::empty(WalletType::MnemonicWallet(MnemonicWallet {
            mnemonic_phrase: mnemonic_phrase.to_string(),
            derivation_indices: DerivationIndices::default(),
        }));
        wallet.init_addresses();
        wallet
    }

    /// Creates a new root key-based wallet using the provided root key.
    ///
    /// # Arguments
    ///
    /// * `root_key` - The bech32-encoded root key
    ///
    /// # Returns
    ///
    /// A new `Wallet` instance with initialized addresses
    pub fn new_root_key(root_key: &str) -> Self {
        let mut wallet = Self::empty(WalletType::RootKeyWallet(RootKeyWallet {
            root_key: root_key.to_string(),
            derivation_indices: DerivationIndices::default(),
        }));
        wallet.init_addresses();
        wallet
    }

    /// Sets the network ID for the wallet and reinitializes addresses.
    ///
    /// # Arguments
    ///
    /// * `network_id` - The network ID to use (Preprod or Mainnet)
    ///
    /// # Returns
    ///
    /// The updated wallet with reinitialized addresses
    pub fn with_network_id(mut self, network_id: NetworkId) -> Self {
        self.network_id = network_id;
        self.init_addresses();
        self
    }

    /// Attaches a fetcher implementation to the wallet.
    ///
    /// A fetcher is used to fetch UTxOs and other blockchain data.
    ///
    /// # Arguments
    ///
    /// * `fetcher` - The fetcher implementation to use
    ///
    /// # Returns
    ///
    /// The updated wallet with fetcher capability
    pub fn with_fetcher<F: Fetcher + 'static>(mut self, fetcher: F) -> Self {
        self.fetcher = Some(Box::new(fetcher));
        self
    }

    /// Attaches a submitter implementation to the wallet.
    ///
    /// A submitter is used to submit transactions to the blockchain.
    ///
    /// # Arguments
    ///
    /// * `submitter` - The submitter implementation to use
    ///
    /// # Returns
    ///
    /// The updated wallet with submitter capability
    pub fn with_submitter<S: Submitter + 'static>(mut self, submitter: S) -> Self {
        self.submitter = Some(Box::new(submitter));
        self
    }

    /// Sets the payment account indices for the wallet.
    ///
    /// This updates the derivation path for the payment address.
    ///
    /// # Arguments
    ///
    /// * `account_index` - The account index to use
    /// * `key_index` - The key index to use
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
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
        self.init_addresses()
    }

    /// Sets the stake account indices for the wallet.
    ///
    /// This updates the derivation path for the stake address.
    ///
    /// # Arguments
    ///
    /// * `account_index` - The account index to use
    /// * `key_index` - The key index to use
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
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
        self.init_addresses()
    }

    /// Sets the delegation representative (DRep) account indices for the wallet.
    ///
    /// This updates the derivation path for the DRep address.
    ///
    /// # Arguments
    ///
    /// * `account_index` - The account index to use
    /// * `key_index` - The key index to use
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
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
        self.init_addresses()
    }

    /// Initializes or re-initializes wallet addresses based on the wallet type and current network ID.
    ///
    /// This method generates base and enterprise addresses for mnemonic and root key wallets.
    /// It's automatically called when constructing a wallet or when changing wallet parameters
    /// that affect addresses.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Panics
    ///
    /// May panic if there are issues creating a mnemonic or decoding a root key.
    /// Consider using a version that returns Result instead if you need to handle these errors.
    pub fn init_addresses(&mut self) -> &mut Self {
        self.addresses = match &self.wallet_type {
            WalletType::MnemonicWallet(mnemonic_wallet) => {
                let mnemonic =
                    Mnemonic::from_phrase(&mnemonic_wallet.mnemonic_phrase, Language::English)
                        .map_err(WError::from_err(
                            "Wallet - init_addresses - failed to create mnemonic",
                        ))
                        .unwrap();
                let entropy = mnemonic.entropy();
                let mut root_key = Bip32PrivateKey::from_bip39_entropy(entropy, &[]);
                for index in mnemonic_wallet.derivation_indices.0.iter().take(3) {
                    root_key = root_key.derive(index.clone());
                }

                let payment_credential = Credential::from_keyhash(
                    &root_key
                        .derive(mnemonic_wallet.derivation_indices.0[3].clone())
                        .derive(mnemonic_wallet.derivation_indices.0[4].clone())
                        .to_public()
                        .to_raw_key()
                        .hash(),
                );

                let stake_credential = Credential::from_keyhash(
                    &root_key.derive(2).derive(0).to_public().to_raw_key().hash(),
                );

                self.create_addresses(payment_credential, stake_credential)
            }
            WalletType::RootKeyWallet(root_key_wallet) => {
                let mut root_key = Bip32PrivateKey::from_bech32(&root_key_wallet.root_key)
                    .map_err(WError::from_err(
                        "Wallet - init_addresses - invalid root key hex",
                    ))
                    .unwrap();
                for index in root_key_wallet.derivation_indices.0.iter().take(3) {
                    root_key = root_key.derive(index.clone());
                }

                let payment_credential = Credential::from_keyhash(
                    &root_key
                        .derive(root_key_wallet.derivation_indices.0[3].clone())
                        .derive(root_key_wallet.derivation_indices.0[4].clone())
                        .to_public()
                        .to_raw_key()
                        .hash(),
                );

                let stake_credential = Credential::from_keyhash(
                    &root_key.derive(2).derive(0).to_public().to_raw_key().hash(),
                );

                self.create_addresses(payment_credential, stake_credential)
            }
            WalletType::Cli(_private_key) => Addresses {
                base_address: None,
                enterprise_address: None,
            },
        };
        self
    }

    /// Helper method to create addresses from payment and stake credentials.
    /// This reduces code duplication between wallet types.
    fn create_addresses(
        &self,
        payment_credential: Credential,
        stake_credential: Credential,
    ) -> Addresses {
        Addresses {
            base_address: Some(BaseAddress::new(
                self.network_id as u8,
                &payment_credential,
                &stake_credential,
            )),
            enterprise_address: Some(EnterpriseAddress::new(
                self.network_id as u8,
                &payment_credential,
            )),
        }
    }

    /// Gets the account private and public keys based on the wallet type and derivation indices.
    ///
    /// # Returns
    ///
    /// A Result containing either an Account with the private and public keys or an error
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

    /// Signs a transaction using the wallet's private key.
    ///
    /// # Arguments
    ///
    /// * `tx_hex` - The transaction to sign in hexadecimal format
    ///
    /// # Returns
    ///
    /// A Result containing either the signed transaction in hexadecimal format or an error
    pub fn sign_tx(&self, tx_hex: &str) -> Result<String, WError> {
        match &self.wallet_type {
            WalletType::Cli(cli_skey) => {
                let signed_tx = sign_transaction(tx_hex, &[cli_skey]).map_err(WError::from_err(
                    "Wallet - sign_tx - failed to sign with CLI key",
                ))?;
                Ok(signed_tx)
            }
            _ => {
                let account = self
                    .get_account()
                    .map_err(WError::from_err("Wallet - sign_tx - failed to get account"))?;
                let signed_tx = account.sign_transaction(tx_hex).map_err(WError::from_err(
                    "Wallet - sign_tx - failed to sign transaction",
                ))?;
                Ok(signed_tx.to_string())
            }
        }
    }

    /// Gets a wallet address based on the specified address type.
    ///
    /// # Arguments
    ///
    /// * `address_type` - The type of address to get (Payment or Enterprise)
    ///
    /// # Returns
    ///
    /// A Result containing either the bech32-encoded address or an error
    pub fn get_change_address(&self, address_type: AddressType) -> Result<String, WError> {
        match address_type {
            AddressType::Payment => {
                if let Some(base_address) = &self.addresses.base_address {
                    let address = base_address.to_address();
                    address.to_bech32(None).map_err(WError::from_err(
                        "Failed to convert payment address to bech32",
                    ))
                } else {
                    Err(WError::from_err(
                        "Base address not available for this wallet type",
                    )("Base address not initialized"))
                }
            }
            AddressType::Enterprise => {
                if let Some(enterprise_address) = &self.addresses.enterprise_address {
                    let address = enterprise_address.to_address();
                    address.to_bech32(None).map_err(WError::from_err(
                        "Failed to convert enterprise address to bech32",
                    ))
                } else {
                    Err(WError::from_err(
                        "Enterprise address not available for this wallet type",
                    )("Enterprise address not initialized"))
                }
            }
        }
    }

    /// Fetches unspent transaction outputs (UTxOs) for a wallet address.
    ///
    /// # Arguments
    ///
    /// * `address_type` - Optional address type (Payment or Enterprise). Defaults to Payment if not specified.
    /// * `asset` - Optional asset ID to filter UTxOs. If specified, only UTxOs containing the asset will be returned.
    ///
    /// # Returns
    ///
    /// A Result containing either a vector of UTxOs or an error
    ///
    /// # Errors
    ///
    /// Returns an error if no fetcher is configured or if there's an issue getting the address or fetching UTxOs.
    pub async fn get_utxos(
        &self,
        address_type: Option<AddressType>,
        asset: Option<&str>,
    ) -> Result<Vec<UTxO>, WError> {
        let fetcher = self.fetcher.as_ref().ok_or_else(|| {
            WError::from_err("Fetcher is required to fetch UTxOs. Please provide a fetcher.")(
                "No fetcher provided",
            )
        })?;

        let address_type = address_type.unwrap_or(AddressType::Payment);
        let address = self.get_change_address(address_type)?;

        fetcher
            .fetch_address_utxos(&address, asset)
            .await
            .map_err(WError::from_err("Failed to fetch UTxOs"))
    }

    /// Fetches suitable collateral UTXOs from the wallet.
    ///
    /// Collateral UTXOs must:
    /// 1. Contain only lovelace (no other assets)
    /// 2. Have at least 5,000,000 lovelace (5 ADA)
    ///
    /// This method returns the smallest suitable UTxO to minimize locked collateral.
    ///
    /// # Arguments
    ///
    /// * `address_type` - Optional address type to fetch UTXOs from. Defaults to Payment.
    ///
    /// # Returns
    ///
    /// A Result containing either a vector with the smallest suitable collateral UTxO,
    /// or an empty vector if no suitable UTxO is found, or an error.
    pub async fn get_collateral(
        &self,
        address_type: Option<AddressType>,
    ) -> Result<Vec<UTxO>, WError> {
        let address_type = address_type.unwrap_or(AddressType::Payment);
        let utxos = self.get_utxos(Some(address_type), None).await?;

        let mut collateral_candidates: Vec<UTxO> = utxos
            .into_iter()
            .filter(|utxo| {
                utxo.output.amount.len() == 1
                    && utxo.output.amount[0].unit() == "lovelace"
                    && utxo.output.amount[0].quantity_i128() >= 5_000_000
            })
            .collect();

        collateral_candidates.sort_by(|a, b| {
            let a_quantity = a.output.amount[0].quantity_i128();
            let b_quantity = b.output.amount[0].quantity_i128();
            a_quantity.cmp(&b_quantity)
        });

        if let Some(smallest_utxo) = collateral_candidates.first() {
            Ok(vec![smallest_utxo.clone()])
        } else {
            Ok(vec![])
        }
    }
}
