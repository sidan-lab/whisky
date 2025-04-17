use super::derivation_indices::DerivationIndices;

pub struct RootKeyWallet {
    pub root_key: String,
    pub derivation_indices: DerivationIndices,
}

impl RootKeyWallet {
    pub fn payment_account(&mut self, account_index: u32, key_index: u32) -> &mut Self {
        self.derivation_indices = DerivationIndices::payment(account_index, key_index);
        self
    }

    pub fn stake_account(&mut self, account_index: u32, key_index: u32) -> &mut Self {
        self.derivation_indices = DerivationIndices::stake(account_index, key_index);
        self
    }

    pub fn drep_account(&mut self, account_index: u32, key_index: u32) -> &mut Self {
        self.derivation_indices = DerivationIndices::drep(account_index, key_index);
        self
    }
}
