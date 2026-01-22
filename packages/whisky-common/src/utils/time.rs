use crate::Network;
use serde::{Deserialize, Serialize};

/// Slot configuration for a Cardano network.
/// Contains the parameters needed to convert between slots and POSIX time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotConfig {
    /// The POSIX time (in milliseconds) at the zero slot
    pub zero_time: u64,
    /// The slot number at zero time
    pub zero_slot: u64,
    /// The length of each slot in milliseconds
    pub slot_length: u64,
    /// The starting epoch number
    pub start_epoch: u64,
    /// The number of slots in each epoch
    pub epoch_length: u64,
}

impl SlotConfig {
    /// Create a new SlotConfig with custom parameters
    pub fn new(
        zero_time: u64,
        zero_slot: u64,
        slot_length: u64,
        start_epoch: u64,
        epoch_length: u64,
    ) -> Self {
        Self {
            zero_time,
            zero_slot,
            slot_length,
            start_epoch,
            epoch_length,
        }
    }

    /// Get the slot configuration for Cardano mainnet (starting at Shelley era)
    pub fn mainnet() -> Self {
        Self {
            zero_time: 1596059091000,
            zero_slot: 4492800,
            slot_length: 1000,
            start_epoch: 208,
            epoch_length: 432000,
        }
    }

    /// Get the slot configuration for Cardano preview testnet
    pub fn preview() -> Self {
        Self {
            zero_time: 1666656000000,
            zero_slot: 0,
            slot_length: 1000,
            start_epoch: 0,
            epoch_length: 86400,
        }
    }

    /// Get the slot configuration for Cardano preprod testnet
    pub fn preprod() -> Self {
        Self {
            zero_time: 1654041600000 + 1728000000,
            zero_slot: 86400,
            slot_length: 1000,
            start_epoch: 4,
            epoch_length: 432000,
        }
    }
}

impl Default for SlotConfig {
    fn default() -> Self {
        Self::mainnet()
    }
}

/// Get the slot configuration for a specific network.
///
/// # Arguments
/// * `network` - The Cardano network
///
/// # Returns
/// The slot configuration for the network, or None for Custom networks
pub fn get_slot_config(network: &Network) -> Option<SlotConfig> {
    match network {
        Network::Mainnet => Some(SlotConfig::mainnet()),
        Network::Preview => Some(SlotConfig::preview()),
        Network::Preprod => Some(SlotConfig::preprod()),
        Network::Custom(_) => None,
    }
}

/// Convert a slot number to the beginning POSIX time (in milliseconds).
///
/// # Arguments
/// * `slot` - The slot number
/// * `slot_config` - The slot configuration for the network
///
/// # Returns
/// The POSIX time in milliseconds at the beginning of the slot
pub fn slot_to_begin_unix_time(slot: u64, slot_config: &SlotConfig) -> u64 {
    let ms_after_begin = (slot - slot_config.zero_slot) * slot_config.slot_length;
    slot_config.zero_time + ms_after_begin
}

/// Convert a POSIX time to the enclosing slot number.
///
/// # Arguments
/// * `unix_time` - The POSIX time in milliseconds
/// * `slot_config` - The slot configuration for the network
///
/// # Returns
/// The slot number that contains the given time
pub fn unix_time_to_enclosing_slot(unix_time: u64, slot_config: &SlotConfig) -> u64 {
    let time_passed = unix_time - slot_config.zero_time;
    let slots_passed = time_passed / slot_config.slot_length;
    slots_passed + slot_config.zero_slot
}

/// Resolve the slot number for a network at a given time.
///
/// # Arguments
/// * `network` - The Cardano network
/// * `milliseconds` - Optional POSIX time in milliseconds (defaults to current time)
///
/// # Returns
/// The slot number as a string, or None if the network is Custom
pub fn resolve_slot_no(network: &Network, milliseconds: Option<u64>) -> Option<String> {
    let slot_config = get_slot_config(network)?;
    let time = milliseconds.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    });
    Some(unix_time_to_enclosing_slot(time, &slot_config).to_string())
}

/// Resolve the epoch number for a network at a given time.
///
/// # Arguments
/// * `network` - The Cardano network
/// * `milliseconds` - Optional POSIX time in milliseconds (defaults to current time)
///
/// # Returns
/// The epoch number, or None if the network is Custom
pub fn resolve_epoch_no(network: &Network, milliseconds: Option<u64>) -> Option<u64> {
    let config = get_slot_config(network)?;
    let time = milliseconds.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    });

    let epoch = (time - config.zero_time) / 1000 / config.epoch_length + config.start_epoch;
    Some(epoch)
}
