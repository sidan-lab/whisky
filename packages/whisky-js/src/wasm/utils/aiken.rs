use crate::*;
use whisky_common::*;
use whisky_csl::apply_params_to_script;

#[wasm_bindgen]
pub fn js_apply_params_to_script(
    plutus_script: &str,
    params: JsVecString,
    param_type: BuilderDataType,
) -> Result<String, WError> {
    let mut params_to_apply: Vec<&str> = vec![];
    for param in params.iter() {
        params_to_apply.push(param);
    }
    let param_script = apply_params_to_script(plutus_script, &params_to_apply, param_type)?;
    Ok(param_script)
}
