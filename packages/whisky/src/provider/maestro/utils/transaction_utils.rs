use super::address_utils::maestro_utxo_to_utxo;
use maestro_rust_sdk::models::transactions::TransactionDetail;
use sidan_csl_rs::model::TransactionInfo;

pub fn transaction_detail_to_info(transaction_detail: TransactionDetail) -> TransactionInfo {
    TransactionInfo {
        index: transaction_detail.block_tx_index as u32,
        block: transaction_detail.block_hash,
        hash: transaction_detail.tx_hash,
        slot: transaction_detail.block_absolute_slot.to_string(),
        fees: transaction_detail.fee.to_string(),
        size: transaction_detail.size as u32,
        deposit: transaction_detail.deposit.to_string(),
        invalid_before: transaction_detail.invalid_before.to_string(),
        invalid_after: transaction_detail.invalid_hereafter.to_string(),
        inputs: transaction_detail
            .inputs
            .iter()
            .map(|utxo| maestro_utxo_to_utxo(utxo.clone()))
            .collect(),
        outputs: transaction_detail
            .outputs
            .iter()
            .map(|utxo| maestro_utxo_to_utxo(utxo.clone()))
            .collect(),
        block_height: Some(transaction_detail.block_height as u32),
        block_time: Some(transaction_detail.block_timestamp as u64),
    }
}
