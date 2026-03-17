use whisky_common::{LanguageVersion, WError};

use crate::wrapper::witness_set::{native_script::NativeScript, plutus_script::PlutusScript};

pub fn get_native_script_hash(script: &str) -> Result<String, WError> {
    NativeScript::decode_bytes(&hex::decode(script).map_err(|e| {
        WError::new(
            "WhiskyPallas - Decoding native script:",
            &format!("Hex decode error: {}", e.to_string()),
        )
    })?)
    .map(|ns| ns.hash())
}

pub fn get_script_hash(script: &str, version: LanguageVersion) -> Result<String, WError> {
    match version {
        LanguageVersion::V1 => Ok(
            PlutusScript::<1>::decode_bytes(&hex::decode(script).map_err(|e| {
                WError::new(
                    "WhiskyPallas - Decoding Plutus script:",
                    &format!("Hex decode error: {}", e.to_string()),
                )
            })?)?
            .hash(),
        ),
        LanguageVersion::V2 => Ok(
            PlutusScript::<2>::decode_bytes(&hex::decode(script).map_err(|e| {
                WError::new(
                    "WhiskyPallas - Decoding Plutus script:",
                    &format!("Hex decode error: {}", e.to_string()),
                )
            })?)?
            .hash(),
        ),
        LanguageVersion::V3 => Ok(
            PlutusScript::<3>::decode_bytes(&hex::decode(script).map_err(|e| {
                WError::new(
                    "WhiskyPallas - Decoding Plutus script:",
                    &format!("Hex decode error: {}", e.to_string()),
                )
            })?)?
            .hash(),
        ),
    }
}
