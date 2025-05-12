use pallas_codec::minicbor;
use uplc::Fragment;
use whisky_common::*;

pub fn apply_double_cbor_encoding(script: &str) -> Result<String, WError> {
    let mut double_encoded_script_buf: Vec<u8> = vec![];
    let mut encoder = minicbor::Encoder::new(&mut double_encoded_script_buf);
    encoder
        .bytes(&hex_to_bytes(script)?)
        .map_err(WError::from_err(
            "apply_double_cbor_encoding - encoding error",
        ))?;
    Ok(bytes_to_hex(&double_encoded_script_buf))
}

pub fn apply_params_to_script(
    plutus_script: &str,
    params_to_apply: &[&str],
) -> Result<String, WError> {
    let param_list_bytes: Vec<pallas_primitives::PlutusData> = params_to_apply
        .iter()
        .map(|str_param| {
            let param_bytes = hex_to_bytes(str_param).map_err(WError::from_err(
                "apply_params_to_script - hex to bytes error, one of the params has an invalid hex",
            )).unwrap();
            pallas_primitives::PlutusData::decode_fragment(&param_bytes)
                .map_err(WError::from_err(
                    "apply_params_to_script - decoding param error",
                ))
                .unwrap()
        })
        .collect();
    let param_list: pallas_primitives::PlutusData = pallas_primitives::PlutusData::Array(
        pallas_primitives::MaybeIndefArray::Indef(param_list_bytes),
    );
    let bytes = uplc::tx::apply_params_to_script(
        &param_list.encode_fragment().unwrap(),
        &hex_to_bytes(plutus_script).unwrap(),
    )
    .map_err(WError::from_err(
        "apply_params_to_script - apply params to script error",
    ))?;

    Ok(
        apply_double_cbor_encoding(&bytes_to_hex(&bytes)).map_err(WError::from_err(
            "apply_params_to_script - double cbor encoding error",
        ))?,
    )
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

#[test]
fn test_apply_param_to_script() {
    let script = "583401010022332259800a518a4d15330024911856616c696461746f722072657475726e65642066616c736500136564004ae715cd01";
    assert_eq!(
        apply_params_to_script(script, &["182a"]).unwrap(),
        "583d583b010100322332259800a518a4d153300249011856616c696461746f722072657475726e65642066616c736500136564004ae715cd260102182a0001"
    );
}
