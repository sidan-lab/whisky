mod int_tests {
    use serde_json::{json, to_string};
    use sidan_csl_rs::{
        builder::core::MeshTxBuilderCore,
        model::builder::{Asset, Budget, LanguageVersion, Redeemer},
    };

    #[test]
    fn test_complex_plutus_mint_spend_with_ref_tx() {
        let cns_owner_addr = "addr_test1vr3vljjxan0hl6u28fle2l4ds6ugc9t08lwevpauk38t3agx7rtq6";
        let record_validator_addr =
            "addr_test1wz97vqzhce0m4ek4cpnnlzvlaf5gdzck46axlur094lnzcgj0pq2u";
        let cns_policy_id = "baefdc6c5b191be372a794cd8d40d839ec0dbdd3c28957267dc81700";
        let record_token_policy_id = "19683f7853c85a7eb53615b580f15f89a1280f8fbd642edc4cb753e6";
        let cns_token_mp_script_ref_txhash =
            "63210437b543c8a11afbbc6765aa205eb2733cb74e2805afd4c1c8cb72bd8e22";
        let cns_token_mp_script_ref_txid = "0";
        let record_validator_script_ref_txhash =
            "bb712547a5abe3697f8aba72870e33a52fd2c0401715950197f9b7370d137998";
        let record_validator_script_ref_txid = "0";
        let cns_owner_pubkey = "e2cfca46ecdf7feb8a3a7f957ead86b88c156f3fdd9607bcb44eb8f5";

        let wallet_address = "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x";
        let domain = "6d65736874657374696e67340a";
        let domain_with_ext = "6d65736874657374696e67342e6164610a";
        let metadata = json!({
          "baefdc6c5b191be372a794cd8d40d839ec0dbdd3c28957267dc81700": {
            "meshtesting4.ada": {
              "cnsType": "Normal",
              "description": "CNS, the digital social identity on Cardano.",
              "expiry": "1731369600000",
              "image": "ipfs://QmVEr6bkAek9Fibo7qotxfUWyXup2Bmav3SL9vB7t68Ngd",
              "mediaType": "image/jpeg",
              "name": "meshtesting4.ada",
              "origin": "Cardano Name Service",
              "virtualSubdomainEnabled": "Disabled",
              "virtualSubdomainLimits": 0,
            },
          },
          "version": 1,
        });

        let record_token_name_hex = "434e53205265636f7264202834303029";
        let record_tx_hash = "aae2b8a5bf420c0d2fc785d54fe3eacc107145dee01b8c61beedcd13e6be9a71";
        let record_tx_id = 0;

        let mut mesh = MeshTxBuilderCore::new();

        mesh.tx_in(
            "fc1c806abc9981f4bee2ce259f61578c3341012f3d04f22e82e7e40c7e7e3c3c".to_string(),
            3,
            vec![Asset {
                unit: "lovelace".to_string(),
                quantity: "9692479606".to_string(),
            }],
            "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x".to_string(),
        )
        .read_only_tx_in_reference(
            "8b7ea04a142933b3d8005bf98be906bdba10978891593b383deac933497e2ea7".to_string(),
            1,
        )
        .mint_plutus_script_v2()
        .mint(1, cns_policy_id.to_string(), domain_with_ext.to_string())
        .mint_tx_in_reference(
            cns_token_mp_script_ref_txhash.to_string(),
            cns_token_mp_script_ref_txid.parse::<u32>().unwrap(),
            cns_policy_id.to_string(),
            LanguageVersion::V2,
        )
        .mint_redeemer_value(Redeemer {
            data: to_string(&json!({
                "constructor": 0,
                "fields": [{ "bytes": domain }]
            }))
            .unwrap(),
            ex_units: Budget {
                mem: 3386819,
                steps: 1048170931,
            },
        })
        .spending_plutus_script_v2()
        .tx_in(
            record_tx_hash.to_string(),
            record_tx_id,
            vec![Asset {
                unit: record_token_policy_id.to_string() + record_token_name_hex,
                quantity: "1".to_string(),
            }],
            "addr_test1wz97vqzhce0m4ek4cpnnlzvlaf5gdzck46axlur094lnzcgj0pq2u".to_string(),
        )
        .spending_reference_tx_in_inline_datum_present()
        .spending_reference_tx_in_redeemer_value(Redeemer {
            data: to_string(&json!({
              "constructor": 0,
              "fields": [{ "bytes": domain }],
            }))
            .unwrap(),
            ex_units: Budget {
                mem: 9978951,
                steps: 4541421719,
            },
        })
        .spending_tx_in_reference(
            record_validator_script_ref_txhash.to_string(),
            record_validator_script_ref_txid.parse::<u32>().unwrap(),
            "8be60057c65fbae6d5c0673f899fea68868b16aeba6ff06f2d7f3161".to_string(),
            LanguageVersion::V2,
        )
        .tx_out(
            wallet_address.to_string(),
            vec![
                Asset {
                    unit: "lovelace".to_string(),
                    quantity: "2000000".to_string(),
                },
                Asset {
                    unit: cns_policy_id.to_string() + domain_with_ext,
                    quantity: "1".to_string(),
                },
            ],
        )
        .tx_out(
            cns_owner_addr.to_string(),
            vec![Asset {
                unit: "lovelace".to_string(),
                quantity: "30000000".to_string(),
            }],
        )
        .tx_out(
            record_validator_addr.to_string(),
            vec![
                Asset {
                    unit: "lovelace".to_string(),
                    quantity: "20000000".to_string(),
                },
                Asset {
                    unit: record_token_policy_id.to_string() + record_token_name_hex,
                    quantity: "1".to_string(),
                },
            ],
        )
        .tx_out_inline_datum_value(
            to_string(&json!({
              "constructor": 0,
              "fields": [{ "bytes": domain }],
            }))
            .unwrap(),
        )
        .required_signer_hash(cns_owner_pubkey.to_string())
        .metadata_value("721".to_string(), to_string(&metadata).unwrap())
        .tx_in_collateral(
            "3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814".to_string(),
            6,
            vec![Asset {
                unit: "lovelace".to_string(),
                quantity: "10000000".to_string(),
            }],
            "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x".to_string(),
        )
        .tx_in_collateral(
            "3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814".to_string(),
            7,
            vec![Asset {
                unit: "lovelace".to_string(),
                quantity: "10000000".to_string(),
            }],
            "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x".to_string(),
        )
        .change_address(wallet_address.to_string())
        .complete_sync(None);

        assert!(mesh.tx_hex != *"");
    }
}
