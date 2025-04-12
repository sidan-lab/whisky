use crate::providers::maestro::models::transaction::TransactionDetail;

use whisky_common::{models::TransactionInfo, WError};

use super::utxo_utils::to_utxo;

pub fn transaction_detail_to_info(
    transaction_detail: TransactionDetail,
) -> Result<TransactionInfo, WError> {
    let tx_info = TransactionInfo {
        index: transaction_detail.block_tx_index as u32,
        block: transaction_detail.block_hash,
        hash: transaction_detail.tx_hash,
        slot: transaction_detail.block_absolute_slot.to_string(),
        fees: transaction_detail.fee.to_string(),
        size: transaction_detail.size as u32,
        deposit: transaction_detail.deposit.to_string(),
        invalid_before: match transaction_detail.invalid_before {
            Some(i) => i.to_string(),
            None => "".to_string(),
        },
        invalid_after: transaction_detail
            .invalid_hereafter
            .unwrap_or(0)
            .to_string(),
        inputs: transaction_detail
            .inputs
            .iter()
            .map(to_utxo)
            .collect::<Result<Vec<_>, _>>()?,
        outputs: transaction_detail
            .outputs
            .iter()
            .map(to_utxo)
            .collect::<Result<Vec<_>, _>>()?,
        block_height: Some(transaction_detail.block_height as u32),
        block_time: Some(transaction_detail.block_timestamp as u64),
    };
    Ok(tx_info)
}
