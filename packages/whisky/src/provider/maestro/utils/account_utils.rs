use sidan_csl_rs::model::AccountInfo;

use crate::provider::maestro::models::account::AccountInformation;

pub fn account_information_to_account_info(account_information: AccountInformation) -> AccountInfo {
    AccountInfo {
        active: account_information.registered,
        pool_id: account_information
            .delegated_pool
            .unwrap_or(("").to_string()),
        balance: account_information.total_balance.to_string(),
        rewards: account_information.rewards_available.to_string(),
        withdrawals: account_information.total_withdrawn.to_string(),
    }
}
