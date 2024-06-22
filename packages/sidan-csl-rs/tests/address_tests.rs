mod address_tests {
    use sidan_csl_rs::{
        core::utils::{script_to_address, serialize_bech32_address},
        model::SerializedAddress,
    };

    #[test]
    fn test_script_to_address() {
        let addr = script_to_address(
            0,
            "ac43e2abd0909c559966056aaa35d2172717174e224feb81e34c306a".to_string(),
            None,
        );
        assert!(addr == "addr_test1wzky8c4t6zgfc4vevczk42346gtjw9chfc3yl6upudxrq6sghz0nw");

        let base_addr = script_to_address(
            0,
            "e55a6e7c9f4e96692a3c23a56f126911cc70a29d2e2ac967dc644432".to_string(),
            Some((
                "6d913965402b012050e09f12012c533e6c33678d1c5ed2154b328d25".to_string(),
                false,
            )),
        );

        assert!(base_addr == "addr_test1zrj45mnuna8fv6f28s362mcjdygucu9zn5hz4jt8m3jygvndjyuk2sptqys9pcylzgqjc5e7dsek0rgutmfp2jej35jseqau4y");
    }

    #[test]
    fn test_script_to_address_script_stake_key() {
        let base_addr = script_to_address(
            0,
            "c12e891c8e995cfa5d1547ace30413cad298827a19fbb8ea49b46469".to_string(),
            Some((
                "867c8b572e5ac8f0c14aa7417cb9caec9d1ff50e994f772eab2d69f4".to_string(),
                true,
            )),
        );

        assert!(base_addr == "addr_test1xrqjazgu36v4e7jaz4r6eccyz09d9xyz0gvlhw82fx6xg6vx0j94wtj6ercvzj48g97tnjhvn50l2r5efamja2edd86ql04h5v");
    }

    #[test]
    fn test_serialize_address() {
        let addr1 = "addr_test1qz8j439j54afpl4hw978xcw8qsa0dsmyd6wm9v8xzeyz7ucrj5rt3et7z59mvmmpxnejvn2scwmseezdq5h5fpw08z8s8d93my";
        let addr1_result = serialize_bech32_address(addr1.to_string());
        assert!(
            addr1_result
                == SerializedAddress::new(
                    "8f2ac4b2a57a90feb7717c7361c7043af6c3646e9db2b0e616482f73".to_string(),
                    "".to_string(),
                    "039506b8e57e150bb66f6134f3264d50c3b70ce44d052f4485cf388f".to_string(),
                    "".to_string(),
                )
        );

        let addr2 = "addr_test1zqjmsmh2sjjy508e3068pck6lgp23k2msypgc52cxcgzjlju5ayjvx4rk9a29n2tqf4uv4nvfv2yy8tqs0kuue8luh9s5cdt49";
        let addr2_result = serialize_bech32_address(addr2.to_string());
        assert!(
            addr2_result
                == SerializedAddress::new(
                    "".to_string(),
                    "25b86eea84a44a3cf98bf470e2dafa02a8d95b81028c51583610297e".to_string(),
                    "5ca749261aa3b17aa2cd4b026bc6566c4b14421d6083edce64ffe5cb".to_string(),
                    "".to_string(),
                )
        );

        let addr3 = "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x";
        let addr3_result = serialize_bech32_address(addr3.to_string());
        assert!(
            addr3_result
                == SerializedAddress::new(
                    "5ca51b304b1f79d92eada8c58c513e969458dcd27ce4f5bc47823ffa".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string()
                )
        );

        let addr4 = "addr_test1qqmrzjhtanauj20wg37uk58adyrqfm82a9qr52vdnv0e54r42v0mu8ngky0f5yxmh3wl3z0da2fryk59kavth0u8xhvsufgmc8";
        let addr4_result = serialize_bech32_address(addr4.to_string());
        assert!(
            addr4_result
                == SerializedAddress::new(
                    "36314aebecfbc929ee447dcb50fd690604eceae9403a298d9b1f9a54".to_string(),
                    "".to_string(),
                    "75531fbe1e68b11e9a10dbbc5df889edea92325a85b758bbbf8735d9".to_string(),
                    "".to_string()
                )
        );

        let addr5 = "addr_test1xrqjazgu36v4e7jaz4r6eccyz09d9xyz0gvlhw82fx6xg6vx0j94wtj6ercvzj48g97tnjhvn50l2r5efamja2edd86ql04h5v";
        let addr5_result = serialize_bech32_address(addr5.to_string());
        assert!(
            addr5_result
                == SerializedAddress::new(
                    "".to_string(),
                    "c12e891c8e995cfa5d1547ace30413cad298827a19fbb8ea49b46469".to_string(),
                    "".to_string(),
                    "867c8b572e5ac8f0c14aa7417cb9caec9d1ff50e994f772eab2d69f4".to_string()
                )
        );
    }
}
