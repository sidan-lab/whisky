use whisky_common::{models::BlockInfo, WError};

use crate::providers::blockfrost::models::BlockContent;

pub fn block_content_to_block_info(block_content: BlockContent) -> Result<BlockInfo, WError> {
    let block_info = BlockInfo {
        time: block_content.time as u64,
        hash: block_content.hash,
        slot: match block_content.slot {
            Some(s) => s.to_string(),
            None => "".to_string(),
        },
        epoch: block_content
            .epoch
            .ok_or_else(WError::from_opt("block_content_to_block_info", "epoch"))?
            as u32,
        epoch_slot: match block_content.epoch_slot {
            Some(s) => s.to_string(),
            None => "".to_string(),
        },
        slot_leader: block_content.slot_leader,
        size: block_content.size as usize,
        tx_count: block_content.tx_count as usize,
        output: block_content.output.unwrap_or("".to_string()),
        fees: block_content.fees.unwrap_or("".to_string()),
        previous_block: block_content.previous_block.unwrap_or("".to_string()),
        next_block: block_content.next_block.unwrap_or("".to_string()),
        confirmations: block_content.confirmations as usize,
        operational_certificate: block_content.op_cert.unwrap_or("".to_string()),
        vrf_key: block_content.block_vrf.unwrap_or("".to_string()),
    };
    Ok(block_info)
}
