mod int_tests {
    use serde_json::{json, to_string};
    use sidan_csl_rs::{
        core::common::con_str0,
        model::{Asset, Budget, LanguageVersion},
    };
    use whisky::{
        builder::{ MeshTxBuilder, MeshTxBuilderParam, WData::{self, JSON}, WRedeemer},
        core::utils::merge_vkey_witnesses_to_transaction, model::Anchor,
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

        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let res = mesh
            .tx_in(
                "fc1c806abc9981f4bee2ce259f61578c3341012f3d04f22e82e7e40c7e7e3c3c",
                3,
                &[Asset::new_from_str("lovelace", "9692479606")],
                "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
            )
            .read_only_tx_in_reference(
                "8b7ea04a142933b3d8005bf98be906bdba10978891593b383deac933497e2ea7",
                1,
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
                data: JSON(
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
                data: JSON(
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
            .tx_out_inline_datum_value(&JSON(
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
            .change_output_datum(JSON(con_str0(json!([])).to_string()))
            .complete_sync(None);

        match res {
            Ok(_) => {
                let signed_tx = mesh.complete_signing().unwrap();
                println!("{}", signed_tx);
                assert!(mesh.core.mesh_csl.tx_hex != *"");
            }
            Err(e) => {
                println!("error: {:?}", e);
                // failing the test case
                panic!()
            }
        }
        println!("{}", mesh.core.mesh_csl.tx_hex);
        assert!(mesh.core.mesh_csl.tx_hex != *"");
    }

    #[test]
    fn test_simple_spend() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let signed_tx = mesh
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
            .complete_signing().unwrap();

        println!("{}", signed_tx);
        assert!(mesh.core.mesh_csl.tx_hex != *"");
    }

    #[test]
    fn test_simple_withdraw() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let signed_tx = mesh
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
        assert!(mesh.core.mesh_csl.tx_hex != *"");
    }

    #[test]
    fn test_plutus_withdraw() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let signed_tx = mesh
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
                data: JSON(con_str0(json!([])).to_string()),
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
        assert!(mesh.core.mesh_csl.tx_hex != *"");
    }

    #[test]
    fn test_native_script_ref() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = mesh
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

        let signed_tx = merge_vkey_witnesses_to_transaction(unsigned_tx, "a10081825820096348a7a3640d8ecc89819abffc7ed89cde399346046d50444acbd6e467f9df5840111279e89d341c9ab51f9ee7d5bb3a8db068ca6d09b7d3d4aaa48940dc55162903fd8f194df5c048055c9ac869e95729273b4ebb752be8a998f3483fac5d6e05".to_string());

        println!("{}", signed_tx);
        assert!(mesh.core.mesh_csl.tx_hex != *"");
    }

    #[test]
    fn test_plutus_script_cert_registration() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = mesh
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
        assert!(mesh.core.mesh_csl.tx_hex != *"");
    }

