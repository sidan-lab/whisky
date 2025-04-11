use whisky_csl::csl::{Address, BaseAddress, JsError, RewardAddress};

pub fn resolve_reward_address(bech32: &str) -> Result<String, JsError> {
    let address = Address::from_bech32(bech32)?;

    if let Some(base_address) = BaseAddress::from_address(&address) {
        let stake_credential = BaseAddress::stake_cred(&base_address);

        let reward_address = RewardAddress::new(address.network_id()?, &stake_credential)
            .to_address()
            .to_bech32(None);
        Ok(reward_address?)
    } else {
        Err(JsError::from_str(
            "An error occurred during resolveRewardAddress",
        ))
    }
}
