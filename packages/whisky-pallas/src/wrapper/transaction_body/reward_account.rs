use std::str::FromStr;

use pallas::{
    codec::utils::Bytes,
    ledger::primitives::{conway::RewardAccount as PallasRewardAccount, Fragment},
};
use whisky_common::WError;

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