    #[test]
    fn test_plutus_script_cert_deregistration() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = mesh
                .tx_in("b3b05ac96e1eb4cd3b3cb8150cc48ee006d12683ed1b87ee57122d83235069df",
            0,
        &[Asset::new_from_str("lovelace", "1488554147")],
        "addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6",)
        .tx_in_collateral("541e2c5e6af1661a08aedf53fc4fb66aee00885629100196abbe42b05121adff", 5, &[Asset::new_from_str("lovelace", "5000000")], "addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
        .change_address("addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
        .deregister_stake_certificate("stake_test17rvfqm99c7apyjsyq73jm2ehktyzkyanmnv3z8jzjsxuafq5a6z2j")
        .certificate_script("5251010000322253330034a229309b2b2b9a01", Some(LanguageVersion::V2))
        .certificate_redeemer_value(&WRedeemer {            
            data: JSON(con_str0(json!([])).to_string()),
            ex_units: Budget {
                mem: 7000000,
                steps: 14000000
            }})
        .complete_sync(None)    
        .unwrap()
        .complete_signing().unwrap();

        println!("{}", unsigned_tx);
        assert!(mesh.core.mesh_csl.tx_hex != *"");
    }

    #[test]
    fn test_mint_two_tokens_with_same_policy() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = mesh.
        tx_in("b68d2e8340d9454c66b0530de8fdeca5bc829c577217b12f0c0beeb7f42b6b18", 0, &[Asset::new_from_str("lovelace", "100000000000")], "addr_test1qrfkkp5dwgj07fljdum677pglfm5707hd8nwj5wgfqdhfp0m7kq4cxp4nznl6v9yp2wxvwl2vsh0mk7eq7g97vczj6uqse4e3j")
        .tx_in_collateral("541e2c5e6af1661a08aedf53fc4fb66aee00885629100196abbe42b05121adff", 5, &[Asset::new_from_str("lovelace", "5000000")], "addr_test1qpsmz8q2xj43wg597pnpp0ffnlvr8fpfydff0wcsyzqyrxguk5v6wzdvfjyy8q5ysrh8wdxg9h0u4ncse4cxhd7qhqjqk8pse6")
        .mint_plutus_script_v2()
        .mint(1, "d8906ca5c7ba124a0407a32dab37b2c82b13b3dcd9111e42940dcea4",  "7465737431")
        .mint_redeemer_value(&WRedeemer {            
            data: JSON(con_str0(json!([])).to_string()),
            ex_units: Budget {
                mem: 7000000,
                steps: 14000000
            }})
        .minting_script("5251010000322253330034a229309b2b2b9a01")
        .mint_plutus_script_v2()
        .mint(1, "d8906ca5c7ba124a0407a32dab37b2c82b13b3dcd9111e42940dcea4", "7465737432")
        .mint_redeemer_value(&WRedeemer {            
            data: JSON(con_str0(json!([])).to_string()),
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
        assert!(mesh.core.mesh_csl.tx_hex != *"");
    }

    #[test]
    fn test_spend_withdraw_and_unreg() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let reward_address = "stake_test17q3hjj9svuvmmj5untsrclvlwzs8q528tzj0k3g5hgkzajc23t4fh";

        let unsigned_tx = mesh
        .spending_plutus_script_v2()
        .tx_in("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 0, &[Asset::new_from_str("lovelace", "6904620")], "addr_test1zrrpfzell3549ulhjwar3juz8dv8qcc99kfvlwrfzu2sw76u5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9shn8fam")
        .spending_tx_in_reference("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 1, "237948b06719bdca9c9ae03c7d9f70a070514758a4fb4514ba2c2ecb", 950)
        .tx_in_inline_datum_present()
        .spending_reference_tx_in_redeemer_value(&WRedeemer {            
            data: JSON(con_str0(json!([])).to_string()),
            ex_units: Budget {
                mem: 35588,
                steps: 13042895
            }})
        .spending_plutus_script_v2()
        .tx_in("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 1, &[Asset::new_from_str("lovelace", "5159070")], "addr_test1zrrpfzell3549ulhjwar3juz8dv8qcc99kfvlwrfzu2sw76u5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9shn8fam")
        .spending_tx_in_reference("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 1, "237948b06719bdca9c9ae03c7d9f70a070514758a4fb4514ba2c2ecb", 950)
        .tx_in_inline_datum_present()
        .spending_reference_tx_in_redeemer_value(&WRedeemer {            
            data: JSON(con_str0(json!([])).to_string()),
            ex_units: Budget {
                mem: 35588,
                steps: 13042895
            }})
        .deregister_stake_certificate(reward_address)
        .certificate_tx_in_reference("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 0, "237948b06719bdca9c9ae03c7d9f70a070514758a4fb4514ba2c2ecb", Some(LanguageVersion::V2), 953)
        .certificate_redeemer_value(&WRedeemer {            
            data: JSON(con_str0(json!([])).to_string()),
            ex_units: Budget {
                mem: 120022,
                steps: 44400485
            }})
        .withdrawal_plutus_script_v2()
        .withdrawal(reward_address, 0)
        .withdrawal_redeemer_value(&WRedeemer {            
            data: JSON(con_str0(json!([])).to_string()),
            ex_units: Budget {
                mem: 120022,
                steps: 44400485
            }})
        .withdrawal_tx_in_reference("e4e94d4369b5a1b6366d468bf01bf4d332d29abd8061889e6d80fc5074248ed1", 0, "237948b06719bdca9c9ae03c7d9f70a070514758a4fb4514ba2c2ecb", 953)
        .read_only_tx_in_reference("d3e7e43ec9c85cfdb90f98fb40bb4edd58fdd3d056e32f827739fe0b915c6eb7", 0)
        .change_address("addr_test1qqjcvv7huxlf9epjq49j4952pez8l4zyrm6c4wrf2vtcym4jg6fd5d54p0k5mqy46ph5z3r59tkhnhjvsxx53dq5rvdsnaeh3a")
        .tx_in_collateral("3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814", 7, &[Asset::new_from_str("lovelace", "10000000")], "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x")
        .required_signer_hash("258633d7e1be92e432054b2a968a0e447fd4441ef58ab8695317826e")
        .required_signer_hash("5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa")
        .complete_sync(None)
        .unwrap()
        .complete_signing()
        .unwrap();    

        println!("{}", unsigned_tx);
        assert!(mesh.core.mesh_csl.tx_hex != *"");
    }

    #[test]
    fn test_embedded_datum_output() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });
        let signed_tx = mesh
            .tx_in(
                "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
                3,
                &[Asset::new_from_str("lovelace", "9891607895")],
                "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
            )
            .tx_out("addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh", &[Asset::new_from_str("lovelace", "2000000")])
            .tx_out_datum_embed_value(&WData::JSON(json!({
                "constructor": 0,
                "fields": []
              }).to_string()))
            .change_address("addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh")
            .signing_key("51022b7e38be01d1cc581230e18030e6e1a3e949a1fdd2aeae5f5412154fe82b")
            .complete_sync(None)
            .unwrap()
            .complete_signing().unwrap();

        println!("{}", signed_tx);
        assert!(mesh.core.mesh_csl.tx_hex != *"");
    }

    #[test]
    fn test_register_drep() {
        let mut mesh = MeshTxBuilder::new(MeshTxBuilderParam {
            evaluator: None,
            fetcher: None,
            submitter: None,
            params: None,
        });

        let unsigned_tx = mesh
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

        println!("{:?}", json!(mesh.core.mesh_tx_builder_body).to_string());
        println!("{}", unsigned_tx);
    }
}
