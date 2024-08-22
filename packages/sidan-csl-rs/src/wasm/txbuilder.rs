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
    let mesh_tx_builder_body_json = r#"{"inputs":[{"scriptTxIn":{"txIn":{"txHash":"bebcd73af6a397d819490efa16e54a7af93cac8d132032aa7b5277c609937643","txIndex":2,"amount":[{"unit":"lovelace","quantity":"2529970"},{"unit":"2eed08b98bb6b0b5ec15083746dce483f0daba190b35cd7fd0c1018a502d30","quantity":"1"}],"address":"addr_test1wq48zxpyzr40w8afkmrpmhzlaukr42nxeazlddgden8aecqsaydld"},"scriptTxIn":{"scriptSource":{"inlineScriptSource":{"refTxIn":{"txHash":"c2f91bdbac47bdebe728ea999170b1500478eee1a9af6858c4354e8eadbeccb6","txIndex":0},"scriptHash":"","languageVersion":"v2","scriptSize":0}},"datumSource":{"inlineDatumSource":{"txHash":"bebcd73af6a397d819490efa16e54a7af93cac8d132032aa7b5277c609937643","txIndex":2}},"redeemer":{"data":"d87a80","exUnits":{"mem":7000000,"steps":3000000000}}}}},{"pubKeyTxIn":{"txIn":{"txHash":"8d346b70dcb178bd8122dcdab4757135009c9ce82cc0d110229bde0b2dd4fc8a","txIndex":5,"amount":[{"unit":"lovelace","quantity":"695016220"}],"address":"addr_test1qpvx0sacufuypa2k4sngk7q40zc5c4npl337uusdh64kv0uafhxhu32dys6pvn6wlw8dav6cmp4pmtv7cc3yel9uu0nq93swx9"}}}],"outputs":[{"address":"addr_test1wq48zxpyzr40w8afkmrpmhzlaukr42nxeazlddgden8aecqsaydld","amount":[{"unit":"2eed08b98bb6b0b5ec15083746dce483f0daba190b35cd7fd0c1018a502d30","quantity":"1"}],"datum":{"inline":"d8799f9f582035434668374a724e394631506b445a48637362782b787053705571704c6c5370582057475939627a6569623870556e2f6e6747456d305238312b767072617739676958204d4e4b446174647864576b57717167416b526135596543766e7348555139783058203637464661614f5a51436a45345269633157716d6262786a434d516134626b5258205237347169715a66493464334554555745365259627957422f4b6e554c4d316d58204a6230584b4148704d584c66577a57777673662f785a4873515836586156723258206164652f66496a546b705951564d48634f36326b573249576649763546473770544a4438786e4377776772556d5a326c7854413d3dff581c5867c3b8e27840f556ac268b781578b14c5661fc63ee720dbeab663f4d323033393738313133323031330200d87a80ff"},"referenceScript":null},{"address":"addr_test1wq48zxpyzr40w8afkmrpmhzlaukr42nxeazlddgden8aecqsaydld","amount":[{"unit":"2eed08b98bb6b0b5ec15083746dce483f0daba190b35cd7fd0c1018a502d302d31","quantity":"1"}],"datum":{"inline":"d8799f9f582075657a536a4d615a536d754f46307a712b6449477339785863336c384d4d52705820464947456f476c76722f744673484f774f4e36652b577a305239334d6a774d38582034464c615a785a567234627267736c684a364d5164776f4b58594c5668617351582046427a74527178436d66612f532b657077696e6631734a33505877555a6d456258202f505030736179454f613770763177796d653672304b6952564a566f344f414a58204c4470417843664776786a66654d34564b7147686e616873304145773078336c582070784b54667169534f3247427150486d6168586a69757177412f52714175666e5461524d5a585238633142696a356c554b38673d3dff581cc89786195a0534d26c21a9949dfcb8066b7b024e40411fd616b893504d323033393738313133323031330000d87a80ff"},"referenceScript":null}],"collaterals":[{"txIn":{"txHash":"b676676e63bf4fdcd00fd7aa33ca15858b1b202fe60930ab07a4080d9a0b1c65","txIndex":1,"amount":[{"unit":"lovelace","quantity":"5476500"}],"address":"addr_test1qpvx0sacufuypa2k4sngk7q40zc5c4npl337uusdh64kv0uafhxhu32dys6pvn6wlw8dav6cmp4pmtv7cc3yel9uu0nq93swx9"}}],"requiredSignatures":["5867c3b8e27840f556ac268b781578b14c5661fc63ee720dbeab663f"],"referenceInputs":[{"txHash":"bebcd73af6a397d819490efa16e54a7af93cac8d132032aa7b5277c609937643","txIndex":0}],"mints":[{"scriptMint":{"mint":{"policyId":"2eed08b98bb6b0b5ec15083746dce483f0daba190b35cd7fd0c1018a","assetName":"502d302d31","amount":1},"redeemer":{"data":"d87a9f9f582075657a536a4d615a536d754f46307a712b6449477339785863336c384d4d52705820464947456f476c76722f744673484f774f4e36652b577a305239334d6a774d38582034464c615a785a567234627267736c684a364d5164776f4b58594c5668617351582046427a74527178436d66612f532b657077696e6631734a33505877555a6d456258202f505030736179454f613770763177796d653672304b6952564a566f344f414a58204c4470417843664776786a66654d34564b7147686e616873304145773078336c582070784b54667169534f3247427150486d6168586a69757177412f52714175666e5461524d5a585238633142696a356c554b38673d3dff581cc89786195a0534d26c21a9949dfcb8066b7b024e40411fd616b893504d32303339373831313332303133ff","exUnits":{"mem":7000000,"steps":3000000000}},"scriptSource":{"inlineScriptSource":{"refTxIn":{"txHash":"eaf02714482a1e1f98ad96f1cfa6414434cdada6050069f86edf1bb9112db961","txIndex":0},"scriptHash":"","languageVersion":"v2","scriptSize":0}}}}],"changeAddress":"addr_test1qpvx0sacufuypa2k4sngk7q40zc5c4npl337uusdh64kv0uafhxhu32dys6pvn6wlw8dav6cmp4pmtv7cc3yel9uu0nq93swx9","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[],"network":"mainnet"}"#;
    // let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"5b0145fe7b0212a7807e7dba24997049374d965f587300a2039b73cd30806c78","txIndex":1,"amount":[{"unit":"lovelace","quantity":"1132923230"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}}],"outputs":[{"address":"addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8","amount":[{"unit":"lovelace","quantity":"1600000"}],"datum":{"hash":{"type":"Mesh","content":"supersecret"}},"referenceScript":null}],"collaterals":[{"txIn":{"txHash":"ec0c2e70b898cf531b03c9db937602e98c45378d9fa8e8a5b5a91ec5c1d7540d","txIndex":5,"amount":[{"unit":"lovelace","quantity":"5000000"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[]}"#;
    let params_json = r#"{"epoch":0,"coinsPerUtxoSize":0,"priceMem":0,"priceStep":0,"minFeeA":0,"minFeeB":0,"keyDeposit":2000000,"maxTxSize":16384,"maxValSize":5000,"poolDeposit":500000000,"maxCollateralInputs":3,"decentralisation":0,"maxBlockSize":65536,"collateralPercent":150,"maxBlockHeaderSize":1100,"minPoolCost":"340000000","maxTxExMem":"14000000","maxTxExSteps":"10000000000","maxBlockExMem":"62000000","maxBlockExSteps":"20000000000","minFeeRefScriptCostPerByte":15}"#;
    let tx_build_result = js_serialize_tx_body(mesh_tx_builder_body_json, params_json);
    println!("{:?}", tx_build_result);
}
