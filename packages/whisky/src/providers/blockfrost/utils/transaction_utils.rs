use whisky_common::{TransactionInfo, UTxO};

use crate::providers::blockfrost::models::{
    BlockfrostTxInfo, BlockfrostTxUtxoOutputs, BlockfrostUtxo,
};

pub fn blockfrost_txinfo_to_txinfo(
    blockfrost_tx_info: BlockfrostTxInfo,
    inputs: Vec<UTxO>,
    outputs: Vec<UTxO>,
) -> TransactionInfo {
    TransactionInfo {
        index: blockfrost_tx_info.index as u32,
        block: blockfrost_tx_info.block,
        hash: blockfrost_tx_info.hash,
        slot: blockfrost_tx_info.slot.to_string(),
        fees: blockfrost_tx_info.fees,
        size: blockfrost_tx_info.size as u32,
        deposit: blockfrost_tx_info.deposit,
        invalid_before: blockfrost_tx_info.invalid_before.unwrap_or("".to_string()),
        invalid_after: blockfrost_tx_info
            .invalid_hereafter
            .unwrap_or("".to_string()),
        inputs,
        outputs,
        block_height: Some(blockfrost_tx_info.block_height as u32),
        block_time: Some(blockfrost_tx_info.block_time as u64),
    }
}

pub fn blockfrost_tx_output_utxo_to_blockfrost_utxo(
    utxo: &BlockfrostTxUtxoOutputs,
    tx_hash: &str,
) -> BlockfrostUtxo {
    BlockfrostUtxo {
        address: utxo.address.clone(),
        tx_hash: tx_hash.to_string(),
        tx_index: utxo.output_index,
        output_index: utxo.output_index,
        amount: utxo.amount.clone(),
        block: "".to_string(),
        data_hash: utxo.data_hash.clone(),
        inline_datum: utxo.inline_datum.clone(),
        reference_script_hash: utxo.reference_script_hash.clone(),
    }
}
