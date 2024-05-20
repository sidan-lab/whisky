mod parser_tests {
    use serde_json::json;
    use sidan_csl_rs::core::common::*;

    #[test]
    fn test_con_str() {
        let correct_con_str = "{\"constructor\":10,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(
            con_str(10, json!([builtin_byte_string("hello")])).to_string(),
            correct_con_str
        );
    }

    #[test]
    fn test_con_str0() {
        let correct_con_str0 = "{\"constructor\":0,\"fields\":{\"bytes\":\"hello\"}}";
        assert_eq!(
            con_str0(builtin_byte_string("hello")).to_string(),
            correct_con_str0
        );
    }

    #[test]
    fn test_con_str1() {
        let correct_con_str1 = "{\"constructor\":1,\"fields\":{\"bytes\":\"hello\"}}";
        assert_eq!(
            con_str1(builtin_byte_string("hello")).to_string(),
            correct_con_str1
        );
    }

    #[test]
    fn test_con_str2() {
        let correct_con_str2 = "{\"constructor\":2,\"fields\":{\"bytes\":\"hello\"}}";
        assert_eq!(
            con_str2(builtin_byte_string("hello")).to_string(),
            correct_con_str2
        );
    }

    #[test]
    fn test_bool() {
        let correct_bool = "{\"constructor\":1,\"fields\":[]}";
        assert_eq!(bool(true).to_string(), correct_bool);
    }

    #[test]
    fn test_builtin_byte_string() {
        let correct_builtin_byte_string = "{\"bytes\":\"hello\"}";
        assert_eq!(
            builtin_byte_string("hello").to_string(),
            correct_builtin_byte_string
        );
    }

    #[test]
    fn test_integer() {
        let correct_integer = "{\"int\":1}";
        assert_eq!(integer(1).to_string(), correct_integer);
    }

    #[test]
    fn test_list() {
        let correct_list = "{\"list\":[1,2,3]}";
        assert_eq!(list(vec![1, 2, 3]).to_string(), correct_list);
    }

    #[test]
    fn test_maybe_staking_hash() {
        let correct_maybe_staking_hash = "{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"}]}]}]}";
        assert_eq!(
            maybe_staking_hash("hello").to_string(),
            correct_maybe_staking_hash
        );
    }

    #[test]
    fn test_pub_key_address() {
        let correct_pub_key_address = "{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"bytes\":\"8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73\"}]},{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"bytes\":\"039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f\"}]}]}]}]}";
        assert_eq!(
            pub_key_address(
                "8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73",
                Some("039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f")
            )
            .to_string(),
            correct_pub_key_address
        );
    }

    #[test]
    fn test_script_address() {
        let correct_script_address = "{\"constructor\":0,\"fields\":[{\"constructor\":1,\"fields\":[{\"bytes\":\"hello\"}]},{\"constructor\":1,\"fields\":[]}]}";
        assert_eq!(
            script_address("hello", None).to_string(),
            correct_script_address
        );
    }

    #[test]
    fn test_asset_class() {
        let correct_asset_class =
            "{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"},{\"bytes\":\"world\"}]}";
        assert_eq!(
            asset_class("hello", "world").to_string(),
            correct_asset_class
        );
    }

    #[test]
    fn test_tx_out_ref() {
        let correct_tx_out_ref = "{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"}]},{\"int\":12}]}";
        assert_eq!(tx_out_ref("hello", 12).to_string(), correct_tx_out_ref);
    }

    #[test]
    fn test_assoc_map() {
        let correct_assoc_map =
      "{\"map\":[{\"k\":{\"bytes\":\"hello\"},\"v\":{\"bytes\":\"world\"}},{\"k\":{\"bytes\":\"123\"},\"v\":{\"bytes\":\"456\"}}]}";
        assert_eq!(
            assoc_map(vec![
                (builtin_byte_string("hello"), builtin_byte_string("world")),
                (builtin_byte_string("123"), builtin_byte_string("456"))
            ])
            .to_string(),
            correct_assoc_map
        );
    }

    #[test]
    fn test_tuple() {
        let correct_tuple =
            "{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"},{\"bytes\":\"world\"}]}";
        assert_eq!(
            tuple(builtin_byte_string("hello"), builtin_byte_string("world")).to_string(),
            correct_tuple
        );
    }
}
