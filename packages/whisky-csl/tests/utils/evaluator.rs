#[cfg(test)]
mod tests {
    use cardano_serialization_lib::{self as csl};
    use pallas_codec::minicbor::Decoder;
    use pallas_primitives::conway::ScriptRef;
    use serde_json::json;
    use uplc::tx::SlotConfig;
    use whisky_common::*;
    use whisky_csl::{evaluate_tx_scripts, to_pallas_script_ref, JsonSlotConfig};

    #[test]
    fn test_eval() {
        let result = evaluate_tx_scripts(
            "84a80082825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad9800825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98010d81825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad9801128182582004b9070a30bd63abaaf59a3c48a1575c4127bb0edb00ecd5141fd18a85c721aa000181a200581d601fd5bab167338971d92b4d8f0bdf57d889903e6e934e7ea38c7dadf1011b00000002529898c810a200581d601fd5bab167338971d92b4d8f0bdf57d889903e6e934e7ea38c7dadf1011b0000000252882db4111a000412f1021a0002b74b0b5820775d0cf3c95993f6210e4410e92f72ebc3942ce9c1433694749aa239e5d13387a200818258201557f444f3ae6e61dfed593ae15ec8dbd57b8138972bf16fde5b4c559f41549b5840729f1f14ef05b7cf9b0d7583e6777674f80ae64a35bbd6820cc3c82ddf0412ca1d751b7d886eece3c6e219e1c5cc9ef3d387a8d2078f47125d54b474fbdfbd0105818400000182190b111a000b5e35f5f6",
            &vec![UTxO {
                input: UtxoInput {
                    tx_hash: "604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98".to_string(),
                    output_index: 0
                },
                output: UtxoOutput {
                    address: "addr_test1wzlwsgq97vchypqzk8u8lz30w932tvx7akcj7csm02scl7qlghd97".to_string(),
                    amount: vec![Asset::new_from_str("lovelace", "986990")],
                    data_hash: None,
                    plutus_data: Some(csl::PlutusData::from_json(&
                                                                     json!({
                                                                         "constructor": 0,
                                                                         "fields": []
                                                                     })
                                                                         .to_string(), csl::PlutusDatumSchema::DetailedSchema).unwrap().to_hex()),
                    script_hash: None,
                    script_ref: None,
                }
        },
                  UTxO {
                      input: UtxoInput {
                          tx_hash: "604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98".to_string(),
                          output_index: 1
                      },
                      output: UtxoOutput {
                          address: "addr_test1vq0atw43vuecjuwe9dxc7z7l2lvgnyp7d6f5ul4r3376mug8v67h5".to_string(),
                          amount: vec![Asset::new_from_str("lovelace", "9974857893")],
                          data_hash: None,
                          plutus_data: None,
                          script_hash: None,
                          script_ref: None,
                      }
                  },
                  UTxO {
                      input: UtxoInput {
                          tx_hash: "04b9070a30bd63abaaf59a3c48a1575c4127bb0edb00ecd5141fd18a85c721aa".to_string(),
                          output_index: 0
                      },
                      output: UtxoOutput {
                          address: "addr_test1wzlwsgq97vchypqzk8u8lz30w932tvx7akcj7csm02scl7qlghd97".to_string(),
                          amount: vec![Asset::new_from_str("lovelace", "986990")],
                          data_hash: None,
                          plutus_data: None,
                          script_hash: None,
                          script_ref: Some("82025655010000322223253330054a229309b2b1bad0025735".to_string())
                      }
                  }],
            &[],
            &Network::Mainnet,
            &SlotConfig::default()
        );

        let redeemers = result.unwrap();
        let mut redeemer_json = serde_json::Map::new();

        assert_eq!(redeemers.len(), 1);

        let redeemer = match &redeemers[0] {
            EvalResult::Success(redeemer) => Ok(redeemer),
            EvalResult::Error(_) => Err("Unexpected error"),
        }
        .unwrap();

        redeemer_json.insert("index".to_string(), redeemer.index.to_string().into());
        let mut ex_unit_json = serde_json::Map::new();
        ex_unit_json.insert("mem".to_string(), redeemer.budget.mem.into());
        ex_unit_json.insert("steps".to_string(), redeemer.budget.steps.into());
        redeemer_json.insert(
            "ex_units".to_string(),
            serde_json::Value::Object(ex_unit_json),
        );
        assert_eq!(
            serde_json::json!({"ex_units":{"mem":2833,"steps":528893},"index":"0"}).to_string(),
            serde_json::json!(redeemer_json).to_string()
        )
    }

