use whisky_common::{
    Asset, Budget, DatumSource, InlineDatumSource, ProvidedScriptSource, PubKeyTxIn, Redeemer,
    ScriptTxIn, ScriptTxInParameter, TxBuilderBody, TxIn, TxInParameter, ValidityRange,
};
use whisky_pallas::tx_builder::core_pallas::CorePallas;

#[test]
fn test_from_tx_builder_body() {
    let tx_builder_body = TxBuilderBody {
        inputs: vec![TxIn::PubKeyTxIn(PubKeyTxIn {
            tx_in: TxInParameter {
                tx_hash: "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052".to_string(),
                tx_index: 0,
                amount: Some(vec![Asset::new_from_str("lovelace", "4633697637")]),
                address: Some("addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj".to_string())
            }
        }), 
        TxIn::ScriptTxIn(ScriptTxIn {
            tx_in: TxInParameter {
                tx_hash: "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052".to_string(),
                tx_index: 1,
                amount: Some(vec![Asset::new_from_str("lovelace", "4633697637")]),
                address: Some("addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj".to_string())
            },
            script_tx_in: ScriptTxInParameter {
                script_source: Some(whisky_common::ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                    script_cbor: "525101010023259800a518a4d136564004ae69".to_string(),
                    language_version: whisky_common::LanguageVersion::V3
                })),
                datum_source: Some(DatumSource::InlineDatumSource(InlineDatumSource {
                    tx_hash: "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052".to_string(),
                    tx_index: 1,
                })),
                redeemer: Some(Redeemer {
                    data: "d87980".to_string(),
                    ex_units: Budget {
                        mem: 0,
                        steps:0
                    }
                })
            }
        })],
        outputs: vec![],
        collaterals: vec![],
        reference_inputs: vec![],
        withdrawals: vec![],
        mints: vec![],
        certificates: vec![],
        votes: vec![],
        change_address: "addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj".to_string(),
        fee: Some(0.to_string()),
        required_signatures: vec![],
        change_datum: None,
        metadata: vec![],
        validity_range: ValidityRange {
            invalid_before: None,
            invalid_hereafter: None,
        },
        signing_key: vec![],
        network: None,
        total_collateral: None,
        collateral_return_address: None,
    };
    let mut core_pallas = CorePallas::new(tx_builder_body, 100);
    let result = core_pallas.build_tx();
    assert!(result.is_ok());
    println!("Serialized transaction hex: {}", result.unwrap());
}
