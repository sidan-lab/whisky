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

    pub fn from_str(derivation_path_str: &str) -> Self {
        let derivation_path_vec: Vec<&str> = derivation_path_str.split('/').collect();
        let derivation_path_vec_u32: Vec<u32> = derivation_path_vec
            .iter()
            .skip(1)
            .filter_map(|&s| {
                if s.ends_with("'") {
                    // Remove the last character (')
                    let path_str = s.strip_suffix("'").unwrap();
                    // Parse the string to u32 and add 0x80000000 for hardening
                    Some(path_str.parse::<u32>().unwrap() + 0x80000000)
                } else {
                    s.parse::<u32>().ok()
                }
            })
            .collect();
        DerivationIndices(derivation_path_vec_u32)
    }
}
