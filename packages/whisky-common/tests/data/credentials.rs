#[cfg(test)]
mod tests {
    use whisky_common::data::*;

    #[test]
    fn test_verification_key() {
        let correct_vkey = "{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(
            constr0(vec![byte_string("hello")]).to_string(),
            correct_vkey
        );
        assert_eq!(
            VerificationKey::from("hello").to_json_string(),
            correct_vkey
        );
    }

    #[test]
    fn test_script() {
        let correct_script = "{\"constructor\":1,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(
            constr1(vec![byte_string("hello")]).to_string(),
            correct_script
        );
        assert_eq!(Script::from("hello").to_json_string(), correct_script);
    }

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
        assert_eq!(
            Address::new(
                "8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73",
                Some("039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f"),
                false,
                false
            )
            .to_json_string(),
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
        assert_eq!(
            Address::new(
                "8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73",
                Some("039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f"),
                true,
                true
            )
            .to_json_string(),
            correct_script_address
        );
    }

    // ==================== from_json round-trip tests ====================

    #[test]
    fn test_verification_key_from_json_roundtrip() {
        let original = VerificationKey::from("abcd1234");
        let json_str = original.to_json_string();
        let parsed = VerificationKey::from_json_string(&json_str).unwrap();
        assert_eq!(original.0.fields.bytes, parsed.0.fields.bytes);
    }

    #[test]
    fn test_script_from_json_roundtrip() {
        let original = Script::from("deadbeef");
        let json_str = original.to_json_string();
        let parsed = Script::from_json_string(&json_str).unwrap();
        assert_eq!(original.0.fields.bytes, parsed.0.fields.bytes);
    }

    #[test]
    fn test_credential_verification_key_from_json_roundtrip() {
        let original = Credential::new(("abcd1234", false));
        let json_str = original.to_json_string();
        let parsed = Credential::from_json_string(&json_str).unwrap();
        match (original, parsed) {
            (Credential::VerificationKey(orig), Credential::VerificationKey(pars)) => {
                assert_eq!(orig.0.fields.bytes, pars.0.fields.bytes);
            }
            _ => panic!("Expected VerificationKey variant"),
        }
    }

    #[test]
    fn test_credential_script_from_json_roundtrip() {
        let original = Credential::new(("deadbeef", true));
        let json_str = original.to_json_string();
        let parsed = Credential::from_json_string(&json_str).unwrap();
        match (original, parsed) {
            (Credential::Script(orig), Credential::Script(pars)) => {
                assert_eq!(orig.0.fields.bytes, pars.0.fields.bytes);
            }
            _ => panic!("Expected Script variant"),
        }
    }

    #[test]
    fn test_address_pub_key_with_stake_from_json_roundtrip() {
        let original = Address::new(
            "8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73",
            Some("039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f"),
            false,
            false,
        );
        let json_str = original.to_json_string();
        let parsed = Address::from_json_string(&json_str).unwrap();
        assert_eq!(original.payment_key_hash, parsed.payment_key_hash);
        assert_eq!(original.stake_credential, parsed.stake_credential);
        assert_eq!(original.is_script_payment_key, parsed.is_script_payment_key);
        assert_eq!(original.is_script_stake_key, parsed.is_script_stake_key);
    }

    #[test]
    fn test_address_script_with_script_stake_from_json_roundtrip() {
        let original = Address::new(
            "8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73",
            Some("039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f"),
            true,
            true,
        );
        let json_str = original.to_json_string();
        let parsed = Address::from_json_string(&json_str).unwrap();
        assert_eq!(original.payment_key_hash, parsed.payment_key_hash);
        assert_eq!(original.stake_credential, parsed.stake_credential);
        assert_eq!(original.is_script_payment_key, parsed.is_script_payment_key);
        assert_eq!(original.is_script_stake_key, parsed.is_script_stake_key);
    }

    #[test]
    fn test_address_script_no_stake_from_json_roundtrip() {
        let original = Address::new(
            "hello",
            None,
            true,
            false,
        );
        let json_str = original.to_json_string();
        let parsed = Address::from_json_string(&json_str).unwrap();
        assert_eq!(original.payment_key_hash, parsed.payment_key_hash);
        assert_eq!(original.stake_credential, parsed.stake_credential);
        assert_eq!(original.is_script_payment_key, parsed.is_script_payment_key);
    }

    #[test]
    fn test_address_pub_key_no_stake_from_json_roundtrip() {
        let original = Address::new(
            "abcdef123456",
            None,
            false,
            false,
        );
        let json_str = original.to_json_string();
        let parsed = Address::from_json_string(&json_str).unwrap();
        assert_eq!(original.payment_key_hash, parsed.payment_key_hash);
        assert_eq!(original.stake_credential, parsed.stake_credential);
        assert_eq!(original.is_script_payment_key, parsed.is_script_payment_key);
    }

    #[test]
    fn test_credential_from_json_invalid_tag_error() {
        // Constructor tag 2 is invalid for Credential
        let invalid_json = r#"{"constructor": 2, "fields": [{"bytes": "test"}]}"#;
        let result = Credential::from_json_string(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_address_from_json_invalid_tag_error() {
        // Address must have constructor tag 0
        let invalid_json = r#"{"constructor": 1, "fields": []}"#;
        let result = Address::from_json_string(invalid_json);
        assert!(result.is_err());
    }
}
