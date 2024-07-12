use cardano_serialization_lib as csl;
use csl::JsError;
use model::{Anchor, DRep};

use crate::*;

pub fn to_csl_drep(drep: &DRep) -> Result<csl::DRep, JsError> {
    match drep {
        DRep::KeyHash(key_hash) => Ok(csl::DRep::new_key_hash(&csl::Ed25519KeyHash::from_hex(
            key_hash,
        )?)),
        DRep::ScriptHash(script_hash) => Ok(csl::DRep::new_script_hash(
            &csl::ScriptHash::from_hex(script_hash)?,
        )),
        DRep::AlwaysAbstain => Ok(csl::DRep::new_always_abstain()),
        DRep::AlwaysNoConfidence => Ok(csl::DRep::new_always_no_confidence()),
    }
}

pub fn to_csl_anchor(anchor: &Anchor) -> Result<csl::Anchor, JsError> {
    Ok(csl::Anchor::new(
        &csl::URL::new(anchor.anchor_url.clone())?,
        &csl::AnchorDataHash::from_hex(&anchor.anchor_data_hash)?,
    ))
}
