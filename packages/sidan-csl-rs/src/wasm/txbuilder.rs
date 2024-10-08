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
    // let mesh_tx_builder_body_json = r#"{"inputs":[{"scriptTxIn":{"txIn":{"txHash":"72a934097270cfdf8cdc445deba5928a004c52c78cbfe94c0024d4f043cc0e77","txIndex":0,"amount":[{"unit":"lovelace","quantity":"1017160"}],"address":"addr_test1wrw35u9mkg76cj0h3npuu4d5zjc3kyl57kyx460cu6zu93gpjj7h0"},"scriptTxIn":{"scriptSource":{"providedScriptSource":{"scriptCbor":"59032759032401010033333323232323232322322322322322322533300c323232323253323301230013013375400426464646464a66602e66e1d20000011533301a301937540102a0042c2a66602e600c0022a66603460326ea8020540085854ccc05ccdc3a40080022a66603460326ea8020540085854ccc05ccdc3a400c0022a66603460326ea8020540085858c05cdd5003899191929919980c99b87480000104cc008dd59801980d9baa00c30013301d375202a97ae01533301930080041330023756600660366ea8030c004cc074dd4809a5eb8054ccc064cdc3a40080082660046eacc00cc06cdd500618009980e9ba90114bd7009919299980d99198008009bac3021302230223022302200322533302000114a0264a66603c66e3cdd7181180100e0a511330030030013023001100114a064a66603c002294054ccc078c0840044c8c8c94ccc078cdc79bae30230030141533301e3371e002038266e1c009200114a02940dd7181118118011bad302130223022001375860400022940c8cc004004dd5981000111299980f8008a5eb804c8ccc888c8cc00400400c894ccc094004400c4c8cc09cdd3998139ba90063302730240013302730250014bd7019801801981480118138009bae301e0013756603e0026600600660460046042002603e603e603e603e60366ea8030dd2a400444646600200200644a66603c00229404c94ccc070cdd7802180e98108010a5113300300300130210012301c301d301d301d301d301d301d0013016375400c60326034004603000260286ea8008dc3a40042c602a602c004602800260280046024002601c6ea800452613656375c0026eb8004dd70009bae001375c002ae6955ceaab9e5573eae815d0aba24c011e581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604ae004c011e581c3dc7e4d31bf34d68a822712c47bbf544d74c22fe2350ae7bb51a3f02004c011e581c3dc7e4d31bf34d68a822712c47bbf544d74c22fe2350ae7bb51a3f02004c011e581c18c154376a49c965027adb6afbe97ce43c96a2f9b8bf0eaf9ff15851004c011e581ccc0932775ffd17eba5738509ad28e45637b3d3d45d182d5f1fbf9fe20001","languageVersion":"v3"}},"datumSource":{"inlineDatumSource":{"txHash":"72a934097270cfdf8cdc445deba5928a004c52c78cbfe94c0024d4f043cc0e77","txIndex":0}},"redeemer":{"data":"d87980","exUnits":{"mem":7000000,"steps":3000000000}}}}},{"pubKeyTxIn":{"txIn":{"txHash":"3c05db00dfec497ce870faaa67e6176f8e08352689bee93e203c12cc800c3ebc","txIndex":4,"amount":[{"unit":"lovelace","quantity":"106756821"}],"address":"addr_test1qrgkr4jwau8wkk0ezf84yruv3uahzlksgxvd2nytzlnqft4x8s2nlvl23f82fut92a82jytnw4k7p0esygk2p626vjdqu58u9n"}}}],"outputs":[],"collaterals":[{"txIn":{"txHash":"3c05db00dfec497ce870faaa67e6176f8e08352689bee93e203c12cc800c3ebc","txIndex":0,"amount":[{"unit":"lovelace","quantity":"5000000"}],"address":"addr_test1qrgkr4jwau8wkk0ezf84yruv3uahzlksgxvd2nytzlnqft4x8s2nlvl23f82fut92a82jytnw4k7p0esygk2p626vjdqu58u9n"}}],"requiredSignatures":["d161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604ae","5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa"],"referenceInputs":[{"txHash":"8d68748457cd0f1a8596f41fd2125a415315897d2da4a4b94335829cee7198ae","txIndex":0}],"mints":[],"changeAddress":"addr_test1qrgkr4jwau8wkk0ezf84yruv3uahzlksgxvd2nytzlnqft4x8s2nlvl23f82fut92a82jytnw4k7p0esygk2p626vjdqu58u9n","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[{"plutusScriptWithdrawal":{"address":"stake_test17q7u0exnr0e5669gyfcjc3am74zdwnpzlc34ptnmk5dr7qszs53fr","coin":0,"scriptSource":{"inlineScriptSource":{"refTxIn":{"txHash":"f75b8c0ce490b11ba002bc7ed483119665eb8b52d9a11afb0b4a448ca2663d52","txIndex"#;
    let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"5b0145fe7b0212a7807e7dba24997049374d965f587300a2039b73cd30806c78","txIndex":1,"amount":[{"unit":"lovelace","quantity":"1132923230"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}}],"outputs":[{"address":"addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8","amount":[{"unit":"lovelace","quantity":"1600000"}],"datum":{"hash":{"type":"Mesh","content":"supersecret"}},"referenceScript":null}],"collaterals":[{"txIn":{"txHash":"ec0c2e70b898cf531b03c9db937602e98c45378d9fa8e8a5b5a91ec5c1d7540d","txIndex":5,"amount":[{"unit":"lovelace","quantity":"5000000"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[]}"#;
    let params_json = r#"{"epoch":0,"coinsPerUtxoSize":0,"priceMem":0,"priceStep":0,"minFeeA":0,"minFeeB":0,"keyDeposit":2000000,"maxTxSize":16384,"maxValSize":5000,"poolDeposit":500000000,"maxCollateralInputs":3,"decentralisation":0,"maxBlockSize":65536,"collateralPercent":150,"maxBlockHeaderSize":1100,"minPoolCost":"340000000","maxTxExMem":"14000000","maxTxExSteps":"10000000000","maxBlockExMem":"62000000","maxBlockExSteps":"20000000000","minFeeRefScriptCostPerByte":15}"#;
    let tx_build_result = js_serialize_tx_body(mesh_tx_builder_body_json, params_json);
    println!("{:?}", tx_build_result);
}
