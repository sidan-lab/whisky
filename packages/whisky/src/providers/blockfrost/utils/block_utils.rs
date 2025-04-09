use maestro_rust_sdk::client::block_info::BlockInfoData;
use whisky_common::models::BlockInfo;

pub fn block_info_data_to_block_info(block_info_data: BlockInfoData) -> BlockInfo {
    BlockInfo {
        time: block_info_data.timestamp.parse::<u64>().unwrap_or(0),
        hash: block_info_data.hash,
        slot: block_info_data.absolute_slot.to_string(),
        epoch: block_info_data.epoch as u32,
        epoch_slot: block_info_data.epoch_slot.to_string(),
        slot_leader: block_info_data.block_producer,
        size: block_info_data.size as usize,
        tx_count: block_info_data.tx_hashes.len(),
        output: block_info_data.total_output_lovelace,
        fees: block_info_data.total_fees.to_string(),
        previous_block: block_info_data.previous_block,
        next_block: String::new(), // You'll need to determine the next block somehow
        confirmations: block_info_data.confirmations as usize,
        operational_certificate: block_info_data.operational_certificate.kes_signature,
        vrf_key: block_info_data.vrf_key,
    }
}
