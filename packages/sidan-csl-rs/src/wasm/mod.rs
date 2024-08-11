use crate::*;
mod transaction;
mod txbuilder;
use cardano_serialization_lib::JsError;
pub use transaction::*;
pub use txbuilder::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct WasmResult<T> {
    status: String,
    data: T,
    error: String,
}

#[wasm_bindgen]
impl<T: Clone + Default> WasmResult<T> {
    pub fn new(status: String, data: T) -> Self {
        Self {
            status,
            data,
            error: String::default(),
        }
    }

    pub fn new_error(status: String, error: String) -> Self {
        Self {
            status,
            data: T::default(),
            error,
        }
    }

    pub fn from_result(result: Result<T, JsError>) -> Self {
        match result {
            Ok(data) => Self::new("success".to_string(), data),
            Err(e) => Self::new_error("failure".to_string(), format!("{:?}", e)),
        }
    }

    #[wasm_bindgen]
    pub fn get_status(&self) -> String {
        self.status.clone()
    }

    #[wasm_bindgen]
    pub fn get_data(&self) -> T {
        self.data.clone()
    }

    #[wasm_bindgen]
    pub fn get_error(&self) -> String {
        self.error.clone()
    }
}
