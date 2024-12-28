use crate::{core::builder::serialize_tx_body, *};
use model::{Protocol, TxBuilderBody};
use wasm::WasmResult;

/// ## WASM Transaction building method
///
/// Serialize the transaction body
///
/// ### Arguments
///
/// * `tx_builder_body_json` - The transaction builder body information, serialized as JSON string
/// * `params_json` - Optional protocol parameters, default as Cardano mainnet configuration, serialized as JSON string
///
/// ### Returns
///
/// * `String` - the built transaction hex
#[wasm_bindgen]
pub fn js_serialize_tx_body(tx_builder_body_json: &str, params_json: &str) -> WasmResult {
    let tx_builder_body: TxBuilderBody = match serde_json::from_str(tx_builder_body_json) {
        Ok(tx_builder_body) => tx_builder_body,
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

    match serialize_tx_body(tx_builder_body, params) {
        Ok(tx_hex) => WasmResult::new("success".to_string(), tx_hex.to_string()),
        Err(e) => WasmResult::new_error("failure".to_string(), format!("{:?}", e)),
    }
}

#[test]
fn test_js_serialize_tx_body() {
    let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85","txIndex":3,"amount":[{"unit":"lovelace","quantity":"9891607895"}],"address":"addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh"}}}],"outputs":[{"address":"addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh","amount":[],"datum":null,"referenceScript":null}],"collaterals":[],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[],"votes":[],"network":"mainnet"}"#;
    let params_json = r#"{"epoch":0,"coinsPerUtxoSize":0,"priceMem":0.0,"priceStep":0.0,"minFeeA":0,"minFeeB":0,"keyDeposit":2000000,"maxTxSize":16384,"maxValSize":5000,"poolDeposit":500000000,"maxCollateralInputs":3,"decentralisation":0,"maxBlockSize":65536,"collateralPercent":150,"maxBlockHeaderSize":1100,"minPoolCost":"340000000","maxTxExMem":"14000000","maxTxExSteps":"10000000000","maxBlockExMem":"62000000","maxBlockExSteps":"20000000000","minFeeRefScriptCostPerByte":15}"#;
    let tx_build_result = js_serialize_tx_body(mesh_tx_builder_body_json, params_json);
    println!("{:?}", tx_build_result);
}

#[test]
fn test_js_serialize_tx_body_default_param() {
    let mesh_tx_builder_body_json = r#"{"inputs":[{"simpleScriptTxIn":{"txIn":{"txHash":"cee98215a41d6576bef4c77665c2e911ceeea720877ffde25c046720659e8b36","txIndex":0,"amount":[{"unit":"lovelace","quantity":"301000000"}],"address":"addr1z9wtnlvdd8atlj5c5ylgw4kg5e640vyt8resattrjmwufyvafhxhu32dys6pvn6wlw8dav6cmp4pmtv7cc3yel9uu0nqn30gcl"},"simpleScriptTxIn":{"providedSimpleScriptSource":{"scriptCbor":"830302838200581c188691447471593ad888086cd3cffcb93833f38225ebd56bb19864768200581cf6ed79ef5884bb3ace19f145453e990fb1b0e2a1e736a02c031a18258200581cf1229cc1c389414eee6e195ca71d9e02f23568480947e2a314205c1a"}}}},{"simpleScriptTxIn":{"txIn":{"txHash":"b7f273494e5cef6c11ffbce9db70e32a9e8ded48dcbb376cb6e4f920f4cd5742","txIndex":0,"amount":[{"unit":"lovelace","quantity":"298830000"}],"address":"addr1z9wtnlvdd8atlj5c5ylgw4kg5e640vyt8resattrjmwufyvafhxhu32dys6pvn6wlw8dav6cmp4pmtv7cc3yel9uu0nqn30gcl"},"simpleScriptTxIn":{"providedSimpleScriptSource":{"scriptCbor":"830302838200581c188691447471593ad888086cd3cffcb93833f38225ebd56bb19864768200581cf6ed79ef5884bb3ace19f145453e990fb1b0e2a1e736a02c031a18258200581cf1229cc1c389414eee6e195ca71d9e02f23568480947e2a314205c1a"}}}}],"outputs":[],"collaterals":[],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr1z9wtnlvdd8atlj5c5ylgw4kg5e640vyt8resattrjmwufyvafhxhu32dys6pvn6wlw8dav6cmp4pmtv7cc3yel9uu0nqn30gcl","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[{"simpleScriptCertificate":{"cert":{"dRepRegistration":{"drepId":"drep1ydwtnlvdd8atlj5c5ylgw4kg5e640vyt8resattrjmwufyg4qclf7","coin":500000000,"anchor":{"anchorUrl":"https://2pzt5hfjckkkpzfl.public.blob.vercel-storage.com/drep/drep.collective-Qvzo66SfH5Qb5PGTnHzskz813GexIW.jsonld","anchorDataHash":"693e558c211870145dd31ec5fb0f6b4403cd194d3c3d0da5c5156863165a53c7"}}},"simpleScriptSource":{"providedSimpleScriptSource":{"scriptCbor":"830302838200581c188691447471593ad888086cd3cffcb93833f38225ebd56bb19864768200581cf6ed79ef5884bb3ace19f145453e990fb1b0e2a1e736a02c031a18258200581cf1229cc1c389414eee6e195ca71d9e02f23568480947e2a314205c1a"}}}}],"signingKey":[],"withdrawals":[],"votes":[],"network":"mainnet"}"#;
    let params_json = r#"{"epoch":0,"coinsPerUtxoSize":4310,"priceMem":0.0577,"priceStep":0.0000721,"minFeeA":44,"minFeeB":155381,"keyDeposit":2000000,"maxTxSize":16384,"maxValSize":5000,"poolDeposit":500000000,"maxCollateralInputs":3,"decentralisation":0,"maxBlockSize":65536,"collateralPercent":150,"maxBlockHeaderSize":1100,"minPoolCost":"340000000","maxTxExMem":"14000000","maxTxExSteps":"10000000000","maxBlockExMem":"62000000","maxBlockExSteps":"20000000000","minFeeRefScriptCostPerByte":15}"#;
    let tx_build_result = js_serialize_tx_body(mesh_tx_builder_body_json, params_json);
    println!("{:?}", tx_build_result);
}