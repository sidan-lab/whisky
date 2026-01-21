use std::str::FromStr;

use pallas::{
    codec::utils::Bytes,
    ledger::primitives::{conway::Tx, Fragment},
};
use whisky_common::{
    Asset, Budget,
    Certificate::{self},
    CertificateType, DRepRegistration, Datum, DatumSource, InlineDatumSource, MintItem,
    MintParameter, Output, Protocol, ProvidedScriptSource, PubKeyTxIn, Redeemer, RefTxIn,
    ScriptMint, ScriptSource, ScriptTxIn, ScriptTxInParameter, TxBuilderBody, TxIn, TxInParameter,
    ValidityRange,
};
use whisky_pallas::{converter::bytes_from_bech32, tx_builder::core_pallas::CorePallas};

#[test]
fn test_from_tx_builder_body() {
    let tx_builder_body = TxBuilderBody {
        inputs: vec![
            TxIn::PubKeyTxIn(PubKeyTxIn {
                tx_in: TxInParameter {
                    tx_hash: "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052".to_string(),
                    tx_index: 0,
                    amount: Some(vec![Asset::new_from_str("lovelace", "4633697637"), Asset::new_from_str("0f6b02150cbcc7fedafa388abcc41635a9443afb860100099ba40f07", "1")]),
                    address: Some("addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj".to_string()),
                },
            }),
            TxIn::ScriptTxIn(ScriptTxIn {
                tx_in: TxInParameter {
                    tx_hash: "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052".to_string(),
                    tx_index: 1,
                    amount: Some(vec![Asset::new_from_str("lovelace", "4633697637")]),
                    address: Some("addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj".to_string()),
                },
                script_tx_in: ScriptTxInParameter {
                    script_source: Some(whisky_common::ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                        script_cbor: "525101010023259800a518a4d136564004ae69".to_string(),
                        language_version: whisky_common::LanguageVersion::V3,
                    })),
                    datum_source: Some(DatumSource::InlineDatumSource(InlineDatumSource {
                        tx_hash: "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052".to_string(),
                        tx_index: 1,
                    })),
                    redeemer: Some(Redeemer {
                        data: "d87980".to_string(),
                        ex_units: Budget { mem: 0, steps: 0 },
                    }),
                },
            }),
        ],
        outputs: vec![
            Output {
                address: "addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj".to_string(),
                amount: vec![Asset::new_from_str("lovelace", "3633697637")],
                datum: Some(Datum::Inline("d87980".to_string())),
                reference_script: Some(whisky_common::OutputScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                    script_cbor: "525101010023259800a518a4d136564004ae69".to_string(),
                    language_version: whisky_common::LanguageVersion::V3,
                })),
            },
            Output {
                address: "addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj".to_string(),
                amount: vec![Asset::new_from_str("lovelace", "3633697637")],
                datum: Some(Datum::Hash("d87980".to_string())),
                reference_script: Some(whisky_common::OutputScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                    script_cbor: "525101010023259800a518a4d136564004ae69".to_string(),
                    language_version: whisky_common::LanguageVersion::V3,
                })),
            },
        ],
        collaterals: vec![],
        reference_inputs: vec![
            RefTxIn {
                tx_hash: "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052".to_string(),
                tx_index: 1,
                script_size: Some(0)
            },
            RefTxIn {
                tx_hash: "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052".to_string(),
                tx_index: 2,
                script_size: Some(1000)
            },
            RefTxIn {
                tx_hash: "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052".to_string(),
                tx_index: 3,
                script_size: Some(1000)
            }
        ],
        withdrawals: vec![],
        mints: vec![
            MintItem::ScriptMint(ScriptMint {
                mint: MintParameter {
                    policy_id: "bd3ae991b5aafccafe5ca70758bd36a9b2f872f57f6d3a1ffa0eb777".to_string(),
                    asset_name: "".to_string(),
                    amount: 1,
                },
                redeemer: Some(Redeemer {
                    data: "d87980".to_string(),
                    ex_units: Budget { mem: 1000000, steps: 1000000 },
                }),
                script_source: Some(ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                    script_cbor: "5101010023259800a518a4d136564004ae69".to_string(),
                    language_version: whisky_common::LanguageVersion::V3,
                })),
            }),
            MintItem::ScriptMint(ScriptMint {
                mint: MintParameter {
                    policy_id: "0f6b02150cbcc7fedafa388abcc41635a9443afb860100099ba40f07".to_string(),
                    asset_name: "".to_string(),
                    amount: -1,
                },
                redeemer: Some(Redeemer {
                    data: "d87980".to_string(),
                    ex_units: Budget { mem: 1000000, steps: 1000000 },
                }),
                script_source: Some(ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                    script_cbor: "5101010023259800a518a4d136564004ae69".to_string(),
                    language_version: whisky_common::LanguageVersion::V3,
                })),
            })
        ],
        certificates: vec![
            Certificate::BasicCertificate(CertificateType::DRepRegistration(DRepRegistration {
                drep_id: "drep1yfvaekm32jzaway9kmfl6mpgjvv05yj26gyzdrywqxuv5cqtzay4v".to_string(),
                coin: 5000000,
                anchor: None,
            })),
        ],
        votes: vec![],
        change_address: "addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj".to_string(),
        fee: None,
        required_signatures: vec![],
        change_datum: None,
        metadata: vec![],
        validity_range: ValidityRange {
            invalid_before: None,
            invalid_hereafter: None,
        },
        signing_key: vec![],
        network: Some(whisky_common::Network::Preprod),
        total_collateral: None,
        collateral_return_address: None,
    };
    let mut core_pallas = CorePallas::new(Protocol::default());
    let result = core_pallas.build_tx(tx_builder_body.clone(), true).unwrap();
    let tx_bytes = hex::decode(result.clone()).unwrap();
    let pallas_tx = Tx::decode_fragment(&tx_bytes).unwrap();
    for (i, input) in pallas_tx.transaction_body.inputs.iter().enumerate() {
        if i == 0 {
            assert!(
                input.transaction_id.to_string()
                    == "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052"
            );

            assert!(input.index == 0);
        } else if i == 1 {
            assert!(
                input.transaction_id.to_string()
                    == "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052"
            );

            assert!(input.index == 1);
        }
    }
    for (i, output) in pallas_tx.transaction_body.outputs.iter().enumerate() {
        match output {
            pallas::ledger::primitives::babbage::GenTransactionOutput::Legacy(_keep_raw) => {}
            pallas::ledger::primitives::babbage::GenTransactionOutput::PostAlonzo(output) => {
                if i == 0 {
                    assert!(output.address.to_string() == bytes_from_bech32(
                        "addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj"
                    ).unwrap());
                    match output.value.clone() {
                        pallas::ledger::primitives::conway::Value::Coin(coin) => {
                            assert!(coin.to_string() == "3633697637");
                        }
                        pallas::ledger::primitives::conway::Value::Multiasset(_, _) => {}
                    };
                } else if i == 1 {
                    assert!(output.address.to_string() == bytes_from_bech32(
                        "addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj"
                    ).unwrap());
                    match output.value.clone() {
                        pallas::ledger::primitives::conway::Value::Coin(coin) => {
                            assert!(coin.to_string() == "3633697637");
                        }
                        pallas::ledger::primitives::conway::Value::Multiasset(_, _) => {}
                    };
                } else if i == 2 {
                    assert!(output.address.to_string() == bytes_from_bech32(
                        "addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj"
                    ).unwrap());
                }
            }
        }
    }
    for (i, mint) in pallas_tx
        .clone()
        .transaction_body
        .mint
        .clone()
        .unwrap()
        .iter()
        .enumerate()
    {
        let (policy_id, assets) = mint;
        if i == 0 {
            assert!(
                policy_id.to_string() == "0f6b02150cbcc7fedafa388abcc41635a9443afb860100099ba40f07"
            );
            let mint_amount = assets.get(&Bytes::from_str("").unwrap()).unwrap();
            assert!(i64::try_from(*mint_amount).unwrap() == -1)
        } else if i == 1 {
            assert!(
                policy_id.to_string() == "bd3ae991b5aafccafe5ca70758bd36a9b2f872f57f6d3a1ffa0eb777"
            );
            let mint_amount = assets.get(&Bytes::from_str("").unwrap()).unwrap();
            assert!(i64::try_from(*mint_amount).unwrap() == 1)
        }
    }
}
