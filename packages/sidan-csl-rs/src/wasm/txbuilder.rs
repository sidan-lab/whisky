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
    let mesh_tx_builder_body_json = r#"{"inputs":[{"scriptTxIn":{"txIn":{"txHash":"51e5fa01fec20243a16c3cd45a86b9de22761d6aebe7ca7c904664a17a317416","txIndex":2,"amount":[{"unit":"bdeb17d07ea41c2ccaa923fc9a97f1036f09d77f062ae7932f838d9e","quantity":"1"},{"unit":"lovelace","quantity":"1323170"}],"address":"addr_test1wrh7mlyk9s540ue3puvax3qrg4spvsceel7fz6d3g2cvg3sh9pzy3"},"scriptTxIn":{"scriptSource":{"providedScriptSource":{"scriptCbor":"58b258b0010100333232323232232225333005323232323232330010013758601c601e601e601e601e601e601e601e601e60186ea8c038c030dd50039129998070008a50132533300d3371e6eb8c04000802c52889980180180098080009806180680118058009805801180480098031baa00114984d958dd7000ab9a5573caae7d5d0aba24c011e581cfa5136e9e9ecbc9071da73eeb6c9a4ff73cbf436105cf8380d1c525c004c010847362d7370656e640001","languageVersion":"v3"}},"datumSource":{"inlineDatumSource":{"txHash":"51e5fa01fec20243a16c3cd45a86b9de22761d6aebe7ca7c904664a17a317416","txIndex":2}},"redeemer":{"data":"d87b80","exUnits":{"mem":7000000,"steps":3000000000}}}}},{"scriptTxIn":{"txIn":{"txHash":"54a4c620467373a2d1b294fdfa6d53d995ae8d074c5b477306a999bd0067896b","txIndex":1,"amount":[{"unit":"57dd033da7865978bd4224234c58f38d387e78f3abcfac760dbea6e4","quantity":"1"},{"unit":"lovelace","quantity":"1323170"}],"address":"addr_test1wpau6rysxjgkf4qj8rsyr334ty35sr4pn6zrn2cz2ne63yqeptp8a"},"scriptTxIn":{"scriptSource":{"providedScriptSource":{"scriptCbor":"58b258b0010100333232323232232225333005323232323232330010013758601c601e601e601e601e601e601e601e601e60186ea8c038c030dd50039129998070008a50132533300d3371e6eb8c04000802c52889980180180098080009806180680118058009805801180480098031baa00114984d958dd7000ab9a5573caae7d5d0aba24c011e581cfa5136e9e9ecbc9071da73eeb6c9a4ff73cbf436105cf8380d1c525c004c010847352d7370656e640001","languageVersion":"v3"}},"datumSource":{"inlineDatumSource":{"txHash":"54a4c620467373a2d1b294fdfa6d53d995ae8d074c5b477306a999bd0067896b","txIndex":1}},"redeemer":{"data":"d87d80","exUnits":{"mem":7000000,"steps":3000000000}}}}}],"outputs":[{"address":"addr_test1wpau6rysxjgkf4qj8rsyr334ty35sr4pn6zrn2cz2ne63yqeptp8a","amount":[{"unit":"bdeb17d07ea41c2ccaa923fc9a97f1036f09d77f062ae7932f838d9e","quantity":"1"},{"unit":"lovelace","quantity":"1323170"}],"datum":{"inline":"d8799f58200000000000000000000000000000000000000000000000000000000000000000ff"},"referenceScript":null},{"address":"addr_test1wpau6rysxjgkf4qj8rsyr334ty35sr4pn6zrn2cz2ne63yqeptp8a","amount":[{"unit":"57dd033da7865978bd4224234c58f38d387e78f3abcfac760dbea6e4","quantity":"1"},{"unit":"lovelace","quantity":"1323170"}],"datum":{"inline":"d8799f58200000000000000000000000000000000000000000000000000000000000000000ff"},"referenceScript":null},{"address":"addr_test1wz0a74329m3tdc5dgkxpatzc7vkxwjtp4rlg2h47ngyfp9qpqyzes","amount":[{"unit":"3ef107f3846191c353d00a7a6c7320771d11703593d7506d5bad85fc","quantity":"1"}],"datum":{"inline":"d8799f581c04845038ee499ee8bc0afe56f688f27b2dd76f230d3698a9afcc1b66a140a1401a02faf080ff"},"referenceScript":null}],"collaterals":[{"txIn":{"txHash":"83fc6b20417455242112b682137346048d16cf2a3ec1674f6e54ec06b42501b4","txIndex":0,"amount":[],"address":"addr_test1vra9zdhfa8kteyr3mfe7adkf5nlh8jl5xcg9e7pcp5w9yhq5exvwh"}}],"requiredSignatures":["fa5136e9e9ecbc9071da73eeb6c9a4ff73cbf436105cf8380d1c525c"],"referenceInputs":[],"mints":[{"scriptMint":{"mint":{"policyId":"3ef107f3846191c353d00a7a6c7320771d11703593d7506d5bad85fc","assetName":"","amount":1},"redeemer":{"data":"d87980","exUnits":{"mem":7000000,"steps":3000000000}},"scriptSource":{"providedScriptSource":{"scriptCbor":"58b158af010100333232323232232225333005323232323232330010013758601c601e601e601e601e601e601e601e601e60186ea8c038c030dd50039129998070008a50132533300d3371e6eb8c04000802c52889980180180098080009806180680118058009805801180480098031baa00114984d958dd7000ab9a5573caae7d5d0aba24c011e581cfa5136e9e9ecbc9071da73eeb6c9a4ff73cbf436105cf8380d1c525c004c010746392d6d696e740001","languageVersion":"v3"}}}}],"changeAddress":"addr_test1qra9zdhfa8kteyr3mfe7adkf5nlh8jl5xcg9e7pcp5w9yhyf5tek6vpnha97yd5yw9pezm3wyd77fyrfs3ynftyg7njs5cfz2x","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[{"plutusScriptWithdrawal":{"address":"stake_test17r6tcnyl7kgclhuc5zney663we9tzdgdeh62cxx03xh7hvsuragew","coin":0,"scriptSource":{"providedScriptSource":{"scriptCbor":"58b758b5010100333232323232232225333005323232323232330010013758601c601e601e601e601e601e601e601e601e60186ea8c038c030dd50039129998070008a50132533300d3371e6eb8c04000802c52889980180180098080009806180680118058009805801180480098031baa00114984d958dd7000ab9a5573caae7d5d0aba24c011e581cfa5136e9e9ecbc9071da73eeb6c9a4ff73cbf436105cf8380d1c525c004c010d4c312d68796472612d6f70656e0001","languageVersion":"v3"}},"redeemer":{"data":"d8799f4040ff","exUnits":{"mem":7000000,"steps":3000000000}}}}],"votes":[],"network":"mainnet"}"#;
    // let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"5b0145fe7b0212a7807e7dba24997049374d965f587300a2039b73cd30806c78","txIndex":1,"amount":[{"unit":"lovelace","quantity":"1132923230"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}}],"outputs":[{"address":"addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8","amount":[{"unit":"lovelace","quantity":"1600000"}],"datum":{"hash":{"type":"Mesh","content":"supersecret"}},"referenceScript":null}],"collaterals":[{"txIn":{"txHash":"ec0c2e70b898cf531b03c9db937602e98c45378d9fa8e8a5b5a91ec5c1d7540d","txIndex":5,"amount":[{"unit":"lovelace","quantity":"5000000"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[]}"#;
    let params_json = r#"{"epoch":0,"coinsPerUtxoSize":0,"priceMem":0.0,"priceStep":0.0,"minFeeA":0,"minFeeB":0,"keyDeposit":2000000,"maxTxSize":16384,"maxValSize":5000,"poolDeposit":500000000,"maxCollateralInputs":3,"decentralisation":0,"maxBlockSize":65536,"collateralPercent":150,"maxBlockHeaderSize":1100,"minPoolCost":"340000000","maxTxExMem":"14000000","maxTxExSteps":"10000000000","maxBlockExMem":"62000000","maxBlockExSteps":"20000000000","minFeeRefScriptCostPerByte":15}"#;
    let tx_build_result = js_serialize_tx_body(mesh_tx_builder_body_json, params_json);
    println!("{:?}", tx_build_result);
}