    #[test]
    fn test_eval_2() {
        let result = evaluate_tx_scripts(
            "84a800d90102818258205fefa1a58e20732f7e55972315dda0123b90e5a1d7f086bc37b566f0f9210229000182a300581d71fe23153596f3415c21374454937dd858b1afcce3ee9c7d43fb05a90f01821a00178b44a1581c9e138ea088768af3d2f1216271f2f41172a96b54780225036bc0475ea14001028201d8185859d8799f1b00000193ae9213085820cff20d3f762fecf4f90a186062263bbe07928bff70d50166be88c4c2bffbd10ed8799fd87a9f581cd649c0eb6532b3649e1a2b358fd83ce5778d1d91fdc8d3f6e0539a20ffd87a80ff03ff825839013354e5941d41d08051005f407b50355a3e7ad6bb983016028baae6a1f3f2436fa221ebc2d81481ee126944d7e11256ea793da594e0e033921a00f19ca6021a000eda89031a0860837005a1581df124f5fe1c897b34180e6dcbc57fa7b2435b4d7cf6e8f3773a3edf08250009a1581c9e138ea088768af3d2f1216271f2f41172a96b54780225036bc0475ea140010b582071c6e77365d456e9ed48e7bfbe4c76a5d7f91a999080624f60d40922661c9fc80dd90102818258205fefa1a58e20732f7e55972315dda0123b90e5a1d7f086bc37b566f0f921022900a301d9010281830303878200581cf800e7ea3e1312cccf402b957e568e0d29786efe3f9cb818caadc7e38200581cfcbb77e221db810dfcbb9c9684c9c23e7f4c25868a20ff0701bf619e8200581c2efaf69bb7551328bb8625684bc1cea922ebc5aeca48df09b7b830458200581c0eaabc8286318f13f45d1eb7383e100964a94a01124b184bde55ba9a8200581c804ff6e71914f1ef08e31c86dc3ee7046424b464a6ee1536b38a04e38200581c21c3bd0ab2357a1879a96b948145fee311015430a68617996901b8bb8200581cd0730dfe0cb0140621dcdf751bbea7b1fb03c5eca57779c3fbc07f1706d9010281590b84590b8101000033332323232323232323232322322232232323232322533300f3232323232325333015300b3017375400226464a66602e66ebcdd30009ba633300500248900480084c94ccc060c038c068dd500089919299980d19b8748010c070dd500089919191919299980f980a98109baa00113232533302153330213375e602060486ea80240705288a998112481366f776e5f6f75747075742e61646472657373203d3d20636f6c645f696e74656e745f686f6c645f61646472657373203f2046616c73650014a02a666042a666042008294454cc0892411174746c5f636865636b203f2046616c73650014a02a666042002294454cc089240117617574686f726974795f636865636b203f2046616c73650014a02940528299981019198008009bab302730283028302830283028302830243754602060486ea804c894ccc0980045280992999811992999812180d18131baa00113375e6054604e6ea800406c54cc0952412465787065637420496e6c696e65286372656429203d207769746864726177616c2e31737400163025302900214a22660060060026052002266e1cdd6981318139813981398119baa00500114a06eb4c094c088dd50008a9981024814265787065637420536f6d6528617574686f726974795f6e756d62657229203d206765745f666972737428617574686f726974795f6d61702c2072656465656d6572290016323300100101b22533302400114c0103d87a8000132323253330233375e034604a006266e95200033028375000297ae0133005005002375a604a0046050004604c0026464a66603e602a0022a6604092012a5472616e73616374696f6e2074746c2073686f756c64206861766520616e20757070657220626f756e6400161533301f30140011337126eb4c094c088dd50011bad300e302237540082a66040921365472616e73616374696f6e20757070657220626f756e642073686f756c646e277420626520706f73697469766520696e66696e697479001630203754002601860406ea8c08cc090c080dd51811981218121812181218121812181218101baa300c3020375401ea6660386024603c6ea80044c94ccc08400454cc078070584c8c94ccc08c00454cc080078584c8c94ccc09400454cc088080584c8c94ccc09c00454cc090088584c94ccc0a0c0ac0084c9265333024301a30263754006264a6660520022a6604c0482c26464a6660560022a6605004c2c264a666058605e0042646493192999815181000089929998178008a998160150b0992999818181980109924c64a66605a6046002264a6660640022a6605e05a2c264a666066606c00426493198118008170a998180170b19299999981b80088008a998180170b0a998180170b0a998180170b0a998180170b181a00098181baa0021533302d30220011325333032001153302f02d16132325333034001153303102f161323253330360011533033031161325333037303a002149854cc0d00c858c94cccccc0ec00454cc0d00c85854cc0d00c85854cc0d00c8584dd68008a9981a0190b181c000981c00119299999981c8008a998190180b0a998190180b0a998190180b09bad00115330320301630360013036002325333333037001153303002e16153303002e16153303002e161375a0022a6606005c2c606800260606ea800854cc0b80b058c0b8dd50008a998168158b19299999981a00088008a998168158b0a998168158b0a998168158b0a998168158b181880098169baa0031533302a301f0011533302e302d37540062930a998158148b0a998158148b18159baa0023301c0030271533029027163253333330300011001153302902716153302902716153302902716153302902716302d001302d00232533333302e0011001153302702516153302702516153302702516153302702516302b001302737540062a6604a0462c2a6604a0462c64a6666660580022a6604a0462c2a6604a0462c2a6604a0462c26eb400454cc09408c58c0a4004c0a4008c94cccccc0a8004400454cc08c0845854cc08c0845854cc08c0845854cc08c08458c09c004c09c008c94cccccc0a000454cc08407c5854cc08407c5854cc08407c5854cc08407c584dd7000981280098128011929999998130008a9980f80e8b0a9980f80e8b0a9980f80e8b09bad001153301f01d163023001301f37540022a6603a0362ca66666604600220022a660380342c2a660380342c2a660380342c2a660380342c6040603a6ea800454cc06d2413665787065637420496e6c696e65446174756d286f776e5f6f75747075745f6461746129203d206f776e5f6f75747075742e646174756d00163006301c3754002603c60366ea800454cc065241146f776e206f7574707574206e6f7420666f756e64001632533301c00114c103d87a800013374a90001980e980f000a5eb80c8cc004004dd61802980d9baa3007301b375401444a66603a002297ae013232533301b3375e6e98cc030dd59805980f1baa002488100374c66601200c91100480084cc080008cc0100100044cc010010004c084008c07c00454ccc05ccdd79ba6001374c66600a00491100480045288a503300737566038603a603a603a603a60326ea8c014c064dd500424500375c603660306ea800454cc059240128657870656374204d696e74286f776e5f706f6c6963795f696429203d206374782e707572706f7365001630043017375400c4603460366036002444a66602a6016002297adef6c6013232330010014bd6f7b63011299980e00089980e99bb0375200c6e9800d2f5bded8c0264646464a66603866e400280084cc084cdd81ba900a374c00e00a2a66603866e3c0280084cc084cdd81ba900a374c00e00626604266ec0dd48011ba6001330060060033756603c0066eb8c070008c080008c078004c8cc0040052f5bded8c044a66603600226603866ec0dd48021ba80034bd6f7b630099191919299980d99b90008002133020337606ea4020dd40038028a99980d99b8f008002133020337606ea4020dd400380189981019bb037520046ea0004cc01801800cdd6980e8019bae301b002301f002301d0012301800123017301800122323300100100322533301700114bd6f7b630099191919299980b99b91007002153330173371e00e0042006200a26603866ec0dd48011ba600133006006003375660320066eb8c05c008c06c008c064004526153301049011856616c696461746f722072657475726e65642066616c736500136563300100400b2232533300f3005001132533301400115330110031613253330153018002149854cc04801058c94cccccc06400454cc0480105854cc0480105854cc0480105854cc048010584dd7000980b00098091baa0031533300f3004001132533301400115330110031613253330153018002149854cc04801058c94cccccc06400454cc0480105854cc0480105854cc0480105854cc048010584dd7000980b00098091baa003153301000216301037540046e1d2002370e9000299999980880088008a998050038b0a998050038b0a998050038b0a998050038b1bae00137560029211472656465656d65723a2043726564656e7469616c0049013d657870656374206f776e5f6f75747075745f646174756d3a20436f6c64526566496e74656e74446174756d203d206f776e5f6f75747075745f64617461005734ae7155ceaab9e5573eae815d0aba25748981b0a5d87a9f581c24f5fe1c897b34180e6dcbc57fa7b2435b4d7cf6e8f3773a3edf0825ff03d87a9f581c883af74520be6f27da1ee389aee3c8ce5b11c7fb24d715404caec1c4ff04d87a9f581c059030947d487b5a1230cb9acae8f93fb29cb03d8061f798133d6de3ff05d87a9f581cc844f7034f137e3230540be47a22e3c6693c3bc76f728ada836d1b8fff06d87a9f581c97a5e64f557774e55fda8171feb7d3af93b5567838b3b25ae99990d9ff07004c0129d8799fd87a9f581cfe23153596f3415c21374454937dd858b1afcce3ee9c7d43fb05a90fffd87a80ff004c01165554737974787743327542326c4b654767436b755136000105a182010082d87a9f581c24f5fe1c897b34180e6dcbc57fa7b2435b4d7cf6e8f3773a3edf0825ff821a006acfc01ab2d05e00f5f6",
            &[UTxO {
                input: UtxoInput {
                    tx_hash: "5fefa1a58e20732f7e55972315dda0123b90e5a1d7f086bc37b566f0f9210229".to_string(),
                    output_index: 0
                },
                output: UtxoOutput {
                    address: "addr1qye4fev5r4qapqz3qp05q76sx4dru7kkhwvrq9sz3w4wdg0n7fpklg3pa0pds9ypacfxj3xhuyf9d6ne8kjefc8qxwfqply43d".to_string(),
                    amount: vec![Asset::new_from_str("lovelace", "18350707")],
                    data_hash: None,
                    plutus_data: None,
                    script_hash: None,
                    script_ref: None,
                }
            }],
            &[],
            &Network::Mainnet,
            &SlotConfig::default()
        );
        assert_eq!(
            serde_json::json!([{"success": {"budget":{"mem":184912,"steps":61492185},"index": 0, "tag": "mint"}}]).to_string(),
            serde_json::json!(result.unwrap()).to_string()
        )
    }

