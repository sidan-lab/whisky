use whisky_common::WError;
use whisky_csl::csl::JsError;

pub fn from_werror(werror: WError) -> JsError {
    JsError::from_str(&format!("{:?}", werror))
}
