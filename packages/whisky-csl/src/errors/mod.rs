use cardano_serialization_lib::JsError;
use whisky_common::WError;

pub fn from_werror(werror: WError) -> JsError {
    JsError::from_str(&werror.to_string())
}
