use std::str::FromStr;

use pallas::{
    codec::utils::Bytes,
    ledger::primitives::{
        conway::RewardAccount as PallasRewardAccount, Fragment,
        StakeCredential as PallasStakeCredential,
    },
};
use pallas_crypto::hash::Hash;
use whisky_common::WError;

use crate::wrapper::transaction_body::StakeCredential;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct RewardAccount {
    pub inner: PallasRewardAccount,
}

impl RewardAccount {
    pub fn new(reward_account: String) -> Result<Self, WError> {
        let inner = Bytes::from_str(&reward_account)
            .map_err(|e| WError::new("RewardAccount - Invalid reward account", &e.to_string()))?;
        Ok(Self { inner })
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Self, WError> {
        let (_hrp, data) = bech32::decode(bech32_str)
            .map_err(|e| WError::new("Bech32 decode error", &format!("{}", e)))?;

        Ok(Self {
            inner: Bytes::from(data),
        })
    }

    pub fn to_stake_cred(&self) -> Result<StakeCredential, WError> {
        let bytes = self.inner.to_vec();
        let header_byte = bytes.first().ok_or_else(|| {
            WError::new("StakeCredential - Bech32 decode error", "Empty data part")
        })?;

        // Check the header byte starts with 111
        if header_byte >> 5 != 0b111 {
            return Err(WError::new(
                "StakeCredential - Bech32 decode error",
                "Invalid StakeCredential header byte",
            ));
        } else {
            // If the 3rd bit is 0, it's a key hash; if it's 1, it's a script hash
            let is_script_hash = (header_byte >> 4) & 0b1 == 1;
            if is_script_hash {
                Ok(StakeCredential {
                    inner: PallasStakeCredential::ScriptHash(Hash::from(&bytes[1..])),
                })
            } else {
                Ok(StakeCredential {
                    inner: PallasStakeCredential::AddrKeyhash(Hash::from(&bytes[1..])),
                })
            }
        }
    }

    pub fn encode(&self) -> Result<String, WError> {
        let encoded_fragment = self
            .inner
            .encode_fragment()
            .map_err(|e| WError::new("RewardAccount - Fragment encode error", &e.to_string()))?;
        Ok(hex::encode(encoded_fragment))
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasRewardAccount::decode_fragment(&bytes)
            .map_err(|e| WError::new("RewardAccount - Fragment decode error", &e.to_string()))?;
        Ok(Self { inner })
    }
}
