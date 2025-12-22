use cardano_serialization_lib as csl;
use pallas_codec::minicbor::data::Tag;
use pallas_codec::minicbor::Encoder;
use whisky_common::WError;

use super::primitives::proto_to_bignum;
use crate::tx_prototype::types::*;

/// Convert NativeScriptPrototype to CSL NativeScript
pub fn proto_to_native_script(script: &NativeScriptPrototype) -> Result<csl::NativeScript, WError> {
    match script {
        NativeScriptPrototype::ScriptPubkey { value: pubkey } => {
            let keyhash = csl::Ed25519KeyHash::from_hex(&pubkey.addr_keyhash).map_err(
                WError::from_err("proto_to_native_script - invalid addr_keyhash"),
            )?;
            Ok(csl::NativeScript::new_script_pubkey(
                &csl::ScriptPubkey::new(&keyhash),
            ))
        }
        NativeScriptPrototype::ScriptAll { value: all } => {
            let mut scripts = csl::NativeScripts::new();
            for s in &all.native_scripts {
                scripts.add(&proto_to_native_script(s)?);
            }
            Ok(csl::NativeScript::new_script_all(&csl::ScriptAll::new(
                &scripts,
            )))
        }
        NativeScriptPrototype::ScriptAny { value: any } => {
            let mut scripts = csl::NativeScripts::new();
            for s in &any.native_scripts {
                scripts.add(&proto_to_native_script(s)?);
            }
            Ok(csl::NativeScript::new_script_any(&csl::ScriptAny::new(
                &scripts,
            )))
        }
        NativeScriptPrototype::ScriptNOfK { value: nofk } => {
            let mut scripts = csl::NativeScripts::new();
            for s in &nofk.native_scripts {
                scripts.add(&proto_to_native_script(s)?);
            }
            Ok(csl::NativeScript::new_script_n_of_k(&csl::ScriptNOfK::new(
                nofk.n, &scripts,
            )))
        }
        NativeScriptPrototype::TimelockStart { value: start } => {
            let slot = proto_to_bignum(&start.slot)?;
            Ok(csl::NativeScript::new_timelock_start(
                &csl::TimelockStart::new_timelockstart(&slot),
            ))
        }
        NativeScriptPrototype::TimelockExpiry { value: expiry } => {
            let slot = proto_to_bignum(&expiry.slot)?;
            Ok(csl::NativeScript::new_timelock_expiry(
                &csl::TimelockExpiry::new_timelockexpiry(&slot),
            ))
        }
    }
}

/// Convert ScriptRefPrototype (hex string) to CSL ScriptRef
/// Handles both regular format and "82" prefixed format that needs CBOR tag 24 wrapping
pub fn proto_to_script_ref(script_ref: &ScriptRefPrototype) -> Result<csl::ScriptRef, WError> {
    if script_ref.starts_with("82") {
        // Special handling for "82" prefixed scripts - wrap with CBOR tag 24
        let bytes = hex::decode(script_ref).map_err(WError::from_err(
            "proto_to_script_ref - failed to decode hex",
        ))?;
        let mut encoder = Encoder::new(Vec::new());
        encoder
            .tag(Tag::new(24))
            .map_err(|_| WError::new("proto_to_script_ref", "failed to write tag"))?;
        encoder.bytes(&bytes).map_err(|e| {
            WError::new(
                "proto_to_script_ref",
                &format!("failed to encode bytes: {:?}", e),
            )
        })?;
        let write_buffer = encoder.writer().clone();
        csl::ScriptRef::from_bytes(write_buffer).map_err(WError::from_err(
            "proto_to_script_ref - failed to parse wrapped script ref",
        ))
    } else {
        csl::ScriptRef::from_hex(script_ref).map_err(WError::from_err(
            "proto_to_script_ref - invalid script ref hex",
        ))
    }
}
