#[cfg(test)]
mod credentials {
    use whisky_common::data::*;

    #[test]
    fn test_payment_pub_key_hash() {
        let correct_payment_pub_key_hash = "{\"bytes\":\"hello\"}";
        assert_eq!(
            payment_pub_key_hash("hello").to_string(),
            correct_payment_pub_key_hash
        );
    }

    #[test]
    fn test_pub_key_hash() {
        let correct_pub_key_hash = "{\"bytes\":\"hello\"}";
        assert_eq!(pub_key_hash("hello").to_string(), correct_pub_key_hash);
    }

    #[test]
    fn test_maybe_staking_hash() {
        let correct_maybe_staking_hash = "{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"}]}]}]}";
        assert_eq!(
            maybe_staking_hash("hello", false).to_string(),
            correct_maybe_staking_hash
        );
    }

    #[test]
    fn test_pub_key_address() {
        let correct_pub_key_address = "{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"bytes\":\"8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73\"}]},{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"bytes\":\"039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f\"}]}]}]}]}";
        assert_eq!(
            pub_key_address(
                "8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73",
                Some("039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f"),
                false
            )
            .to_string(),
            correct_pub_key_address
        );
    }

    #[test]
    fn test_script_address() {
        let correct_script_address = "{\"constructor\":0,\"fields\":[{\"constructor\":1,\"fields\":[{\"bytes\":\"hello\"}]},{\"constructor\":1,\"fields\":[]}]}";
        assert_eq!(
            script_address("hello", None, false).to_string(),
            correct_script_address
        );
    }

    #[test]
    fn test_script_address_script_stake_key() {
        let correct_script_address = "{\"constructor\":0,\"fields\":[{\"constructor\":1,\"fields\":[{\"bytes\":\"8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73\"}]},{\"constructor\":0,\"fields\":[{\"constructor\":0,\"fields\":[{\"constructor\":1,\"fields\":[{\"bytes\":\"039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f\"}]}]}]}]}";
        assert_eq!(
            script_address(
                "8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73",
                Some("039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f"),
                true
            )
            .to_string(),
            correct_script_address
        );
    }
}
