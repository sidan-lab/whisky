mod int_tests {
    use pallas_primitives::conway::Tx;
    use serde_json::{json, to_string};
    use uplc::Fragment;
    use whisky::{Credential as TxBuilderCredential, *};
    use whisky_common::data::*;
    use whisky_pallas::{tx_parser::parse, WhiskyPallas};

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
            "tx_buildertesting4.ada": {
              "cnsType": "Normal",
              "description": "CNS, the digital social identity on Cardano.",
              "expiry": "1731369600000",
              "image": "ipfs://QmVEr6bkAek9Fibo7qotxfUWyXup2Bmav3SL9vB7t68Ngd",
              "mediaType": "image/jpeg",
              "name": "tx_buildertesting4.ada",
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

        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let res = tx_builder
            .tx_in(
                "fc1c806abc9981f4bee2ce259f61578c3341012f3d04f22e82e7e40c7e7e3c3c",
                3,
                &[Asset::new_from_str("lovelace", "9692479606")],
                "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
            )
            .read_only_tx_in_reference(
                "8b7ea04a142933b3d8005bf98be906bdba10978891593b383deac933497e2ea7",
                1,
                None,
            )
            .mint_plutus_script_v2()
            .mint(1, cns_policy_id, domain_with_ext)
            .mint_tx_in_reference(
                cns_token_mp_script_ref_txhash,
                cns_token_mp_script_ref_txid.parse::<u32>().unwrap(),
                cns_policy_id,
                100,
            )
            .mint_redeemer_value(&WRedeemer {
                data: WData::JSON(
                    to_string(&json!({
                        "constructor": 0,
                        "fields": [{ "bytes": domain }]
                    }))
                    .unwrap(),
                ),
                ex_units: Budget {
                    mem: 3386819,
                    steps: 1048170931,
                },
            })
            .spending_plutus_script_v2()
            .tx_in(
                record_tx_hash,
                record_tx_id,
                &[Asset::new(
                    record_token_policy_id.to_string() + record_token_name_hex,
                    "1".to_string(),
                )],
                "addr_test1wz97vqzhce0m4ek4cpnnlzvlaf5gdzck46axlur094lnzcgj0pq2u",
            )
            .spending_reference_tx_in_inline_datum_present()
            .spending_reference_tx_in_redeemer_value(&WRedeemer {
                data: WData::JSON(
                    to_string(&json!({
                      "constructor": 0,
                      "fields": [{ "bytes": domain }],
                    }))
                    .unwrap(),
                ),
                ex_units: Budget {
                    mem: 9978951,
                    steps: 4541421719,
                },
            })
            .spending_tx_in_reference(
                record_validator_script_ref_txhash,
                record_validator_script_ref_txid.parse::<u32>().unwrap(),
                "8be60057c65fbae6d5c0673f899fea68868b16aeba6ff06f2d7f3161",
                100,
            )
            .tx_out(
                wallet_address,
                &[
                    Asset::new_from_str("lovelace", "2000000"),
                    Asset::new(cns_policy_id.to_string() + domain_with_ext, "1".to_string()),
                ],
            )
            .tx_out(
                cns_owner_addr,
                &[Asset::new_from_str("lovelace", "30000000")],
            )
            .tx_out(
                record_validator_addr,
                &[
                    Asset::new_from_str("lovelace", "20000000"),
                    Asset::new(
                        record_token_policy_id.to_string() + record_token_name_hex,
                        "1".to_string(),
                    ),
                ],
            )
            .tx_out_inline_datum_value(&WData::JSON(
                to_string(&json!({
                  "constructor": 0,
                  "fields": [{ "bytes": domain }],
                }))
                .unwrap(),
            ))
            .required_signer_hash(cns_owner_pubkey)
            .metadata_value("721", to_string(&metadata).unwrap().as_str())
            .tx_in_collateral(
                "3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814",
                6,
                &[Asset::new_from_str("lovelace", "10000000")],
                "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
            )
            .tx_in_collateral(
                "3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814",
                7,
                &[Asset::new_from_str("lovelace", "10000000")],
                "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
            )
            .change_address(wallet_address)
            .change_output_datum(WData::JSON(constr0(json!([])).to_string()))
            .complete_sync(None);

        match res {
            Ok(_) => {
                let signed_tx = tx_builder.complete_signing().unwrap();
                println!("{}", signed_tx);
                assert!(tx_builder.serializer.tx_hex() != *"");
            }
            Err(e) => {
                println!("error: {:?}", e);
                // failing the test case
                panic!()
            }
        }
        println!("{}", tx_builder.serializer.tx_hex());
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_simple_spend() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let signed_tx = tx_builder
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .change_address("addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh")
            .signing_key("51022b7e38be01d1cc581230e18030e6e1a3e949a1fdd2aeae5f5412154fe82b")
            .complete_sync(None)
            .unwrap()
            .complete_signing()
            .unwrap();

        println!("{}", signed_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_simple_withdraw() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let signed_tx = tx_builder
            .tx_in(
                "fbd3e8091c9f0c5fb446be9e58d9235f548546a5a7d5f60ee56e389344db9c5e",
                0,
                &[Asset::new_from_str("lovelace", "9496607660")],
                "addr_test1qpjfsrjdr8kk5ffj4jnw02ht3y3td0y0zkcm52rc6w7z7flmy7vplnvz6a7dncss4q5quqwt48tv9dewuvdxqssur9jqc4x459",
            )
            .change_address("addr_test1qpjfsrjdr8kk5ffj4jnw02ht3y3td0y0zkcm52rc6w7z7flmy7vplnvz6a7dncss4q5quqwt48tv9dewuvdxqssur9jqc4x459")
            .withdrawal("stake_test1uraj0xqlekpdwlxeugg2s2qwq896n4kzkuhwxxnqggwpjeqe9s9k2", 0)
            .required_signer_hash("fb27981fcd82d77cd9e210a8280e01cba9d6c2b72ee31a60421c1964")
            .required_signer_hash("64980e4d19ed6a2532aca6e7aaeb8922b6bc8f15b1ba2878d3bc2f27")
            .signing_key("58208d4cfa90e8bd0c48c52d2fb62c77ba3f6f5eb46f640d5f997390012928d670f7")
            .signing_key("5820ba73019f1239fa47f8d9c0c42c5d05bf34f2b2f6ebd1c556f8f86e5bee1aac66")
            .complete_sync(None)
            .unwrap()
            .complete_signing().unwrap();

        println!("{}", signed_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_plutus_withdraw() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let signed_tx = tx_builder
            .tx_in(
                "60b6a29a4c164bece283738abd57fa35c0b839f298f15836ee54a875ede87d37",
                0,
                &[Asset::new_from_str("lovelace", "9999639476")],
                "addr_test1yp8ezxpltlrus89uz8g7e07795w0cxn3a7w7nxdac8s4aj7cjpk2t3a6zf9qgpar9k4n0vkg9vfm8hxezy0y99qde6jq58zjfw",
            )
            .tx_in_collateral(
            "60b6a29a4c164bece283738abd57fa35c0b839f298f15836ee54a875ede87d37",
            0,
            &[Asset::new_from_str("lovelace", "9999639476")],
            "addr_test1yp8ezxpltlrus89uz8g7e07795w0cxn3a7w7nxdac8s4aj7cjpk2t3a6zf9qgpar9k4n0vkg9vfm8hxezy0y99qde6jq58zjfw",
            )
            .change_address("addr_test1yp8ezxpltlrus89uz8g7e07795w0cxn3a7w7nxdac8s4aj7cjpk2t3a6zf9qgpar9k4n0vkg9vfm8hxezy0y99qde6jq58zjfw")
            .withdrawal_plutus_script_v2()
            .withdrawal("stake_test17rvfqm99c7apyjsyq73jm2ehktyzkyanmnv3z8jzjsxuafq5a6z2j", 0)
            .withdrawal_script("5251010000322253330034a229309b2b2b9a01")
            .withdrawal_redeemer_value(&WRedeemer {
                data: WData::JSON(constr0(json!([])).to_string()),
                ex_units: Budget {
                    mem: 2501,
                    steps: 617656,
                },
            })
            .required_signer_hash("4f91183f5fc7c81cbc11d1ecbfde2d1cfc1a71ef9de999bdc1e15ecb")
            .signing_key("5820c835cd2413c6330537c85e3d510b313dfdeee5708206e76ce8bd387cdd4b6bb2")
            .complete_sync(None)
            .unwrap()
            .complete_signing().unwrap();

        println!("{}", signed_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_native_script_ref() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let _unsigned_tx = tx_builder
            .tx_in(
                "db0937db0e8a743e6e97e8cf29077af1e951b52e46f2e2c63ef12a3abaaf9052",
                80,
                &[Asset::new_from_str("lovelace", "4633697637")],
                "addr_test1qr3a9rrclgf9rx90lmll2qnfzfwgrw35ukvgjrk36pmlzu0jemqwylc286744g0tnqkrvu0dkl8r48k0upkfmg7mncpqf0672w",
            )
            .change_address("addr_test1qr3a9rrclgf9rx90lmll2qnfzfwgrw35ukvgjrk36pmlzu0jemqwylc286744g0tnqkrvu0dkl8r48k0upkfmg7mncpqf0672w")
            .tx_out("addr_test1qr3a9rrclgf9rx90lmll2qnfzfwgrw35ukvgjrk36pmlzu0jemqwylc286744g0tnqkrvu0dkl8r48k0upkfmg7mncpqf0672w", &[Asset::new_from_str("lovelace", "5000000")])
            .tx_out_reference_script("8200581ce3d28c78fa125198affefff50269125c81ba34e598890ed1d077f171", None)
            .complete_sync(None)
            .unwrap()
            .complete_signing().unwrap();

        // let signed_tx = merge_vkey_witnesses_to_transaction(unsigned_tx, "a10081825820096348a7a3640d8ecc89819abffc7ed89cde399346046d50444acbd6e467f9df5840111279e89d341c9ab51f9ee7d5bb3a8db068ca6d09b7d3d4aaa48940dc55162903fd8f194df5c048055c9ac869e95729273b4ebb752be8a998f3483fac5d6e05".to_string());
        // println!("{}", signed_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_plutus_script_cert_registration() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = tx_builder
                .tx_in("b3b05ac96e1eb4cd3b3cb8150cc48ee006d12683ed1b87ee57122d83235069df",
            0,
        &[Asset::new_from_str("lovelace", "1488554147")],
        "addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6",)
        .tx_in_collateral("541e2c5e6af1661a08aedf53fc4fb66aee00885629100196abbe42b05121adff", 5, &[Asset::new_from_str("lovelace", "5000000")], "addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
        .change_address("addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
        .register_stake_certificate("stake_test17rvfqm99c7apyjsyq73jm2ehktyzkyanmnv3z8jzjsxuafq5a6z2j")
        .complete_sync(None)
        .unwrap()
        .complete_signing().unwrap();

        println!("{}", unsigned_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_plutus_script_cert_deregistration() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = tx_builder
                .tx_in("b3b05ac96e1eb4cd3b3cb8150cc48ee006d12683ed1b87ee57122d83235069df",
            0,
        &[Asset::new_from_str("lovelace", "1488554147")],
        "addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6",)
        .tx_in_collateral("541e2c5e6af1661a08aedf53fc4fb66aee00885629100196abbe42b05121adff", 5, &[Asset::new_from_str("lovelace", "5000000")], "addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
        .change_address("addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
        .deregister_stake_certificate("stake_test17rvfqm99c7apyjsyq73jm2ehktyzkyanmnv3z8jzjsxuafq5a6z2j")
        .certificate_script("5251010000322253330034a229309b2b2b9a01", Some(LanguageVersion::V2))
        .certificate_redeemer_value(&WRedeemer {
            data: WData::JSON(constr0(json!([])).to_string()),
            ex_units: Budget {
                mem: 7000000,
                steps: 14000000
            }})
        .complete_sync(None)
        .unwrap()
        .complete_signing().unwrap();

        println!("{}", unsigned_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_mint_two_tokens_with_same_policy() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = tx_builder.
        tx_in("b68d2e8340d9454c66b0530de8fdeca5bc829c577217b12f0c0beeb7f42b6b18", 0, &[Asset::new_from_str("lovelace", "100000000000")], "addr_test1qrfkkp5dwgj07fljdum677pglfm5707hd8nwj5wgfqdhfp0m7kq4cxp4nznl6v9yp2wxvwl2vsh0mk7eq7g97vczj6uqse4e3j")
        .tx_in_collateral("541e2c5e6af1661a08aedf53fc4fb66aee00885629100196abbe42b05121adff", 5, &[Asset::new_from_str("lovelace", "5000000")], "addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
        .mint_plutus_script_v2()
        .mint(1, "d8906ca5c7ba124a0407a32dab37b2c82b13b3dcd9111e42940dcea4",  "7465737431")
        .mint_redeemer_value(&WRedeemer {
            data: WData::JSON(constr0(json!([])).to_string()),
            ex_units: Budget {
                mem: 7000000,
                steps: 14000000
            }})
        .minting_script("5251010000322253330034a229309b2b2b9a01")
        .mint_plutus_script_v2()
        .mint(1, "d8906ca5c7ba124a0407a32dab37b2c82b13b3dcd9111e42940dcea4", "7465737432")
        .mint_redeemer_value(&WRedeemer {
            data: WData::JSON(constr0(json!([])).to_string()),
            ex_units: Budget {
                mem: 7000000,
                steps: 14000000
            }})
        .minting_script("5251010000322253330034a229309b2b2b9a01")
        .change_address("addr_test1qrfkkp5dwgj07fljdum677pglfm5707hd8nwj5wgfqdhfp0m7kq4cxp4nznl6v9yp2wxvwl2vsh0mk7eq7g97vczj6uqse4e3j")
        .complete_sync(None)
        .unwrap()
        .complete_signing()
        .unwrap();

        println!("{}", unsigned_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_spend_withdraw_and_unreg() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let reward_address = "stake_test17q3hjj9svuvmmj5untsrclvlwzs8q528tzj0k3g5hgkzajc23t4fh";

        let unsigned_tx = tx_builder
        .spending_plutus_script_v2()
        .tx_in("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 0, &[Asset::new_from_str("lovelace", "6904620")], "addr_test1zrrpfzell3549ulhjwar3juz8dv8qcc99kfvlwrfzu2sw76u5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9shn8fam")
        .spending_tx_in_reference("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 1, "237948b06719bdca9c9ae03c7d9f70a070514758a4fb4514ba2c2ecb", 950)
        .tx_in_inline_datum_present()
        .spending_reference_tx_in_redeemer_value(&WRedeemer {
            data: WData::JSON(constr0(json!([])).to_string()),
            ex_units: Budget {
                mem: 35588,
                steps: 13042895
            }})
        .spending_plutus_script_v2()
        .tx_in("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 1, &[Asset::new_from_str("lovelace", "5159070")], "addr_test1zrrpfzell3549ulhjwar3juz8dv8qcc99kfvlwrfzu2sw76u5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9shn8fam")
        .spending_tx_in_reference("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 1, "237948b06719bdca9c9ae03c7d9f70a070514758a4fb4514ba2c2ecb", 950)
        .tx_in_inline_datum_present()
        .spending_reference_tx_in_redeemer_value(&WRedeemer {
            data: WData::JSON(constr0(json!([])).to_string()),
            ex_units: Budget {
                mem: 35588,
                steps: 13042895
            }})
        .deregister_stake_certificate(reward_address)
        .certificate_tx_in_reference("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 0, "237948b06719bdca9c9ae03c7d9f70a070514758a4fb4514ba2c2ecb", Some(LanguageVersion::V2), 953)
        .certificate_redeemer_value(&WRedeemer {
            data: WData::JSON(constr0(json!([])).to_string()),
            ex_units: Budget {
                mem: 120022,
                steps: 44400485
            }})
        .withdrawal_plutus_script_v2()
        .withdrawal(reward_address, 0)
        .withdrawal_redeemer_value(&WRedeemer {
            data: WData::JSON(constr0(json!([])).to_string()),
            ex_units: Budget {
                mem: 120022,
                steps: 44400485
            }})
        .withdrawal_tx_in_reference("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 0, "237948b06719bdca9c9ae03c7d9f70a070514758a4fb4514ba2c2ecb", 953)
        .read_only_tx_in_reference("d3e7e43ec9c85cfdb90f98fb40bb4edd58fdd3d056e32f827739fe0b915c6eb7", 0, None)
        .change_address("addr_test1qqjcvv7huxlf9epjq49j4952pez8l4zyrm6c4wrf2vtcym4jg6fd5d54p0k5mqy46ph5z3r59tkhnhjvsxx53dq5rvdsnaeh3a")
        .tx_in_collateral("3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814", 7, &[Asset::new_from_str("lovelace", "10000000")], "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x")
        .required_signer_hash("258633d7e1be92e432054b2a968a0e447fd4441ef58ab8695317826e")
        .required_signer_hash("5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa")
        .complete_sync(None)
        .unwrap()
        .complete_signing()
        .unwrap();

        println!("{}", unsigned_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_embedded_datum_output() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let signed_tx = tx_builder
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .tx_out(
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
                &[Asset::new_from_str("lovelace", "2000000")],
            )
            .tx_out_datum_embed_value(&WData::JSON(
                json!({
                  "constructor": 0,
                  "fields": []
                })
                .to_string(),
            ))
            .change_address("addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh")
            .signing_key("51022b7e38be01d1cc581230e18030e6e1a3e949a1fdd2aeae5f5412154fe82b")
            .complete_sync(None)
            .unwrap()
            .complete_signing()
            .unwrap();

        println!("{}", signed_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_register_drep() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = tx_builder
            .change_address("addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .drep_registration("drep1j6257gz2swty9ut46lspyvujkt02pd82am2zq97p7p9pv2euzs7", 500000000, Some(Anchor {
                anchor_url: "https://raw.githubusercontent.com/HinsonSIDAN/cardano-drep/main/HinsonSIDAN.jsonld".to_string(),
                anchor_data_hash: "2aef51273a566e529a2d5958d981d7f0b3c7224fc2853b6c4922e019657b5060".to_string()
            }))
            .complete_sync(None)
            .unwrap()
            .complete_signing()
            .unwrap();

        println!("{}", unsigned_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_register_drep_cip129() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = tx_builder
            .change_address("addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .drep_registration("drep1y2tf2neqf2pevsh3wht7qy3nj2edag95athdggqhc8cy59s6skxy4", 500000000, Some(Anchor {
                anchor_url: "https://raw.githubusercontent.com/HinsonSIDAN/cardano-drep/main/HinsonSIDAN.jsonld".to_string(),
                anchor_data_hash: "2aef51273a566e529a2d5958d981d7f0b3c7224fc2853b6c4922e019657b5060".to_string()
            }))
            .complete_sync(None)
            .unwrap()
            .complete_signing()
            .unwrap();

        println!("{}", unsigned_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_vote_delegation() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = tx_builder
            .change_address("addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .vote_delegation_certificate("stake_test1uzdx8vwxvz5wy45fwdrwk2l85ax7j5wtr4cee6a8xc632cc3p6psh", &DRep::DRepId("drep1j6257gz2swty9ut46lspyvujkt02pd82am2zq97p7p9pv2euzs7".to_string()))
            .complete_sync(None)
            .unwrap()
            .complete_signing()
            .unwrap();

        println!("{}", unsigned_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_drep_vote() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = tx_builder
            .change_address("addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .vote(&Voter::DRepId("drep1j6257gz2swty9ut46lspyvujkt02pd82am2zq97p7p9pv2euzs7".to_string()), &RefTxIn {
                tx_hash: "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85".to_string(),
                tx_index: 2,
                script_size: None,
            }, &VotingProcedure {
                vote_kind: VoteKind::Abstain,
                anchor: Some(Anchor {
                    anchor_url: "https://raw.githubusercontent.com/HinsonSIDAN/cardano-drep/main/HinsonSIDAN.jsonld".to_string(),
                    anchor_data_hash: "2aef51273a566e529a2d5958d981d7f0b3c7224fc2853b6c4922e019657b5060".to_string()
                })
            })
            .complete_sync(None)
            .unwrap()
            .complete_signing()
            .unwrap();

        println!("{}", unsigned_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_cc_vote() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = tx_builder
            .change_address("addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .vote(&Voter::ConstitutionalCommitteeHotCred(TxBuilderCredential::KeyHash("e3a4c41d67592a1b8d87c62e5c5d73f7e8db836171945412d13f40f8".to_string())), &RefTxIn {
                tx_hash: "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85".to_string(),
                tx_index: 2,
                script_size: None
            }, &VotingProcedure {
                vote_kind: VoteKind::Abstain,
                anchor: Some(Anchor {
                    anchor_url: "https://raw.githubusercontent.com/HinsonSIDAN/cardano-drep/main/HinsonSIDAN.jsonld".to_string(),
                    anchor_data_hash: "2aef51273a566e529a2d5958d981d7f0b3c7224fc2853b6c4922e019657b5060".to_string()
                })
            })
            .complete_sync(None)
            .unwrap()
            .complete_signing()
            .unwrap();

        println!("{}", unsigned_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_simple_spend_with_set_fee() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let signed_tx = tx_builder
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .change_address("addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh")
            .signing_key("51022b7e38be01d1cc581230e18030e6e1a3e949a1fdd2aeae5f5412154fe82b")
            .set_fee("500000")
            .complete_sync(None)
            .unwrap()
            .complete_signing()
            .unwrap();

        println!("{}", signed_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_register_stake_with_custom_pp() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: Some(Protocol {
                epoch: 0,
                min_fee_a: 44,
                min_fee_b: 155381,
                max_block_size: 98304,
                max_tx_size: 16384,
                max_block_header_size: 1100,
                key_deposit: 0,
                pool_deposit: 500000000,
                min_pool_cost: "340000000".to_string(),
                price_mem: 0.0577,
                price_step: 0.0000721,
                max_tx_ex_mem: "16000000".to_string(),
                max_tx_ex_steps: "10000000000".to_string(),
                max_block_ex_mem: "80000000".to_string(),
                max_block_ex_steps: "40000000000".to_string(),
                max_val_size: 5000,
                collateral_percent: 150.0,
                max_collateral_inputs: 3,
                coins_per_utxo_size: 4310,
                min_fee_ref_script_cost_per_byte: 15,
                decentralisation: 0.0,
            }),
        });
        let signed_tx = tx_builder
            .change_address("addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh")
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .register_stake_certificate(
                "stake_test17rvfqm99c7apyjsyq73jm2ehktyzkyanmnv3z8jzjsxuafq5a6z2j",
            )
            .complete_sync(None)
            .unwrap()
            .complete_signing()
            .unwrap();

        println!("{}", signed_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_balance() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let signed_tx = tx_builder
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .tx_out(
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
                &[],
            )
            .change_address("addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh")
            .complete_sync(None)
            .unwrap()
            .complete_signing()
            .unwrap();

        println!("{}", signed_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_min_output_with_datum() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let signed_tx = tx_builder
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .tx_out(
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
                &[],
            )
            .tx_out_inline_datum_value(&WData::JSON(
                json!({
                    "constructor": 0,
                    "fields": []
                })
                .to_string(),
            ))
            .change_address("addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh")
            .signing_key("51022b7e38be01d1cc581230e18030e6e1a3e949a1fdd2aeae5f5412154fe82b")
            .complete_sync(None)
            .unwrap()
            .complete_signing()
            .unwrap();

        println!("{}", signed_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[tokio::test]
    async fn test_csl_tx_parser_round_trip() {
        let utxo_1: UTxO = serde_json::from_str("{\"input\":{\"outputIndex\":0,\"txHash\":\"1a6157c0c9e170d716aee64b25384cad275770e2ef86df31eeebda4892980723\"},\"output\":{\"address\":\"addr_test1qrs3jlcsapdufgagzt35ug3nncwl26mlkcux49gs673sflmrjfm6y2eu7del3pprckzt4jaal9s7w9gq5kguqs5pf6fq542mmq\",\"amount\":[{\"quantity\":\"10000000000\",\"unit\":\"lovelace\"}],\"dataHash\":null,\"plutusData\":null,\"scriptHash\":null,\"scriptRef\":null}}").unwrap();
        let utxo_2: UTxO = serde_json::from_str("{\"input\":{\"outputIndex\":5,\"txHash\":\"158a0bff150e9c6f68a14fdb1623c363f54e36cb22efc800911bffafa4e53442\"},\"output\":{\"address\":\"addr_test1qra9zdhfa8kteyr3mfe7adkf5nlh8jl5xcg9e7pcp5w9yhyf5tek6vpnha97yd5yw9pezm3wyd77fyrfs3ynftyg7njs5cfz2x\",\"amount\":[{\"quantity\":\"5000000\",\"unit\":\"lovelace\"}],\"dataHash\":null,\"plutusData\":null,\"scriptHash\":null,\"scriptRef\":null}}").unwrap();

        let utxos = vec![utxo_1, utxo_2];
        let tx_hex = "84a700d90102818258201a6157c0c9e170d716aee64b25384cad275770e2ef86df31eeebda4892980723000183a300581d70506245b8d10428549499ecfcd0435d5a0b9a3aac2c5bccc824441a7201821a001e8480a1581ceab3a1d125a3bf4cd941a6a0b5d7752af96fae7f5bcc641e8a0b6762a14001028201d818586ad8799fd8799fd8799f5041bfc7325343428683bbd0b94a4da41cd8799f581ce1197f10e85bc4a3a812e34e22339e1df56b7fb6386a9510d7a304ffffd8799f581c7c87b6b5a0963af3eadb107da2ac4e1d34747a4df363858b649aa845ffffffa140a1401a00989680ff82581d70ba3efbd72650cbc7d5d7e6bede007cd3cb6730ba1972debf1c2c098f1a007a120082583900e1197f10e85bc4a3a812e34e22339e1df56b7fb6386a9510d7a304ff639277a22b3cf373f88423c584bacbbdf961e71500a591c042814e921b0000000253704b3f021a0003024109a1581ceab3a1d125a3bf4cd941a6a0b5d7752af96fae7f5bcc641e8a0b6762a140010b5820d88d41dd788fcf7c3b1f15808e11b01d71e0413d57265ddb7fc5b5776ff16e720dd9010281825820158a0bff150e9c6f68a14fdb1623c363f54e36cb22efc800911bffafa4e53442050ed9010281581cfa5136e9e9ecbc9071da73eeb6c9a4ff73cbf436105cf8380d1c525ca207d901028158b558b30101009800aba2a6011e581cfa5136e9e9ecbc9071da73eeb6c9a4ff73cbf436105cf8380d1c525c00a6010746332d6d696e740048c8c8c8c88c88966002646464646464660020026eb0c038c03cc03cc03cc03cc03cc03cc03cc03cc030dd5180718061baa0072259800800c52844c96600266e3cdd71808001005c528c4cc00c00c00500d1808000a01c300c300d002300b001300b002300900130063754003149a26cac8028dd7000ab9a5573caae7d5d0905a182010082d87980821956861a0066ad1cf5f6";
        let mut tx_parser = TxParser::new(None);
        let result = tx_parser.parse(tx_hex, &utxos).await;

        assert!(result.is_ok());
        let body = tx_parser.get_builder_body_without_change();

        let mut new_tx_builder = TxBuilder::new_core();
        new_tx_builder.tx_builder_body = body.clone();

        new_tx_builder.complete_sync(None).unwrap();

        let tx_hex_round_trip = new_tx_builder.tx_hex();
        let decoded_by_csl_tx = csl::Transaction::from_hex(&tx_hex).unwrap();
        let decoded_by_csl_tx_after_round_trip =
            csl::Transaction::from_hex(&tx_hex_round_trip).unwrap();
        assert_eq!(decoded_by_csl_tx, decoded_by_csl_tx_after_round_trip);
    }

    #[test]
    fn test_set_total_collateral() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = tx_builder
        .tx_in(
                "fc1c806abc9981f4bee2ce259f61578c3341012f3d04f22e82e7e40c7e7e3c3c",
                3,
                &[Asset::new_from_str("lovelace", "9692479606")],
                "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
        )
        .change_address("addr_test1qqjcvv7huxlf9epjq49j4952pez8l4zyrm6c4wrf2vtcym4jg6fd5d54p0k5mqy46ph5z3r59tkhnhjvsxx53dq5rvdsnaeh3a")
        .tx_in_collateral(
                "3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814",
                6,
                &[Asset::new_from_str("lovelace", "10000000")],
                "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
            )
        .tx_in_collateral(
                "3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814",
                7,
                &[Asset::new_from_str("lovelace", "10000000")],
                "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
        )
        .set_total_collateral("5000000")
        .complete_sync(None)
        .unwrap()
        .complete_signing()
        .unwrap();

        println!("{}", unsigned_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn test_set_total_collateral_and_collateral_return_address() {
        let mut tx_builder = TxBuilder::new(TxBuilderParam {
            serializer: Box::new(WhiskyPallas::new(None)),
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = tx_builder
        .tx_in(
                "fc1c806abc9981f4bee2ce259f61578c3341012f3d04f22e82e7e40c7e7e3c3c",
                3,
                &[Asset::new_from_str("lovelace", "9692479606")],
                "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
        )
        .change_address("addr_test1qqjcvv7huxlf9epjq49j4952pez8l4zyrm6c4wrf2vtcym4jg6fd5d54p0k5mqy46ph5z3r59tkhnhjvsxx53dq5rvdsnaeh3a")
        .tx_in_collateral(
                "3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814",
                6,
                &[Asset::new_from_str("lovelace", "10000000")],
                "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
            )
        .tx_in_collateral(
                "3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814",
                7,
                &[Asset::new_from_str("lovelace", "10000000")],
                "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
    )
        .set_total_collateral("5000000")
        .set_collateral_return_address("addr_test1qqjcvv7huxlf9epjq49j4952pez8l4zyrm6c4wrf2vtcym4jg6fd5d54p0k5mqy46ph5z3r59tkhnhjvsxx53dq5rvdsnaeh3a")
        .complete_sync(None)
        .unwrap()
        .complete_signing()
        .unwrap();

        println!("{}", unsigned_tx);
        assert!(tx_builder.serializer.tx_hex() != *"");
    }

    #[test]
    fn pallas_parser_test() {
        // Build UTxO contexts
        let utxo_1: UTxO = serde_json::from_str("{\"input\":{\"outputIndex\":8,\"txHash\":\"2c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a\"},\"output\":{\"address\":\"addr_test1zptl0h0ceq3d4tgrlkqgyv2n5cwez0juj9rm63uw8nxhpv6u5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9sczalnn\",\"amount\":[{\"quantity\":\"5000000\",\"unit\":\"lovelace\"}],\"dataHash\":\"923918e403bf43c34b4ef6b48eb2ee04babed17320d8d1b9ff9ad086e86f44ec\",\"plutusData\":\"d87980\",\"scriptHash\":null,\"scriptRef\":null}}").unwrap();
        let utxo_2: UTxO = serde_json::from_str("{\"input\":{\"outputIndex\":3,\"txHash\":\"ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c64\"},\"output\":{\"address\":\"addr_test1zptl0h0ceq3d4tgrlkqgyv2n5cwez0juj9rm63uw8nxhpv6u5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9sczalnn\",\"amount\":[{\"quantity\":\"15000000\",\"unit\":\"lovelace\"},{\"quantity\":\"50000000\",\"unit\":\"5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04b55534458\"}],\"dataHash\":\"923918e403bf43c34b4ef6b48eb2ee04babed17320d8d1b9ff9ad086e86f44ec\",\"plutusData\":\"d87980\",\"scriptHash\":null,\"scriptRef\":null}}").unwrap();
        let utxo_3: UTxO = serde_json::from_str("{\"input\":{\"outputIndex\":4,\"txHash\":\"40e1afc8b735a9daf665926554b0e11902e3ed7e4a31a23b917483d4de42c05e\"},\"output\":{\"address\":\"addr_test1zptl0h0ceq3d4tgrlkqgyv2n5cwez0juj9rm63uw8nxhpv6u5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9sczalnn\",\"amount\":[{\"quantity\":\"1500000\",\"unit\":\"lovelace\"},{\"quantity\":\"50000000\",\"unit\":\"5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04b55534458\"}],\"dataHash\":\"923918e403bf43c34b4ef6b48eb2ee04babed17320d8d1b9ff9ad086e86f44ec\",\"plutusData\":\"d87980\",\"scriptHash\":null,\"scriptRef\":null}}").unwrap();
        let utxo_4: UTxO = serde_json::from_str("{\"input\":{\"outputIndex\":2,\"txHash\":\"ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c64\"},\"output\":{\"address\":\"addr_test1zptl0h0ceq3d4tgrlkqgyv2n5cwez0juj9rm63uw8nxhpv6u5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9sczalnn\",\"amount\":[{\"quantity\":\"1500000\",\"unit\":\"lovelace\"},{\"quantity\":\"250000000\",\"unit\":\"5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04b55534458\"}],\"dataHash\":\"923918e403bf43c34b4ef6b48eb2ee04babed17320d8d1b9ff9ad086e86f44ec\",\"plutusData\":\"d87980\",\"scriptHash\":null,\"scriptRef\":null}}").unwrap();
        let utxo_5: UTxO = serde_json::from_str("{\"input\":{\"outputIndex\":0,\"txHash\":\"efe6fbbdd6b993d96883b96c572bfcaa0a4a138c83bd948dec1751d1bfda09b3\"},\"output\":{\"address\":\"addr_test1zqjmsmh2sjjy508e3068pck6lgp23k2msypgc52cxcgzjlju5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9s5cdt49\",\"amount\":[{\"quantity\":\"1909330\",\"unit\":\"lovelace\"},{\"quantity\":\"1\",\"unit\":\"e6e5285a878161c101a59b4e36f1f99e5e464d30f510be3ee34f907f\"}],\"dataHash\":\"f15c78b61720654e3ed4d373e120a74cfd8816897f796f03b944e8dbb3c58523\",\"plutusData\":\"d8799f581ce6e5285a878161c101a59b4e36f1f99e5e464d30f510be3ee34f907fd8799fd87a9f581c25b86eea84a44a3cf98bf470e2dafa02a8d95b81028c51583610297effd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffff581c5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa581cbbb1a36cc3e076d689176e77374ca26d4e09047c9d9dbd10ab0dcdaeff\",\"scriptHash\":null,\"scriptRef\":null}}").unwrap();
        let utxo_6: UTxO = serde_json::from_str("{\"input\":{\"outputIndex\":0,\"txHash\":\"ac7744adce4f25027f1ca009f5cab1d0858753e62c6081a3a3676cfd5333bb03\"},\"output\":{\"address\":\"addr_test1zptl0h0ceq3d4tgrlkqgyv2n5cwez0juj9rm63uw8nxhpv6u5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9sczalnn\",\"amount\":[{\"quantity\":\"12684330\",\"unit\":\"lovelace\"}],\"dataHash\":null,\"plutusData\":null,\"scriptHash\":\"57f7ddf8c822daad03fd80823153a61d913e5c9147bd478e3ccd70b3\",\"scriptRef\":\"d818590a948202590a8f590a8c010000333323232323232323232232232232222323232533300f323232323232323232323232323232323232323253330233370e90000008a99981199b8748008c0880084c94ccc090cdc3a4000604600226464646464646464a66605e60640042646644646600200200444a66606800229444c8c94ccc0cccc0880180084cc010010004528181c0011bae30360013758602a605601c660626ea409ccc0c4dd480225eb80c94ccc0b4cdc3a4000002264646464a666068606e004264649319299981999b87480000044c8c94ccc0e0c0ec0084c9263253330363370e900000089919299981d981f00109924c60520022c607800260680042a66606c66e1d20020011323232323232533303f3042002149858dd6982000098200011bad303e001303e002375a607800260680042c60680022c607200260620062a66606666e1d200200115333036303100314985858c0c4008c08800c58c0d4004c0d4008c0cc004c0ac01858c0ac01458dd7181800098180011bae302e001302e002302c001302c002375c605400260440022c6464a66604a66e1d20003024001132323253330283370e900218138008981698130008b180b981298081812800981580098118008b19809000919b873330103756601c6046601c60460020429110048008dd6180618108020b0a99981199b87480080044c8c94ccc094cdc3a40046048008264a66604c66e1d20003025001132323253330293370e90021814000899191919299981699b87480080044c8c8c8c8c8c8c8c94ccc0d54ccc0d54ccc0d4010400c52808010a50100114a066660326eacc060c0c80540b0c05cc078c0c8c074c0c80352001323232323232323232533303c3370e9001181d80109919299981f002899b88010001003375a60840026074004002264a66607866e1d2002303b00213232533303e0050031337120200026eb4c108004c0e8008004528181d80119b8748008c0e8dd5181d80099bb00033330380014c103d87a80004c0103d87980003370e9001181c1baa303c001303c002303a0013032301e3032001301930310143302137586034606002605866e3c0040acdd7181a800981a8011bad3033001302b00314a06056004603e002605e002604e0022c6030604c6022604c002605800260480022c660186eb0c03cc08c01800458c0a4004c0840644c8c94ccc094cdc3a400460480082646464a66605066e1d200030270011323232323253330303033002132323232325333032533303253330325333032005100414a020062940400852808008a50323232323232323232533303a3370e9001181c80109919299981e002899b88001018003375a60800026070004002264a66607466e1d2002303900213232533303c0050031337120020306eb4c100004c0e0008004528181c80119b8748008c0e0dd5181c80099bb00033330360014c103d87a80004c0103d87980003370e9001181b1baa303a001303a00230380013030301b30300013017302f012323253330323370e900200089919299981a19b8748008c0cc0044c8c8c8c94ccc0ecc0f800854ccc0e0cdc7800819099b8700301414a02c6eb8c0f0004c0f0008dd6981d00098190008b181c00098180010a5030300013020302e004323253330313370e900200089919299981999b8748010c0c80044c8c94ccc0e0c0ec0084cdc78008178b1bae30390013031001163037001302f00214a0605e002603e605a6030605a00c66660266eacc048c0b003c098c04401120023301c3758602a605601c04e2c60620026644646600200200644a666064002297ae013232533303153330313375e6036605e00400e266e1cccc070dd5980d1817801014802a400429404cc0d4008cc0100100044cc010010004c0d8008c0d0004dd6180d98148061807000980a181418099814000981700098130008b198071bac30113025008001302b001302300416375a605200260420326042030604c002604c00460480026038026464a66603e66e1d2002301e00113232323253330233370e90010008980780189919299981299b8748000c0900044c8c8c94ccc0a0cdc3a4000002260286602a0106eb8c0b4c0980084c050cc054020dd7181698130011813000981580098118008b1814800981080118108009805180f8021bae3025001301d001163008301c001230223023302330233023001222232533302300114a0264a666048002264646464a66604aa66604a66e3c0040244cdc78010040a5013370e00600e2940dd718148019bae30283029002375a604e605060500026eb0c09800852818130009919198008008011129998128008a5eb804c8ccc888c8cc00400400c894ccc0ac004400c4c8cc0b4dd3998169ba90063302d37526eb8c0a8004cc0b4dd41bad302b0014bd7019801801981780118168009bae30240013756604a002660060066052004604e002646600200200a44a666048002297adef6c6013232323253330253371e911000021003133029337606ea4008dd3000998030030019bab3026003375c60480046050004604c0024604060426042604260426042604260420024466012004466ebcc018c0680040088c078c07cc07cc07cc07cc07cc07cc07cc07c0048c074c0780048c070004888c8c8c94ccc06ccdc3a40040022900009bad30203019002301900132533301a3370e90010008a60103d87a8000132323300100100222533302000114c103d87a800013232323253330213371e014004266e95200033025375000297ae0133006006003375a60440066eb8c080008c090008c088004dd5980f980c001180c000991980080080211299980e8008a6103d87a8000132323232533301e3371e010004266e95200033022374c00297ae01330060060033756603e0066eb8c074008c084008c07c0048dca0009119b8a00200122323300100100322533301900114c0103d87a8000132325333018300500213374a90001980e00125eb804cc010010004c074008c06c00488c8cc00400400c894ccc06000452809919299980b99b8f00200514a226600800800260380046eb8c0680048c058c05cc05c0048c94ccc044cdc3a400000226464a66602c60320042930b1bae3017001300f002153330113370e900100089919299980b180c8010a4c2c6eb8c05c004c03c00858c03c0045261365632533300f3370e90000008a99980918068028a4c2c2a66601e66e1d20020011323253330143017002132498c94ccc048cdc3a4000002264646464a66603260380042649319299980b99b87480000044c8c94ccc070c07c00852616375c603a002602a0082c602a0062c6eb4c068004c068008c060004c04000858c04000458c054004c03401454ccc03ccdc3a400800226464a666028602e0042930b1bad3015001300d00516300d0043001004232533300e3370e90000008a99980898060010a4c2c2a66601c66e1d200200113232323253330153018002149858dd7180b000980b0011bad3014001300c0021533300e3370e9002000899192999809980b0010a4c2c6eb8c050004c03000858c030004dd70009bae001375c0024600a6ea80048c00cdd5000ab9a5573aaae7955cfaba05742ae8930011e581ce6e5285a878161c101a59b4e36f1f99e5e464d30f510be3ee34f907f004c011e581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604ae004c011e581c2291f67ee643db1a830734bd54d39022c5d1f990682e689c95d8fed00001\"}}").unwrap();
        let utxo_7: UTxO = serde_json::from_str("{\"input\":{\"outputIndex\":7,\"txHash\":\"3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814\"},\"output\":{\"address\":\"addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x\",\"amount\":[{\"quantity\":\"10000000\",\"unit\":\"lovelace\"}],\"dataHash\":null,\"plutusData\":null,\"scriptHash\":null,\"scriptRef\":null}}").unwrap();

        let utxos = vec![utxo_1, utxo_2, utxo_3, utxo_4, utxo_5, utxo_6, utxo_7];
        let tx_hex = "84a700d90102848258202c255d39a6d448b408bdb1734c99dfc8c487ac23fd7ee5e8b431a99bc514980a0882582040e1afc8b735a9daf665926554b0e11902e3ed7e4a31a23b917483d4de42c05e04825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c6402825820ffb4e04fd430ffd1bdf014990c6d63a5303c1745ff228b70823fc757a04b1c64030184a3005839104477981671d60af19c524824cacc0a9822ba2a7f32586e57c18156215ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a0016e360a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a0243d580028201d81843d87980a300583910634a34d9c1ec5dd0cae61e4c86a4e85214bafdc80c57214fc80745b55ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a0075b8d4a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a1298be00028201d81858b1d8799fd8799fd87a9f581c57f7ddf8c822daad03fd80823153a61d913e5c9147bd478e3ccd70b3ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd8799fd87a9f581c4477981671d60af19c524824cacc0a9822ba2a7f32586e57c1815621ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd87a801a000985801a1dcd6500ffa300583910634a34d9c1ec5dd0cae61e4c86a4e85214bafdc80c57214fc80745b55ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb011a004c4b4003d818558203525101010023259800a518a4d136564004ae69a300583910634a34d9c1ec5dd0cae61e4c86a4e85214bafdc80c57214fc80745b55ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb011a0080ef61028201d81858b1d8799fd8799fd87a9f581c57f7ddf8c822daad03fd80823153a61d913e5c9147bd478e3ccd70b3ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd8799fd87a9f581c4477981671d60af19c524824cacc0a9822ba2a7f32586e57c1815621ffd8799fd8799fd8799f581c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cbffffffffd87a801a000985801a1dcd6500ff021a00051ceb0b5820a8fbe851b21a47d77c16808f56a3b4f10d8e5bea42cbc041804e0881a04aabcb0dd90102818258203fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814070ed9010282581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604ae581c5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa12d9010282825820efe6fbbdd6b993d96883b96c572bfcaa0a4a138c83bd948dec1751d1bfda09b300825820ac7744adce4f25027f1ca009f5cab1d0858753e62c6081a3a3676cfd5333bb0300a105a482000082d87980821a000382f61a04d45a0382000182d87980821a000382f61a04d45a0382000282d87980821a000382f61a04d45a0382000382d87980821a000382f61a04d45a03f5f6";
        let mut body = parse(tx_hex, &utxos).unwrap();

        // Edit body to remove last change output
        body.outputs.pop();
        body.reference_inputs.pop();

        let mut tx_builder = TxBuilder::new_core();
        tx_builder.tx_builder_body = body.clone();

        // Edit the tx builder body to add a new output
        let tx_hex = tx_builder.tx_out(
            "addr_test1zp355dxec8k9m5x2uc0yep4yapfpfwhaeqx9wg20eqr5td2u5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9sygqq0c",
            &[Asset::new_from_str("lovelace", "5000000")],
        )
        .invalid_before(100)
        .invalid_hereafter(200)
        .required_signer_hash("3f1b5974f4f09f0974be655e4ce94f8a2d087df378b79ef3916c26b2")
        .complete_sync(None).unwrap().tx_hex();

        let pallas_tx = Tx::decode_fragment(&hex::decode(tx_hex).unwrap()).unwrap();
        let mut output_lovelace: u64 = 0;
        for output in pallas_tx.transaction_body.outputs {
            match output {
                pallas_primitives::conway::PseudoTransactionOutput::Legacy(transaction_output) => {
                    match transaction_output.amount {
                        uplc::Value::Coin(coin) => {
                            output_lovelace += coin;
                        }
                        uplc::Value::Multiasset(coin, _) => {
                            output_lovelace += coin;
                        }
                    }
                }
                pallas_primitives::conway::PseudoTransactionOutput::PostAlonzo(conway_output) => {
                    match conway_output.value {
                        pallas_primitives::conway::Value::Coin(coin) => {
                            output_lovelace += coin;
                        }
                        pallas_primitives::conway::Value::Multiasset(coin, _) => {
                            output_lovelace += coin;
                        }
                    }
                }
            }
        }
        output_lovelace += pallas_tx.transaction_body.fee;

        // Assert total lovelace in outputs + fee equals input lovelace
        assert_eq!(output_lovelace, 5000000 + 1500000 + 1500000 + 15000000);
        // Assert validity interval
        assert_eq!(
            pallas_tx.transaction_body.validity_interval_start,
            Some(100)
        );
        assert_eq!(pallas_tx.transaction_body.ttl, Some(200));
        // Assert required signer edits
        assert!(pallas_tx
            .transaction_body
            .required_signers
            .unwrap()
            .iter()
            .map(|hash| hash.to_string())
            .collect::<Vec<String>>()
            .contains(&"3f1b5974f4f09f0974be655e4ce94f8a2d087df378b79ef3916c26b2".to_string()));
    }
}
