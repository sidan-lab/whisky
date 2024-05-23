mod transaction_tests {
    use cardano_serialization_lib as csl;
    use sidan_csl_rs::core::utils::{build_tx_builder, calculate_tx_hash, to_bignum};

    #[test]
    fn test_calculate_tx_hash() {
        let tx_hex = "84a30081825820cc24e6f228e04d98c80088c830a363fff80a2437959f826e1a5b4c01ec912d0f010182a200581d605ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa011a001c0242a200581d601fd5bab167338971d92b4d8f0bdf57d889903e6e934e7ea38c7dadf1011b0000000252fe47ac021a00028759a100818258201557f444f3ae6e61dfed593ae15ec8dbd57b8138972bf16fde5b4c559f41549b5840b8317b840d4e908cd6a69bad0d294a593a40812749ccacdea993c660952a57cdf89428934973848a1437820b9f0e5784ddc01eb049415d4189977fdc32fda904f5f6";
        let tx_hash = calculate_tx_hash(tx_hex);
        assert_eq!(
            tx_hash,
            "c162f8abf8405b1d7f8f7677bc391b2d8f1911e73035cb97634b2dede72404cf"
        )
    }

    #[test]
    fn test_calculate_tx_hash_2() {
        let tx_hex = "84a400828258200f88c351c8afb3494b70dc2128e61289ea279fee7516db2c58e1562ce8576bbd028258208bbb363df8e0bcadf6b4ac473a06d94d75be243e0772ffbfc34571ea39873a5c000182a3005839008f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f01821a0012593aa1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a00989680028201d81843d87980825839008f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f1ab51927b1021a0002a04509a0a0f5f6";
        let signed_tx_hex = "84a400828258200f88c351c8afb3494b70dc2128e61289ea279fee7516db2c58e1562ce8576bbd028258208bbb363df8e0bcadf6b4ac473a06d94d75be243e0772ffbfc34571ea39873a5c000182a3005839008f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f01821a0012593aa1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a00989680028201d81843d87980825839008f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f1ab51927b1021a0002a04509a0a10081825820eb125c9530b870bab17f5f30dcbf029929c4d8743e4eaaf71a5e883d41a236ce5840bc63f18abf97e386743b00baf8e829a73e19bf600c8bdfc0e53d14c5171c7f6d62adc5a7081b06465d7003641ec2406421316424d216e06605323ebc68c1600cf5f6";
        let tx_hash_from_unsigned_hex = calculate_tx_hash(tx_hex);
        let tx_hash_from_signed_hex = calculate_tx_hash(signed_tx_hex);
        assert_eq!(
            tx_hash_from_unsigned_hex,
            "e8b7aefcee2953cf55a01c97565cfe9d414a21e17064d8fcef1f632f7311f933"
        );
        assert_eq!(
            tx_hash_from_signed_hex,
            "e8b7aefcee2953cf55a01c97565cfe9d414a21e17064d8fcef1f632f7311f933"
        )
    }

    #[test]
    fn test_calculate_tx_hash_3() {
        let tx_hex = "84a80084825820333d35345958010502188e210635fe1e7c6818124258a4e263befdb9d960013b02825820333d35345958010502188e210635fe1e7c6818124258a4e263befdb9d960013b03825820333d35345958010502188e210635fe1e7c6818124258a4e263befdb9d960013b04825820da54b203b14cd36ede2e95f172db58eee5e0a86170a8f757c862ffc208d8679c000186a300583910ccf8e7e341a62a8ffd345d1705b9103a661fd8faa403b33080fac35c5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a0016e360a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a0ee6b280028201d81843d87980a300583910d5f9fc2c6b5300b516ab106797632baeab6047843c06b5706e04e16f5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a001f8697a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a02349340028201d81843d87980a30058391057f7ddf8c822daad03fd80823153a61d913e5c9147bd478e3ccd70b35ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb011a1dcd6500028201d81843d87980a30058391057f7ddf8c822daad03fd80823153a61d913e5c9147bd478e3ccd70b35ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb011a0057bcf0028201d81843d87980a30058391057f7ddf8c822daad03fd80823153a61d913e5c9147bd478e3ccd70b35ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb01821a0016e360a1581c5066154a102ee037390c5236f78db23239b49c5748d3d349f3ccf04ba144555344581a00c65d40028201d81843d8798082581d605ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa1a00111e08021a0005c55809a00b58205145c568a057f9d1786528dd699ec6c78592f228c9d1810f7c78c110a87f2be70d818258203fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814070e82581cd161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604ae581c5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa1287825820da54b203b14cd36ede2e95f172db58eee5e0a86170a8f757c862ffc208d8679c00825820b1754a72245efa4ffe7e1ae04eaa935f1b0df74b873de631b54b67079712987b00825820efe6fbbdd6b993d96883b96c572bfcaa0a4a138c83bd948dec1751d1bfda09b300825820333d35345958010502188e210635fe1e7c6818124258a4e263befdb9d960013b02825820333d35345958010502188e210635fe1e7c6818124258a4e263befdb9d960013b0382582052e37c35580fb1c1081ba55097a5e82d46962f2f9ecfaa6cda9e77fe36579df200825820333d35345958010502188e210635fe1e7c6818124258a4e263befdb9d960013b04a30082825820aa8ce9e908f525c3b700a65669430ec68ca19615e7309e25bb6fa883964cfa9f5840f1682fdbf1e7acc4272e2f12d4a41f46242095fcfad173d6c2e4a0c86c48ee45be5710e5af0dce563ed84b3cfd97ad168741af925a2d88b9280ca236e0484b0f8258207f4747ca0c20a1e5c28716c4a10fffbcbe8fe6253cb427ae2f0e24d231a980845840314afcde73a9281bdd9e9395f474cdd4821650497df0383aa3e50f4d1299b35c5842efa4876b5864bb9d16dd34e344725bf853a7eea666322fef1fa5ed82b80803800584840000d87980821a0007a1201a02faf080840001d87980821a0007a1201a02faf080840002d87980821a0007a1201a02faf080840003d87980821a000f42401a05f5e100f5f6";
        let tx_hash_from_unsigned_hex = calculate_tx_hash(tx_hex);
        assert_eq!(
            tx_hash_from_unsigned_hex,
            "b9469d90b1e2861b338f544820b696e9622c6a42526ae8c7a9cf16ff245eaba8"
        );
    }

    #[test]
    fn test_add_change() {
        let mut tx_builder = build_tx_builder(None);

        let mut test_value = csl::Value::new(&to_bignum(100000000));
        let mut test_multi_asset = csl::MultiAsset::new();
        let mut test_assets = csl::Assets::new();
        let test_assets_names = vec![
            "31", "32", "33", "34", "35", "36", "37", "38", "39", "3130", "3131", "3132", "3133",
            "3134", "3135", "3136", "3137", "3138", "3139", "3230", "3231", "3232", "3233", "3234",
            "3235", "3236", "3237", "3238", "3239", "3330", "3331", "3332", "3333", "3334", "3335",
            "3336", "3337", "3338", "3339", "3430", "3431", "3432", "3433", "3434", "3435", "3436",
            "3437", "3438", "3439", "3530", "3531", "3532", "3533", "3534", "3535", "3536", "3537",
            "3538", "3539", "3630", "3631", "3632", "3633", "3634", "3635", "3636", "3637", "3638",
            "3639", "3730", "3731", "3732", "3733", "3734", "3735", "3736", "3737", "3738", "3739",
            "3830", "3831", "3832", "3833", "3834", "3835", "3836", "3837", "3838", "3839", "3930",
            "3931", "3932", "3933", "3934", "3935", "3936", "3937", "3938", "3939", "313030",
            "313031",
        ];

        for asset_name in test_assets_names {
            test_assets.insert(
                &csl::AssetName::new(hex::decode(asset_name).unwrap()).unwrap(),
                &csl::BigNum::one(),
            );
        }

        for i in 0..20 {
            let native_script =
                csl::NativeScript::new_timelock_start(&csl::TimelockStart::new_timelockstart(
                    &csl::BigNum::from_str(&i.to_string()).unwrap(),
                ));

            test_multi_asset.insert(&native_script.hash(), &test_assets);
        }

        test_value.set_multiasset(&test_multi_asset);

        let mut tx_inputs_builder = csl::TxInputsBuilder::new();
        let _ = tx_inputs_builder.add_regular_input(
            &csl::Address::from_bech32(
                "addr_test1vpvwjd8za9wj8kzse2pm9ch9xcd9zu6aex99jyd6rrgntdqq2vvut",
            )
            .unwrap(),
            &csl::TransactionInput::new(
                &csl::TransactionHash::from_hex(
                    "a04996d5ef87fdece0c74625f02ee5c1497a06e0e476c5095a6b0626b295074a",
                )
                .unwrap(),
                1,
            ),
            &test_value,
        );

        tx_builder.set_inputs(&tx_inputs_builder);

        tx_builder
            .add_change_if_needed(
                &csl::Address::from_bech32(
                    "addr_test1vpvwjd8za9wj8kzse2pm9ch9xcd9zu6aex99jyd6rrgntdqq2vvut",
                )
                .unwrap(),
            )
            .unwrap();

        assert!(tx_builder.build().unwrap().outputs().len() == 2);
    }
}
