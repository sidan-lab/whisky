use whisky_common::{
    Asset, Budget,
    Certificate::{self, BasicCertificate},
    CertificateType, DRepRegistration, Datum, DatumSource, InlineDatumSource, MintItem,
    MintParameter, Output, ProvidedScriptSource, PubKeyTxIn, Redeemer, RefTxIn, ScriptMint,
    ScriptSource, ScriptTxIn, ScriptTxInParameter, TxBuilderBody, TxIn, TxInParameter,
    ValidityRange,
};
use whisky_pallas::{
    tx_builder::core_pallas::CorePallas, wrapper::transaction_body::DRep,
    wrapper::transaction_body::StakeCredential,
};

#[test]
fn test_from_tx_builder_body() {
    let tx_builder_body = TxBuilderBody {
        inputs: vec![
            TxIn::PubKeyTxIn(PubKeyTxIn {
                tx_in: TxInParameter {
                    tx_hash: "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052".to_string(),
                    tx_index: 0,
                    amount: Some(vec![Asset::new_from_str("lovelace", "4633697637")]),
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
                amount: vec![Asset::new_from_str("lovelace", "4633697637")],
                datum: Some(Datum::Inline("d87980".to_string())),
                reference_script: Some(whisky_common::OutputScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                    script_cbor: "525101010023259800a518a4d136564004ae69".to_string(),
                    language_version: whisky_common::LanguageVersion::V3,
                })),
            },
            Output {
                address: "addr_test1qzjhvr7xdqmyk6x7ax84rtgs3uasqyrvglz4k08kwhw4q4jp2fnzs02hl5fhjdtw07kkxeyfac0gf9aepnpp4vv3yy2s67j7tj".to_string(),
                amount: vec![Asset::new_from_str("lovelace", "4633697637")],
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
                script_size: Some(0)
            },
            RefTxIn {
                tx_hash: "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052".to_string(),
                tx_index: 3,
                script_size: Some(0)
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
                    ex_units: Budget { mem: 0, steps: 0 },
                }),
                script_source: Some(ScriptSource::ProvidedScriptSource(ProvidedScriptSource {
                    script_cbor: "5101010023259800a518a4d136564004ae69".to_string(),
                    language_version: whisky_common::LanguageVersion::V3,
                })),
            }),
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
        fee: Some(10000.to_string()),
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
    println!("Serialized transaction hex: {}", result.unwrap());
}

#[test]
fn test_drep() {
    let drep = DRep::from_bech32("drep1yfaaqfzaukju6pr5wa4nhzqglv57p46cjlws424m6hhj3kg2k9vj7");
    println!("DRep: {:?}", drep);
}

#[test]
fn test_stake_cred() {
    let stake_credential = StakeCredential::from_bech32(
        "stake_test1uqevw2xnsc0pvn9t9r9c7qryfqfeerchgrlm3ea2nefr9hqp8n5xl",
    );
    println!("Stake Credential: {:?}", stake_credential);
}
