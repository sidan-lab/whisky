use std::str::FromStr;

use pallas::{
    codec::utils::{Bytes, KeepRaw},
    ledger::primitives::{
        conway::{NativeScript, ScriptRef as PallasScriptRef},
        Fragment, PlutusScript,
    },
};

#[derive(Debug, Clone)]
pub enum ScriptRefKind {
    NativeScript { native_script_hex: String },
    PlutusV1Script { plutus_v1_script_hex: String },
    PlutusV2Script { plutus_v2_script_hex: String },
    PlutusV3Script { plutus_v3_script_hex: String },
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptRef<'a> {
    pub inner: PallasScriptRef<'a>,
}

impl<'a> ScriptRef<'a> {
    pub fn new(script_ref_kind: ScriptRefKind) -> Result<Self, String> {
        let pallas_script_ref = match script_ref_kind {
            ScriptRefKind::NativeScript { native_script_hex } => {
                let bytes = hex::decode(native_script_hex)
                    .map_err(|e| format!("Hex decode error: {}", e))?;
                PallasScriptRef::NativeScript(KeepRaw::from(
                    NativeScript::decode_fragment(&bytes)
                        .map_err(|e| format!("Fragment decode error: {}", e))?,
                ))
            }
            ScriptRefKind::PlutusV1Script {
                plutus_v1_script_hex,
            } => PallasScriptRef::PlutusV1Script(PlutusScript::<1>(
                Bytes::from_str(&plutus_v1_script_hex)
                    .map_err(|e| format!("Invalid Plutus V1 script bytes: {}", e))?,
            )),
            ScriptRefKind::PlutusV2Script {
                plutus_v2_script_hex,
            } => PallasScriptRef::PlutusV2Script(PlutusScript::<2>(
                Bytes::from_str(&plutus_v2_script_hex)
                    .map_err(|e| format!("Invalid Plutus V2 script bytes: {}", e))?,
            )),
            ScriptRefKind::PlutusV3Script {
                plutus_v3_script_hex,
            } => PallasScriptRef::PlutusV3Script(PlutusScript::<3>(
                Bytes::from_str(&plutus_v3_script_hex)
                    .map_err(|e| format!("Invalid Plutus V3 script bytes: {}", e))?,
            )),
        };

        Ok(Self {
            inner: pallas_script_ref,
        })
    }
}
