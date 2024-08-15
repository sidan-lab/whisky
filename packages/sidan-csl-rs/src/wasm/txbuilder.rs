use crate::{core::builder::serialize_tx_body, *};
use model::{MeshTxBuilderBody, Protocol};
use wasm::WasmResult;

/// ## WASM Transaction building method
///
/// Serialize the transaction body
///
/// ### Arguments
///
/// * `mesh_tx_builder_body_json` - The transaction builder body information, serialized as JSON string
/// * `params_json` - Optional protocol parameters, default as Cardano mainnet configuration, serialized as JSON string
///
/// ### Returns
///
/// * `String` - the built transaction hex
#[wasm_bindgen]
pub fn js_serialize_tx_body(mesh_tx_builder_body_json: &str, params_json: &str) -> WasmResult {
    let mesh_tx_builder_body: MeshTxBuilderBody =
        match serde_json::from_str(mesh_tx_builder_body_json) {
            Ok(mesh_tx_builder_body) => mesh_tx_builder_body,
            Err(e) => {
                return WasmResult::new_error(
                    "failure".to_string(),
                    format!("Invalid JSON: {:?}", e),
                )
            }
        };

    let params: Option<Protocol> = match serde_json::from_str(params_json) {
        Ok(params) => Some(params),
        Err(e) => {
            return WasmResult::new_error(
                "failure".to_string(),
                format!("Invalid Protocol Param JSON: {:?} \n {:?}", params_json, e),
            )
        }
    };

    match serialize_tx_body(mesh_tx_builder_body, params) {
        Ok(tx_hex) => WasmResult::new("success".to_string(), tx_hex.to_string()),
        Err(e) => WasmResult::new_error("failure".to_string(), format!("{:?}", e)),
    }
}

#[test]
fn test_js_serialize_tx_body() {
    let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"989c9d1b383a79dbfc7ff197f0cbf2e38cb35ae2c982ba0b5b0d3bee92be7b19","txIndex":0,"amount":[{"unit":"lovelace","quantity":"1000000000"}],"address":"addr_test1qrjvlw7rzlr33audzdak2dwcjsrp3npa9tqy78e44nmcu5ap2gwze63uc7tk94t4tq0y06nqhr42qdpsw9k06c2qc7tqzjkxjr"}}}],"outputs":[{"address":"addr_test1qrjvlw7rzlr33audzdak2dwcjsrp3npa9tqy78e44nmcu5ap2gwze63uc7tk94t4tq0y06nqhr42qdpsw9k06c2qc7tqzjkxjr","amount":[{"unit":"67dd133868f14107b25772f3c5abaa1e0549f4b400b5e0e3a1136152000643b05465737431","quantity":"1"}],"datum":null,"referenceScript":null},{"address":"addr_test1wpna6yecdrc5zpaj2ae083dt4g0q2j05ksqttc8r5yfkz5sxwvzyq","amount":[{"unit":"67dd133868f14107b25772f3c5abaa1e0549f4b400b5e0e3a1136152000de1405465737431","quantity":"1"}],"datum":{"inline":"d8799fa4446e616d6545546573743145696d6167655835697066733a2f2f516d527a6963705265757477436b4d36616f74754b6a4572464355443231334470775071364279757a4d4a617561496d656469615479706549696d6167652f6a70674b6465736372697074696f6e5348656c6c6f20776f726c64202d20434950363802ff"},"referenceScript":null}],"collaterals":[],"requiredSignatures":["e4cfbbc317c718f78d137b6535d8940618cc3d2ac04f1f35acf78e53"],"referenceInputs":[],"mints":[{"scriptMint":{"mint":{"policyId":"67dd133868f14107b25772f3c5abaa1e0549f4b400b5e0e3a1136152","assetName":"000643b05465737431","amount":1},"redeemer":{"data":"d8799f446d657368ff","exUnits":{"mem":7000000,"steps":3000000000}},"scriptSource":{"providedScriptSource":{"scriptCbor":"5883588101000032323232323232322232533300632323232533300a3370e9000000899b8f375c601c601000e911046d6573680014a0601000260180026018002600800429309b2b19299980319b87480000044c8c94ccc02cc03400852616375c601600260080062c60080044600a6ea80048c00cdd5000ab9a5573aaae7955cfaba15745","languageVersion":"v2"}}}},{"scriptMint":{"mint":{"policyId":"67dd133868f14107b25772f3c5abaa1e0549f4b400b5e0e3a1136152","assetName":"000de1405465737431","amount":1},"redeemer":{"data":"d8799f446d657368ff","exUnits":{"mem":7000000,"steps":3000000000}},"scriptSource":{"providedScriptSource":{"scriptCbor":"5883588101000032323232323232322232533300632323232533300a3370e9000000899b8f375c601c601000e911046d6573680014a0601000260180026018002600800429309b2b19299980319b87480000044c8c94ccc02cc03400852616375c601600260080062c60080044600a6ea80048c00cdd5000ab9a5573aaae7955cfaba15745","languageVersion":"v2"}}}}],"changeAddress":"addr_test1qq7lutcvhg5dl0z4rurdpdy9c0khp7ymel83wf8jt6d9qgap2gwze63uc7tk94t4tq0y06nqhr42qdpsw9k06c2qc7tql6q7cf","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[]}"#;
    // let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"5b0145fe7b0212a7807e7dba24997049374d965f587300a2039b73cd30806c78","txIndex":1,"amount":[{"unit":"lovelace","quantity":"1132923230"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}}],"outputs":[{"address":"addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8","amount":[{"unit":"lovelace","quantity":"1600000"}],"datum":{"hash":{"type":"Mesh","content":"supersecret"}},"referenceScript":null}],"collaterals":[{"txIn":{"txHash":"ec0c2e70b898cf531b03c9db937602e98c45378d9fa8e8a5b5a91ec5c1d7540d","txIndex":5,"amount":[{"unit":"lovelace","quantity":"5000000"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[]}"#;
    let params_json = r#"{"epoch":0,"coinsPerUtxoSize":4310,"priceMem":0.0577,"priceStep":0.0000721,"minFeeA":44,"minFeeB":155381,"keyDeposit":2000000,"maxTxSize":16384,"maxValSize":5000,"poolDeposit":500000000,"maxCollateralInputs":3,"decentralisation":0,"maxBlockSize":98304,"collateralPercent":150,"maxBlockHeaderSize":1100,"minPoolCost":"340000000","maxTxExMem":"16000000","maxTxExSteps":"10000000000","maxBlockExMem":"80000000","maxBlockExSteps":"40000000000"}"#;
    let tx_build_result = js_serialize_tx_body(mesh_tx_builder_body_json, params_json);
    println!("{:?}", tx_build_result);
}
