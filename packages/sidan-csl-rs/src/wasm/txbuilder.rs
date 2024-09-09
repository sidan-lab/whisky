use crate::{core::builder::serialize_tx_body, *};
use model::{Protocol, TxBuilderBody};
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
    let mesh_tx_builder_body: TxBuilderBody = match serde_json::from_str(mesh_tx_builder_body_json)
    {
        Ok(mesh_tx_builder_body) => mesh_tx_builder_body,
        Err(e) => {
            return WasmResult::new_error("failure".to_string(), format!("Invalid JSON: {:?}", e))
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
    let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"d9acd325c0f2422357d8d09109e22ae1a026a2ab25acb39b7842105d374e0bd5","txIndex":0,"amount":[{"unit":"lovelace","quantity":"1000000000"}],"address":"addr1qy70envckyh0q9khppytjncekq97gd9kfgy378e4dhczgfrlktfp6jaqzqepjhdv0z2cx6awa9w2szwz8t8ws9e4w0qqdpx0u2"}}}],"outputs":[],"collaterals":[],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr1qy70envckyh0q9khppytjncekq97gd9kfgy378e4dhczgfrlktfp6jaqzqepjhdv0z2cx6awa9w2szwz8t8ws9e4w0qqdpx0u2","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[{"basicCertificate":{"dRepRegistration":{"drepId":"drep1vkg4qhqt6pyzmtxpvvtjkueak4d7pydsmxz5lwllr53nwezunhg","coin":500000000,"anchor":{"anchorUrl":"https://raw.githubusercontent.com/sidan-lab/DRep/main/sidan-lab.jsonld","anchorDataHash":"0e84662b672077968494f620f91e49e3daaf6c983707ae6bd227709dbd453b56"}}}}],"signingKey":[],"withdrawals":[],"network":"mainnet"}"#;
    // let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"5b0145fe7b0212a7807e7dba24997049374d965f587300a2039b73cd30806c78","txIndex":1,"amount":[{"unit":"lovelace","quantity":"1132923230"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}}],"outputs":[{"address":"addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8","amount":[{"unit":"lovelace","quantity":"1600000"}],"datum":{"hash":{"type":"Mesh","content":"supersecret"}},"referenceScript":null}],"collaterals":[{"txIn":{"txHash":"ec0c2e70b898cf531b03c9db937602e98c45378d9fa8e8a5b5a91ec5c1d7540d","txIndex":5,"amount":[{"unit":"lovelace","quantity":"5000000"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[]}"#;
    let params_json = r#"{"epoch":0,"coinsPerUtxoSize":0,"priceMem":0,"priceStep":0,"minFeeA":0,"minFeeB":0,"keyDeposit":2000000,"maxTxSize":16384,"maxValSize":5000,"poolDeposit":500000000,"maxCollateralInputs":3,"decentralisation":0,"maxBlockSize":65536,"collateralPercent":150,"maxBlockHeaderSize":1100,"minPoolCost":"340000000","maxTxExMem":"14000000","maxTxExSteps":"10000000000","maxBlockExMem":"62000000","maxBlockExSteps":"20000000000","minFeeRefScriptCostPerByte":15}"#;
    let tx_build_result = js_serialize_tx_body(mesh_tx_builder_body_json, params_json);
    println!("{:?}", tx_build_result);
}
