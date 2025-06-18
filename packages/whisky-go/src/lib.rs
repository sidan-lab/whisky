use std::ffi::{c_char, CString};
use whisky_common::TxBuilderBody;
use whisky_common::{TxBuildable, *};
use whisky_csl::WhiskyCSL;

#[repr(C)]
pub struct FfiResult {
    pub status: *mut c_char,
    pub data: *mut c_char,
    pub error: *mut c_char,
}

impl FfiResult {
    pub fn success(data: String) -> Self {
        Self {
            status: CString::new("success").unwrap().into_raw(),
            data: CString::new(data).unwrap().into_raw(),
            error: std::ptr::null_mut(),
        }
    }

    pub fn failure(error_msg: String) -> Self {
        Self {
            status: CString::new("failure").unwrap().into_raw(),
            data: std::ptr::null_mut(),
            error: CString::new(error_msg).unwrap().into_raw(),
        }
    }
}

/// Serializes a transaction body from JSON input and returns a result for Go.
///
/// # Safety
#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn, clippy::blocks_in_conditions)]
pub unsafe extern "C" fn go_serialize_tx_body(
    tx_builder_body_json: *const c_char,
    params_json: *const c_char,
) -> FfiResult {
    // tx buidler body
    let tx_json_str = match { std::ffi::CStr::from_ptr(tx_builder_body_json).to_str() } {
        Ok(s) => s,
        Err(e) => {
            return FfiResult::failure(format!("Invalid UTF-8 in tx_builder_body_json: {:?}", e))
        }
    };

    let tx_builder_body: TxBuilderBody = match serde_json::from_str(tx_json_str) {
        Ok(tx_builder_body) => tx_builder_body,
        Err(e) => return FfiResult::failure(format!("Invalid JSON: {:?}", e)),
    };

    // params
    let params_json_str = match { std::ffi::CStr::from_ptr(params_json).to_str() } {
        Ok(s) => s,
        Err(e) => return FfiResult::failure(format!("Invalid UTF-8 in params_json: {:?}", e)),
    };

    let params: Option<Protocol> = match serde_json::from_str(params_json_str) {
        Ok(params) => Some(params),
        Err(e) => {
            return FfiResult::failure(format!(
                "Invalid Protocol Param JSON: {:?} \n {:?}",
                params_json, e
            ))
        }
    };

    let mut tx_builder = WhiskyCSL::new(params).unwrap();
    tx_builder.tx_builder_body = tx_builder_body;

    match tx_builder.unbalanced_serialize_tx_body() {
        Ok(tx_hex) => FfiResult::success(tx_hex.to_string()),
        Err(e) => FfiResult::failure(format!("{:?}", e.to_string())),
    }
}
