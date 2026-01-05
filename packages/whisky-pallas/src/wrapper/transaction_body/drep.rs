use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::conway::DRep as PallasDRep;
use pallas::ledger::primitives::Fragment;
use std::str::FromStr;
use whisky_common::WError;

pub enum DRepKind {
    Key { addr_key_hash: String },
    Script { script_hash: String },
    Abstain,
    NoConfidence,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DRep {
    pub inner: PallasDRep,
}

impl DRep {
    pub fn new(drep: DRepKind) -> Result<Self, WError> {
        let pallas_drep = match drep {
            DRepKind::Key { addr_key_hash } => {
                let key_hash = Hash::<28>::from_str(&addr_key_hash)
                    .map_err(|e| WError::new("DRep - Invalid key hash length", &e.to_string()))?;
                PallasDRep::Key(key_hash)
            }

            DRepKind::Script { script_hash } => {
                let script_hash = Hash::<28>::from_str(&script_hash).map_err(|e| {
                    WError::new("DRep - Invalid script hash length", &e.to_string())
                })?;
                PallasDRep::Script(script_hash)
            }

            DRepKind::Abstain => PallasDRep::Abstain,
            DRepKind::NoConfidence => PallasDRep::NoConfidence,
        };

        Ok(Self { inner: pallas_drep })
    }

    pub fn from_bech32(bech32_str: &str) -> Result<Self, WError> {
        let (hrp, data) = bech32::decode(bech32_str)
            .map_err(|e| WError::new("DRep - Bech32 decode error", &e.to_string()))?;
        // If data length is 28, it's in CIP-105 format
        if data.len() == 28 {
            return match hrp.as_str() {
                "drep_vkh" => Ok(Self {
                    inner: PallasDRep::Key(Hash::<28>::from(&data[..])),
                }),
                "drep" => Ok(Self {
                    inner: PallasDRep::Key(Hash::<28>::from(&data[..])),
                }),
                "drep_script" => Ok(Self {
                    inner: PallasDRep::Script(Hash::<28>::from(&data[..])),
                }),
                _ => Err(WError::new("DRep - Bech32 decode error", "Invalid HRP")),
            };
        } else {
            // Otherwise follow CIP-129
            let header_byte = data
                .first()
                .ok_or_else(|| WError::new("DRep - Bech32 decode error", "Empty data part"))?;

            // Check the header byte starts with 0010
            if header_byte >> 4 != 0b0010 {
                return Err(WError::new(
                    "DRep - Bech32 decode error",
                    "Invalid DRep header byte",
                ));
            } else {
                // If the final 4 bits are 0010, it's a key hash; if it's 0011, it's a script hash
                let is_script_hash = (header_byte & 0b0000_1111) == 0b0011;
                if is_script_hash {
                    let script_hash = Hash::<28>::from(&data[1..]);
                    Ok(Self {
                        inner: PallasDRep::Script(script_hash),
                    })
                } else {
                    let key_hash = Hash::<28>::from(&data[1..]);
                    Ok(Self {
                        inner: PallasDRep::Key(key_hash),
                    })
                }
            }
        }
    }

    pub fn encode(&self) -> Result<String, WError> {
        let encoded_fragment = self
            .inner
            .encode_fragment()
            .map_err(|e| WError::new("DRep - Fragment encode error", &e.to_string()))?;
        Ok(hex::encode(encoded_fragment))
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasDRep::decode_fragment(&bytes)
            .map_err(|e| WError::new("DRep - Fragment decode error", &e.to_string()))?;
        Ok(Self { inner })
    }
}
