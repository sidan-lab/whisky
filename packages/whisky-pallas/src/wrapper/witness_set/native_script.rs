use std::str::FromStr;

use pallas::{
    crypto::hash::Hash,
    ledger::primitives::{conway::NativeScript as PallasNativeScript, Fragment},
};
use whisky_common::WError;

pub enum NativeScriptKind {
    ScriptPubkey(String),
    ScriptAll(Vec<NativeScriptKind>),
    ScriptAny(Vec<NativeScriptKind>),
    ScriptNOfK(u32, Vec<NativeScriptKind>),
    InvalidBefore(u64),
    InvalidHereafter(u64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NativeScript {
    pub inner: PallasNativeScript,
}

impl NativeScript {
    pub fn new(native_script: NativeScriptKind) -> Result<Self, WError> {
        let inner = Self::convert_native_script_kind_to_pallas(native_script)?;
        Ok(NativeScript { inner })
    }

    pub fn new_from_hex(hex_str: &str) -> Result<Self, WError> {
        let bytes = hex::decode(hex_str).map_err(|e| {
            WError::new(
                "WhiskyPallas - Creating native script from hex:",
                &format!("Hex decode error: {}", e.to_string()),
            )
        })?;
        let inner = PallasNativeScript::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Creating native script from hex:",
                &format!("Fragment decode error: {}", e.to_string()),
            )
        })?;
        Ok(NativeScript { inner })
    }

    fn convert_native_script_kind_to_pallas(
        kind: NativeScriptKind,
    ) -> Result<PallasNativeScript, WError> {
        match kind {
            NativeScriptKind::ScriptPubkey(key_hash) => {
                let bytes = Hash::<28>::from_str(&key_hash).map_err(|e| {
                    WError::new(
                        "WhiskyPallas - Converting native script:",
                        &format!("Invalid key hash: {}", e),
                    )
                })?;
                Ok(PallasNativeScript::ScriptPubkey(bytes))
            }
            NativeScriptKind::ScriptAll(scripts) => {
                let converted: Result<Vec<_>, WError> = scripts
                    .into_iter()
                    .map(Self::convert_native_script_kind_to_pallas)
                    .collect();
                Ok(PallasNativeScript::ScriptAll(converted?))
            }
            NativeScriptKind::ScriptAny(scripts) => {
                let converted: Result<Vec<_>, WError> = scripts
                    .into_iter()
                    .map(Self::convert_native_script_kind_to_pallas)
                    .collect();
                Ok(PallasNativeScript::ScriptAny(converted?))
            }
            NativeScriptKind::ScriptNOfK(n, scripts) => {
                let converted: Result<Vec<_>, WError> = scripts
                    .into_iter()
                    .map(Self::convert_native_script_kind_to_pallas)
                    .collect();
                Ok(PallasNativeScript::ScriptNOfK(n, converted?))
            }
            NativeScriptKind::InvalidBefore(slot) => Ok(PallasNativeScript::InvalidBefore(slot)),
            NativeScriptKind::InvalidHereafter(slot) => {
                Ok(PallasNativeScript::InvalidHereafter(slot))
            }
        }
    }

    pub fn encode(&self) -> Result<String, WError> {
        self.inner
            .encode_fragment()
            .map(|bytes| hex::encode(bytes))
            .map_err(|_| {
                WError::new(
                    "WhiskyPallas - Encoding native script:",
                    "Failed to encode fragment",
                )
            })
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasNativeScript::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Decoding native script:",
                &format!("Fragment decode error: {}", e.to_string()),
            )
        })?;
        Ok(Self { inner })
    }
}
