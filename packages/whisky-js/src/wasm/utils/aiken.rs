use crate::{error::from_werror, *};
use whisky_common::BuilderDataType;
use whisky_csl::csl::JsError;
use whisky_pallas::apply_params_to_script;

#[wasm_bindgen]
pub fn js_apply_params_to_script(
    plutus_script: &str,
    params: JsVecString,
    param_type: &str,
) -> Result<String, JsError> {
    let mut params_to_apply: Vec<&str> = vec![];
    for param in params.iter() {
        params_to_apply.push(param);
    }

    let param_type_wdata = match param_type {
        "json" => BuilderDataType::JSON,
        "cbor" => BuilderDataType::CBOR,
        _ => return Err(JsError::from_str("Invalid param type")),
    };

    let param_script = apply_params_to_script(plutus_script, &params_to_apply, param_type_wdata)
        .map_err(from_werror)?;
    Ok(param_script)
}
