use cardano_serialization_lib::JsError;
use whisky_common::WError;

pub fn from_werror(werror: WError) -> JsError {
    JsError::from_str(&werror.to_string())
}

pub fn to_werror(error_origin: &str, js_error: JsError) -> WError {
    WError::new(error_origin, &self.to_string())
}
