use bip39::{Language, Mnemonic};
use cardano_serialization_lib::FixedTransaction;
use cardano_serialization_lib::{Bip32PrivateKey, JsError, PrivateKey, PublicKey};

pub struct Bip32KeyGenerator {
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl Bip32KeyGenerator {
    pub fn new(mnemonic_phrase: &str) -> Self {
        let mnemonic = Mnemonic::from_phrase(mnemonic_phrase, Language::English)
            .expect("Invalid mnemonic phrase");
        let entropy = mnemonic.entropy();
        let root_key = Bip32PrivateKey::from_bip39_entropy(entropy, &[]);

        let hardened_key_start = 2147483648;
        let account_key = root_key
            .derive(hardened_key_start + 1852)
            .derive(hardened_key_start + 1815)
            .derive(hardened_key_start);

        let private_key = account_key.derive(0).derive(0).to_raw_key();
        let public_key = private_key.to_public();
        Bip32KeyGenerator {
            private_key,
            public_key,
        }
    }

    pub fn sign_transaction(&self, tx_hex: &str) -> Result<String, JsError> {
        let mut tx = FixedTransaction::from_hex(tx_hex).expect("Invalid transaction bytes");
        tx.sign_and_add_vkey_signature(&self.private_key)?;
        Ok(tx.to_hex())
    }

    pub fn get_public_key(&self) -> String {
        self.public_key.clone().to_hex()
    }
}

#[test]
fn test_sign_tx() {
    let mnemonic_phrase = "summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer";
    let wallet = Bip32KeyGenerator::new(mnemonic_phrase);
    let tx_hex = "84a4008182582004509185eb98edd8e2420c1ceea914d6a7a3142041039b2f12b4d4f03162d56f04018282581d605867c3b8e27840f556ac268b781578b14c5661fc63ee720dbeab663f1a000f42408258390004845038ee499ee8bc0afe56f688f27b2dd76f230d3698a9afcc1b66e0464447c1f51adaefe1ebfb0dd485a349a70479ced1d198cbdf7fe71a15d35396021a0002917d075820bdaa99eb158414dea0a91d6c727e2268574b23efe6e08ab3b841abe8059a030ca0f5d90103a0";
    let signed_tx = wallet.sign_transaction(tx_hex).unwrap();

    assert_eq!(
        signed_tx,
        "84a4008182582004509185eb98edd8e2420c1ceea914d6a7a3142041039b2f12b4d4f03162d56f04018282581d605867c3b8e27840f556ac268b781578b14c5661fc63ee720dbeab663f1a000f42408258390004845038ee499ee8bc0afe56f688f27b2dd76f230d3698a9afcc1b66e0464447c1f51adaefe1ebfb0dd485a349a70479ced1d198cbdf7fe71a15d35396021a0002917d075820bdaa99eb158414dea0a91d6c727e2268574b23efe6e08ab3b841abe8059a030ca1008182582089f4b576f05f5aad99bce0bdd51afe48529772f7561bb2ac9d84a4afbda1ecd658404cd1466fcc4579fa9c89656dbbd25ca659cccf2d2783417ef13a1b060bf836fbe8383c10e25c6fa323c1c81a0799e87e6cf3eaa25990113b27953a9836635a01f5d90103a0"
    );
}
