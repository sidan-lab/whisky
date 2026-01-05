use pallas::{
    codec::utils::PositiveCoin,
    ledger::primitives::{conway::Value as PallasValue, Fragment},
};

use crate::wrapper::transaction_body::MultiassetPositiveCoin;
use whisky_common::WError;

#[derive(Debug, PartialEq, Clone)]
pub struct Value {
    pub inner: PallasValue,
}

impl Value {
    pub fn new(coin: u64, multiasset: Option<MultiassetPositiveCoin>) -> Self {
        match multiasset {
            Some(ma) => Self {
                inner: PallasValue::Multiasset(coin, ma.inner),
            },
            None => Self {
                inner: PallasValue::Coin(coin),
            },
        }
    }

    pub fn add(&self, other: &Value) -> Result<Value, WError> {
        match (&self.inner, &other.inner) {
            (PallasValue::Coin(a), PallasValue::Coin(b)) => Ok(Value {
                inner: PallasValue::Coin(a + b),
            }),
            (PallasValue::Coin(a), PallasValue::Multiasset(b, b_ma)) => Ok(Value {
                inner: PallasValue::Multiasset(a + b, b_ma.clone()),
            }),
            (PallasValue::Multiasset(a, a_ma), PallasValue::Coin(b)) => Ok(Value {
                inner: PallasValue::Multiasset(a + b, a_ma.clone()),
            }),
            (PallasValue::Multiasset(a, a_ma), PallasValue::Multiasset(b, b_ma)) => {
                let mut combined_ma = a_ma.clone();
                for (policy_id, assets) in b_ma.iter() {
                    let entry = combined_ma.entry(*policy_id).or_default();
                    for (asset_name, amount) in assets.iter() {
                        let asset_entry = entry.entry(asset_name.clone());
                        match asset_entry {
                            std::collections::btree_map::Entry::Vacant(_vacant_entry) => {
                                entry.insert(asset_name.clone(), *amount);
                            }
                            std::collections::btree_map::Entry::Occupied(occupied_entry) => {
                                let new_amount =
                                    u64::from(*occupied_entry.get()) + u64::from(amount);
                                *occupied_entry.into_mut() = PositiveCoin::try_from(new_amount)
                                    .map_err(|_| {
                                        WError::new(
                                        "Value - Add:",
                                        "Failed to create PositiveCoin from added asset amounts",
                                    )
                                    })?;
                            }
                        }
                    }
                }
                Ok(Value {
                    inner: PallasValue::Multiasset(a + b, combined_ma),
                })
            }
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, WError> {
        match (&self.inner, &other.inner) {
            (PallasValue::Coin(a), PallasValue::Coin(b)) => Ok(Value {
                inner: PallasValue::Coin(a - b),
            }),
            (PallasValue::Coin(a), PallasValue::Multiasset(b, b_ma)) => Ok(Value {
                inner: PallasValue::Multiasset(a - b, b_ma.clone()),
            }),
            (PallasValue::Multiasset(a, a_ma), PallasValue::Coin(b)) => Ok(Value {
                inner: PallasValue::Multiasset(a - b, a_ma.clone()),
            }),
            (PallasValue::Multiasset(a, a_ma), PallasValue::Multiasset(b, b_ma)) => {
                let mut combined_ma = a_ma.clone();
                for (policy_id, assets) in b_ma.iter() {
                    if let Some(entry) = combined_ma.get_mut(policy_id) {
                        for (asset_name, amount) in assets.iter() {
                            if let Some(asset_entry) = entry.get_mut(asset_name) {
                                let new_amount = u64::from(*asset_entry) - u64::from(amount);
                                if new_amount == 0 {
                                    entry.remove(asset_name);
                                } else {
                                    *asset_entry = PositiveCoin::try_from(new_amount).map_err(|_| {
                                    WError::new(
                                        "Value - Sub:",
                                        "Failed to create PositiveCoin from subtracted asset amounts",
                                    )
                                })?;
                                }
                            }
                        }
                    }
                }
                for (policy_id, _assets) in b_ma.iter() {
                    if let Some(entry) = combined_ma.get_mut(policy_id) {
                        if entry.is_empty() {
                            combined_ma.remove(policy_id);
                        }
                    }
                }
                Ok(Value {
                    inner: PallasValue::Multiasset(a - b, combined_ma),
                })
            }
        }
    }

    pub fn encode(&self) -> Result<String, WError> {
        let bytes = self.inner.encode_fragment().map_err(|e| {
            WError::new(
                "Value - Encode:",
                &format!("Fragment encoding failed: {}", e),
            )
        })?;
        Ok(hex::encode(bytes))
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasValue::decode_fragment(bytes).map_err(|e| {
            WError::new("Value - Decode:", &format!("Fragment decode error: {}", e))
        })?;
        Ok(Self { inner })
    }
}