    #[test]
    fn test_eval_3() {
        let result = evaluate_tx_scripts(
            "84a700d90102828258206eb7aa11907c377653e41dfd995dae8e6d468cfea0a645cdb4f4341fb963e8780282582078071c027011b376f0decf5e48f204d229be66c5711269101a98de1e7d30f43c010183a300581d70cc8ecbc5fab3ff89558bd3f6fe9dee68b448d7fe0f9a4b33cd38005301821a001e8480a1581cd1b3c84126916e1be2595d030194926546009f712a5432d0e7a2f717a14001028201d8185854d8799f581cfdeb4bf0e8c077114a4553f1e05395e9fb7114db177f02f7b65c8de4a240a1401a1dcd6500581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a1dcd6500ff82581d7064a3f2ba1bb2084486728c9bb5f6c4b5c73598e2746f441593bda40f821a1daee080a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a1dcd650082583900fdeb4bf0e8c077114a4553f1e05395e9fb7114db177f02f7b65c8de44e660f79ce4221d52a2dc249da925112b3ea46bcaba9ce48174fa358821b00000001a49f62e3a2581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581b00000016d0f508c0581cb80aa257a376c9ae7aa0c7a323db88d236e11e0a5ed5e10142da9ea0a14b000de140626561636f6e3301021a000c8a6d09a1581cd1b3c84126916e1be2595d030194926546009f712a5432d0e7a2f717a140010b582011bb34b7d88c308fd3ef46f2d8d149921b47aa74ea9f9d2e6f9452323c94d4b60dd90102818258207ab42fa758f9d5772a212cda5f866a3e1ad9948ce89f916feafeef73cbbabdb4000ed9010281581cfa5136e9e9ecbc9071da73eeb6c9a4ff73cbf436105cf8380d1c525ca207d901028158b158af010100333232323232232225333005323232323232330010013758601c601e601e601e601e601e601e601e601e60186ea8c038c030dd50039129998070008a50132533300d3371e6eb8c04000802c52889980180180098080009806180680118058009805801180480098031baa00114984d958dd7000ab9a5573caae7d5d0aba24c011e581cfa5136e9e9ecbc9071da73eeb6c9a4ff73cbf436105cf8380d1c525c004c010746332d6d696e74000105a182010082d87980821a006acfc01ab2d05e00f5f6",
            &[
                UTxO {
                    input: UtxoInput {
                        tx_hash: "6eb7aa11907c377653e41dfd995dae8e6d468cfea0a645cdb4f4341fb963e878".to_string(),
                        output_index: 2
                    },
                    output: UtxoOutput {
                        address: "addr_test1qr77kjlsarq8wy22g4flrcznjh5lkug5mvth7qhhkewgmezwvc8hnnjzy82j5twzf8dfy5gjk04yd09t488ys9605dvq4ymc4x".to_string(),
                        amount: vec![
                            Asset::new_from_str("lovelace", "7507698128"), 
                            Asset::new_from_str("5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04b55534458", "98430000000"),
                            Asset::new_from_str("b80aa257a376c9ae7aa0c7a323db88d236e11e0a5ed5e10142da9ea0000de140626561636f6e33", "1"), 
                        ],
                        data_hash: None,
                        plutus_data: None,
                        script_hash: None,
                        script_ref: None,
                    }
                },
                UTxO {
                    input: UtxoInput {
                        tx_hash: "78071c027011b376f0decf5e48f204d229be66c5711269101a98de1e7d30f43c".to_string(),
                        output_index: 1
                    },
                    output: UtxoOutput {
                        address: "addr_test1qr77kjlsarq8wy22g4flrcznjh5lkug5mvth7qhhkewgmezwvc8hnnjzy82j5twzf8dfy5gjk04yd09t488ys9605dvq4ymc4x".to_string(),
                        amount: vec![
                            Asset::new_from_str("lovelace", "50000000"), 
                            Asset::new_from_str("5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04b55534458", "65000000"),
                        ],
                        data_hash: None,
                        plutus_data: None,
                        script_hash: None,
                        script_ref: None,
                    }
                },
                UTxO {
                    input: UtxoInput {
                        tx_hash: "7ab42fa758f9d5772a212cda5f866a3e1ad9948ce89f916feafeef73cbbabdb4".to_string(),
                        output_index: 0
                    },
                    output: UtxoOutput {
                        address: "addr_test1qr77kjlsarq8wy22g4flrcznjh5lkug5mvth7qhhkewgmezwvc8hnnjzy82j5twzf8dfy5gjk04yd09t488ys9605dvq4ymc4x".to_string(),
                        amount: vec![
                            Asset::new_from_str("lovelace", "5000000"), 
                        ],
                        data_hash: None,
                        plutus_data: None,
                        script_hash: None,
                        script_ref: None,
                    }
                },
            ],
            &[],
            &Network::Mainnet,
            &SlotConfig::default()
        );
        assert_eq!(
            serde_json::json!([{"success": {"budget":{"mem":15167,"steps":4549992},"index": 0, "tag": "mint"}}]).to_string(),
            serde_json::json!(result.unwrap()).to_string()
        )
    }

