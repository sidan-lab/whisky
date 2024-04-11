use wasm_bindgen::JsError;

use crate::model::Action;

pub trait IEvaluator {
    fn evaluate_tx(&self, tx: String) -> Result<Vec<Action>, JsError>;
}
