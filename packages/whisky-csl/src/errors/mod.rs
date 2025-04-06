use cardano_serialization_lib::JsError;
use whisky_common::WError;

pub fn from_werror(werror: WError) -> JsError {
    JsError::from_str(&werror.to_string())
}

pub fn to_werror<F>(error_origin: &'static str) -> impl FnOnce(F) -> WError
where
    F: std::fmt::Display,
{
    move |err| WError::new(error_origin, &self.to_string())
}