    #[test]
    fn test_eval_4() {
        let result = evaluate_tx_scripts(
            "84a600d90102828258201e126e978ffbc3cd396fb2b69ce3368abb353443292e0ae56f6acf6f3c97022800825820e0b92c29cad3b1c8c8c192eae238f8f21748673b6ce296ec8c2fd7f1935bb3e902018182583900d161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604aea63c153fb3ea8a4ea4f165574ea91173756de0bf30222ca0e95a649a1a06b9f40a021a0002d5610b5820582d077fcb0ea39e9c803a5f09cc543c3777ee3fbc8295b4cedb6617e1242b5b0dd9010281825820e0b92c29cad3b1c8c8c192eae238f8f21748673b6ce296ec8c2fd7f1935bb3e9050ed9010281581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604aea207d901028159011659011301010032323232323225980099191919192cc004cdc3a400460106ea80062646464b30013370e900018059baa005899192cc004c04400a2b30013370e900018069baa003899192cc004cdc79bae30023010375401291010d48656c6c6f2c20576f726c6421008800c528201c332232330010010032259800800c52844c96600266e3cdd7180b0010024528c4cc00c00c005012180b000a0283758602260246024602460246024602460246024601e6ea8028dd7180098079baa3011300f37540084602200316403116403c6eb8c03c004c030dd5002c5900a18069807001180600098049baa0018b200e300a300b00230090013009002300700130043754003149a26cac80115cd2ab9d5573caae7d5d0aba2105a182000082d8799f4d48656c6c6f2c20576f726c6421ff820000f5f6",
            &[UTxO {
                input: UtxoInput {
                    tx_hash: "1e126e978ffbc3cd396fb2b69ce3368abb353443292e0ae56f6acf6f3c970228".to_string(),
                    output_index: 0
                },
                output: UtxoOutput {
                    address: "addr_test1wq8fgm5nunfyc0u3qxhl79me0zzzl85u6ujjn67c9zw98hgxz0k3a".to_string(),
                    amount: vec![Asset::new_from_str("lovelace", "1155080")],
                    data_hash: Some("0d124e70a7b3ee10e29ef38042c675927f5fa12af0a9a2084a630dffd366982c".to_string()),
                    plutus_data: Some("d8799f581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604aeff".to_string()),
                    script_hash: None,
                    script_ref: None,
                }
            },
            UTxO {
                input: UtxoInput {
                    tx_hash: "e0b92c29cad3b1c8c8c192eae238f8f21748673b6ce296ec8c2fd7f1935bb3e9".to_string(),
                    output_index: 2
                },
                output: UtxoOutput {
                    address: "addr_test1wq8fgm5nunfyc0u3qxhl79me0zzzl85u6ujjn67c9zw98hgxz0k3a".to_string(),
                    amount: vec![Asset::new_from_str("lovelace", "111880547")],
                    data_hash: None,
                    plutus_data: None,
                    script_hash: None,
                    script_ref: None,
                }
            }],
            &[],
            &Network::Mainnet,
            &SlotConfig::default()
        );
        assert_eq!(
            serde_json::json!([{"success": {"budget":{"mem":28243,"steps":8926884},"index": 0, "tag": "spend"}}]).to_string(),
            serde_json::json!(result.unwrap()).to_string()
        )
    }

