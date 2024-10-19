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
    let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"e637646b721e730e7f8f1336cb46736c796d069e4af9b5ad175f3805fd4253ef","txIndex":0,"amount":[{"unit":"lovelace","quantity":"3008380"}],"address":"addr_test1vpvx0sacufuypa2k4sngk7q40zc5c4npl337uusdh64kv0c7e4cxr"}}},{"pubKeyTxIn":{"txIn":{"txHash":"bb5c5a24c7bbfb819e5560e73b5bfa22982cb2bced632eb25d6ee7ed5a6b6e60","txIndex":1,"amount":[{"unit":"lovelace","quantity":"47321359"}],"address":"addr_test1qpvx0sacufuypa2k4sngk7q40zc5c4npl337uusdh64kv0uafhxhu32dys6pvn6wlw8dav6cmp4pmtv7cc3yel9uu0nq93swx9"}}}],"outputs":[{"address":"addr_test1vpvx0sacufuypa2k4sngk7q40zc5c4npl337uusdh64kv0c7e4cxr","amount":[],"datum":null,"referenceScript":{"providedScriptSource":{"scriptCbor":"5906d05906cd01010032323232323232253330023232323232533233008300130093754004264646464646464a66601e60060022a66602460226ea8028540085854ccc03cc02000454ccc048c044dd50050a8010b0a9998079802000899191919299980b180c8010a8030b1bae30170013017002375c602a00260226ea802854ccc03ccdc3a400c0022a66602460226ea8028540085858c03cdd5004899191919192999809180318099baa008132323232323232323232323232323232323232323232325333029301d00713333001019011300200f4800854ccc0a4c08801c4cccc004064034c00802d2002132533302a301f00813232323232325333033303600213253330313026303237540022646464a6660686050606a6ea800c4c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c94ccc12cc1380084cc0540444cc0540344cc05401c4c94ccc124c8cc004004cc138dd4815198271ba90294bd701129998270008a51132533304c3302303b375c60a20042660060060022940c144004400452819baf0173232323232323230223305330540073305330540063305330540053305330540043305330540033305330540023305330540013305330543055001330533752048660a66ea408d2f5c060a800260a600260a400260a200260a0002609e00260946ea80d858dd7182600098260011bae304a001304a002375a60900026090004608c002608c0046eb8c110004c110008dd698210009821001182000098200011bae303e001303e002303c001303c002375c6074002606c6ea800c58894ccc0d4c0a4c0d8dd5001099191919299981e181f80109980380189919299981d9817800899192999820182180109919299981f981980089919299982218238010998078008a8020b182280098209baa0031533303f303800113232323232325333048304b0021500816375a609200260920046eb4c11c004c11c008dd6982280098209baa00316303f37540042a0082c6082002607a6ea800c54ccc0ecc0d000454ccc0f8c0f4dd50018a8010b0b181d9baa0021500516303d001303d002303b001303737540042c4464a66606a605200226464a666074607a0042a0082c6eb8c0ec004c0dcdd50018a99981a981700089919299981d181e8010a8020b1bae303b001303737540062c606a6ea8008c0d8c0ccdd50008b181a981b181b18191baa001163034001323300100102122533303300114bd70099192999819299981919baf3037303437540040382605664a666066605860686ea8004520001375a6070606a6ea8004c94ccc0ccc0b0c0d0dd50008a60103d87a80001323300100137566072606c6ea8008894ccc0e0004530103d87a800013232323253330393372291100002153330393371e9101000021300c3303d375000297ae014c0103d87a8000133006006003375a60740066eb8c0e0008c0f0008c0e8004c8cc004004dd5981c181c981a9baa00322533303700114c103d87a80001323232325333038337220480042a66607066e3c0900084c02ccc0f0dd3000a5eb80530103d87a8000133006006003375660720066eb8c0dc008c0ec008c0e400452809981b00119802002000899802002000981b801181a8009ba548000dd7181898190011bae3030001302c375404a2a666054660020320122666600403402c91100480045281119198008008019129998180008a50132533302e3371e6eb8c0cc008010528899801801800981980091111929998180008a50153330303033001132323253330303371e6eb8c0d400c01c54ccc0c0cdc7800803099b8700200514a02940dd7181a181a8011bad303330343034001375860640022940c8cc004004014894ccc0c400452f5c0264666444646600200200644a66606e0022006264660726e9ccc0e4dd48031981c981b0009981c981b800a5eb80cc00c00cc0ec008c0e4004dd718180009bab30310013300300330350023033001232337140029101012900337149110a5265676973747279202800330020013300400148008c0040048894ccc0a0c0840044cdc599b80002481812210013371666e00cdc1801000a40c066600600666e18008004cdc1800a40286002002444a66604c66e2000920141001133300300333706004900a19b8200148050c08cdd500e9bae30263027002375c604a002604a0046eb4c08c004c08cc08c008dd7181080098108011bad301f001301f301f002375c603a002603a004603600260360046eb8c064004c054dd5000980b980a1baa008163758602c602e602e602e602e0046eacc054004c054c054008dd618098009809980998079baa00a370e90001b8748010c03cc040008c038004c028dd50011b874800858c02cc030008c028004c028008c020004c010dd50008a4c26cacae6955ceaab9e5573eae815d0aba201","languageVersion":"v3"}}}],"collaterals":[],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr_test1qpvx0sacufuypa2k4sngk7q40zc5c4npl337uusdh64kv0uafhxhu32dys6pvn6wlw8dav6cmp4pmtv7cc3yel9uu0nq93swx9","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[],"votes":[],"network":"preprod"}"#;
    // let mesh_tx_builder_body_json = r#"{"inputs":[{"pubKeyTxIn":{"txIn":{"txHash":"5b0145fe7b0212a7807e7dba24997049374d965f587300a2039b73cd30806c78","txIndex":1,"amount":[{"unit":"lovelace","quantity":"1132923230"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}}],"outputs":[{"address":"addr_test1wpnlxv2xv9a9ucvnvzqakwepzl9ltx7jzgm53av2e9ncv4sysemm8","amount":[{"unit":"lovelace","quantity":"1600000"}],"datum":{"hash":{"type":"Mesh","content":"supersecret"}},"referenceScript":null}],"collaterals":[{"txIn":{"txHash":"ec0c2e70b898cf531b03c9db937602e98c45378d9fa8e8a5b5a91ec5c1d7540d","txIndex":5,"amount":[{"unit":"lovelace","quantity":"5000000"}],"address":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn"}}],"requiredSignatures":[],"referenceInputs":[],"mints":[],"changeAddress":"addr_test1qq0yavv5uve45rwvfaw96qynrqt8ckpmkwcg08vlwxxdncxk82f5wz75mzaesmqzl79xqsmedwgucwtuav5str6untqqmykcpn","metadata":[],"validityRange":{"invalidBefore":null,"invalidHereafter":null},"certificates":[],"signingKey":[],"withdrawals":[]}"#;
    let params_json = r#"{"epoch":0,"coinsPerUtxoSize":4310,"priceMem":0.0577,"priceStep":0.0000721,"minFeeA":44,"minFeeB":155381,"keyDeposit":2000000,"maxTxSize":16384,"maxValSize":5000,"poolDeposit":500000000,"maxCollateralInputs":3,"decentralisation":0,"maxBlockSize":65536,"collateralPercent":150,"maxBlockHeaderSize":1100,"minPoolCost":"340000000","maxTxExMem":"14000000","maxTxExSteps":"10000000000","maxBlockExMem":"62000000","maxBlockExSteps":"20000000000","minFeeRefScriptCostPerByte":15}"#;
    let tx_build_result = js_serialize_tx_body(mesh_tx_builder_body_json, params_json);
    println!("{:?}", tx_build_result);
}
