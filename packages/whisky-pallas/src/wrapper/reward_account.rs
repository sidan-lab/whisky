use std::str::FromStr;

use pallas::{
    codec::utils::Bytes,
    ledger::primitives::{conway::RewardAccount as PallasRewardAccount, Fragment},
};

pub struct RewardAccount {
    inner: PallasRewardAccount,
}

impl RewardAccount {
    pub fn new(reward_account: String) -> Result<Self, String> {
        let inner = Bytes::from_str(&reward_account)
            .map_err(|e| format!("Invalid reward account: {}", e.to_string()))?;
        Ok(Self { inner })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at RewardAccount"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasRewardAccount::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
