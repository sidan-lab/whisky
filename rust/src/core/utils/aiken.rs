use crate::{
    core::common::{bytes_to_hex, hex_to_bytes},
    csl::{
        error::JsError,
        plutus::{PlutusData, PlutusDatumSchema, PlutusList, PlutusScript},
    },
    *,
};

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct AikenScriptParams(Vec<String>);

#[wasm_bindgen]
impl AikenScriptParams {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> String {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: String) {
        self.0.push(elem.clone());
    }
}

pub fn apply_double_cbor_encoding(script: &str) -> Result<String, JsError> {
    let bytes: Vec<u8> = hex_to_bytes(script).unwrap();

    match PlutusScript::from_bytes(bytes.clone()) {
        Ok(single_encoded_script) => {
            match PlutusScript::from_bytes(single_encoded_script.bytes()) {
                Ok(_) => Ok(script.to_string()),
                Err(_) => {
                    let bytes = PlutusScript::new(bytes).to_bytes();
                    let new_script = bytes_to_hex(&bytes);
                    Ok(new_script)
                }
            }
        }
        Err(err) => Err(JsError::from(err)),
    }
}

#[test]
fn test_apply_double_cbor_encoding() {
    let script =
      "584501000032323232323222533300432323253330073370e900018041baa0011324a2600c0022c60120026012002600600229309b2b118021baa0015734aae7555cf2ba157441";
    assert_eq!(
      apply_double_cbor_encoding(script).unwrap(),
      "5847584501000032323232323222533300432323253330073370e900018041baa0011324a2600c0022c60120026012002600600229309b2b118021baa0015734aae7555cf2ba157441"
  );
}

#[wasm_bindgen]
pub fn apply_params_to_script(
    // params_to_apply: Vec<String>,
    params_to_apply: AikenScriptParams,
    plutus_script: String,
) -> Result<String, JsError> {
    let params_to_apply = params_to_apply.0;
    let double_encoded_script = apply_double_cbor_encoding(&plutus_script).unwrap();
    let plutus_script =
        PlutusScript::from_bytes(hex_to_bytes(&double_encoded_script).unwrap()).unwrap();
    let mut plutus_list = PlutusList::new();
    for param in &params_to_apply {
        let plutus_data = PlutusData::from_json(param, PlutusDatumSchema::DetailedSchema).unwrap();
        plutus_list.add(&plutus_data);
    }
    let bytes = apply_params_to_plutus_script(&plutus_list, plutus_script)?.to_bytes();
    Ok(bytes_to_hex(&bytes))
}

#[test]
fn test_apply_params_to_script() {
    use serde_json::{json, to_string};
    let script =
      "584501000032323232323222533300432323253330073370e900018041baa0011324a2600c0022c60120026012002600600229309b2b118021baa0015734aae7555cf2ba157441";
    let params = vec![to_string(&json!({ "bytes": "1234"})).unwrap()];

    assert_eq!(
        apply_params_to_script(AikenScriptParams(params), script.to_string()).unwrap(),
        "584f584d010000332323232323222533300432323253330073370e900018041baa0011324a2600c0022c60120026012002600600229309b2b118021baa0015734aae7555cf2ba157449801034212340001"
    );
}

pub fn apply_params_to_plutus_script(
    params: &PlutusList,
    plutus_script: PlutusScript,
) -> Result<PlutusScript, JsError> {
    match uplc::tx::apply_params_to_script(&params.to_bytes(), &plutus_script.bytes()) {
        Ok(res) => Ok(PlutusScript::new(res)),
        Err(err) => Err(JsError::from_str(&err.to_string())),
    }
}
