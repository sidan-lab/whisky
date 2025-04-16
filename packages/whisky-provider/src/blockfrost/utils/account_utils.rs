use whisky_common::models::AccountInfo;

use crate::blockfrost::models::account::BlockfrostAccountInfo;

pub fn blockfrost_account_info_to_account_info(
    blockfrost_account_info: BlockfrostAccountInfo,
) -> AccountInfo {
    AccountInfo {
        active: blockfrost_account_info.active,
        pool_id: blockfrost_account_info.pool_id.unwrap_or(("").to_string()),
        balance: blockfrost_account_info.controlled_amount,
        rewards: blockfrost_account_info.withdrawable_amount,
        withdrawals: blockfrost_account_info.withdrawals_sum,
    }
}
