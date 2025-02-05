use crate::*;
mod transaction;
mod txbuilder;
mod txparser;
use cardano_serialization_lib::JsError;
pub use transaction::*;
pub use txbuilder::*;
pub use txparser::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct WasmResult {
    status: String,
    data: String,
    error: String,
}

#[wasm_bindgen]
impl WasmResult {
    pub fn new(status: String, data: String) -> Self {
        Self {
            status,
            data,
            error: "".to_string(),
        }
    }

    pub fn new_error(status: String, error: String) -> Self {
        Self {
            status,
            data: "".to_string(),
            error,
        }
    }

    #[wasm_bindgen]
    pub fn get_status(&self) -> String {
        self.status.clone()
    }

    #[wasm_bindgen]
    pub fn get_data(&self) -> String {
        self.data.clone()
    }

    #[wasm_bindgen]
    pub fn get_error(&self) -> String {
        self.error.clone()
    }
}

impl WasmResult {
    pub fn from_result(result: Result<String, JsError>) -> Self {
        match result {
            Ok(data) => Self::new("success".to_string(), data),
            Err(e) => Self::new_error("failure".to_string(), format!("{:?}", e)),
        }
    }
}
