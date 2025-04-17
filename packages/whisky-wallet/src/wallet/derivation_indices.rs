use crate::wallet_constants::HARDENED_KEY_START;

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

impl DerivationIndices {
    pub fn payment(account_index: u32, key_index: u32) -> Self {
        DerivationIndices(vec![
            HARDENED_KEY_START + 1852,          // purpose
            HARDENED_KEY_START + 1815,          // coin type
            HARDENED_KEY_START + account_index, // account
            0,                                  // payment
            key_index,                          // key index
        ])
    }

    pub fn stake(account_index: u32, key_index: u32) -> Self {
        DerivationIndices(vec![
            HARDENED_KEY_START + 1852,          // purpose
            HARDENED_KEY_START + 1815,          // coin type
            HARDENED_KEY_START + account_index, // account
            2,                                  // stake
            key_index,                          // key index
        ])
    }

    pub fn drep(account_index: u32, key_index: u32) -> Self {
        DerivationIndices(vec![
            HARDENED_KEY_START + 1852,          // purpose
            HARDENED_KEY_START + 1815,          // coin type
            HARDENED_KEY_START + account_index, // account
            3,                                  // stake
            key_index,                          // key index
        ])
    }
}
