use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::conway::DRep as PallasDRep;
use std::str::FromStr;

pub enum DRepKind {
    Key { addr_key_hash: String },
    Script { script_hash: String },
    Abstain,
    NoConfidence,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DRep {
    inner: PallasDRep,
}

impl DRep {
    pub fn new(drep: DRepKind) -> Result<Self, String> {
        let pallas_drep = match drep {
            DRepKind::Key { addr_key_hash } => {
                let key_hash = Hash::<28>::from_str(&addr_key_hash)
                    .map_err(|e| format!("Invalid key hash length: {}", e))?;
                PallasDRep::Key(key_hash)
            }

            DRepKind::Script { script_hash } => {
                let script_hash = Hash::<28>::from_str(&script_hash)
                    .map_err(|e| format!("Invalid script hash length: {}", e))?;
                PallasDRep::Script(script_hash)
            }

            DRepKind::Abstain => PallasDRep::Abstain,
            DRepKind::NoConfidence => PallasDRep::NoConfidence,
        };

        Ok(Self { inner: pallas_drep })
    }
}