    #[test]
    fn test_cbor() {
        let script_bytes = hex::decode("5655010000322223253330054a229309b2b1bad0025735").unwrap();
        let decoded_bytes = Decoder::new(&script_bytes).bytes().unwrap();
        assert_eq!(
            hex::decode("55010000322223253330054a229309b2b1bad0025735").unwrap(),
            decoded_bytes
        );
    }

    #[test]
    fn test_v1_script_ref() {
        let script_ref = to_pallas_script_ref(&Some(
            "82015655010000322223253330054a229309b2b1bad0025735".to_string(),
        ))
        .unwrap()
        .unwrap();

        match script_ref.0 {
            ScriptRef::PlutusV1Script(_) => {}
            _ => panic!("Invalid script ref"),
        }
    }

    #[test]
    fn test_v2_script_ref() {
        let script_ref = to_pallas_script_ref(&Some(
            "82025655010000322223253330054a229309b2b1bad0025735".to_string(),
        ))
        .unwrap()
        .unwrap();

        match script_ref.0 {
            ScriptRef::PlutusV2Script(_) => {}
            _ => panic!("Invalid script ref"),
        }
    }

    #[test]
    fn test_v3_script_ref() {
        let script_ref = to_pallas_script_ref(&Some(
            "82035655010000322223253330054a229309b2b1bad0025735".to_string(),
        ))
        .unwrap()
        .unwrap();

        match script_ref.0 {
            ScriptRef::PlutusV3Script(_) => {}
            _ => panic!("Invalid script ref"),
        }
    }

    #[test]
    fn test_invalid_native_script_ref() {
        let script_ref = to_pallas_script_ref(&Some(
            "82005655010000322223253330054a229309b2b1bad0025735".to_string(),
        ));
        assert!(script_ref.is_err());
    }

    #[test]
    fn test_network_type_decode() {
        let network = Network::Mainnet;
        let network_str = "Mainnet";
        let network_type: Network = network_str.to_string().try_into().unwrap();
        assert_eq!(network, network_type);
    }

    #[test]
    fn test_network_type_decode_error() {
        let network_str = "Invalid";
        let network_type: Result<Network, _> = network_str.to_string().try_into();
        assert!(network_type.is_err());
    }

    #[test]
    fn config_test() {
        println!(
            "{:?}",
            serde_json::to_string(&JsonSlotConfig {
                slot_length: 1000,
                zero_slot: 0,
                zero_time: 1666656000000,
            })
            .unwrap()
        );
    }
}
