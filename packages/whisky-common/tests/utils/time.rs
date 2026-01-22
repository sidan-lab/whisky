#[cfg(test)]
mod tests {
    use whisky_common::{
        utils::{
            get_slot_config, resolve_epoch_no, resolve_slot_no, slot_to_begin_unix_time,
            unix_time_to_enclosing_slot, SlotConfig,
        },
        Network,
    };

    #[test]
    fn test_mainnet_slot_config() {
        let config = SlotConfig::mainnet();
        assert_eq!(config.zero_time, 1596059091000);
        assert_eq!(config.zero_slot, 4492800);
        assert_eq!(config.slot_length, 1000);
        assert_eq!(config.start_epoch, 208);
        assert_eq!(config.epoch_length, 432000);
    }

    #[test]
    fn test_preview_slot_config() {
        let config = SlotConfig::preview();
        assert_eq!(config.zero_time, 1666656000000);
        assert_eq!(config.zero_slot, 0);
        assert_eq!(config.slot_length, 1000);
        assert_eq!(config.start_epoch, 0);
        assert_eq!(config.epoch_length, 86400);
    }

    #[test]
    fn test_preprod_slot_config() {
        let config = SlotConfig::preprod();
        assert_eq!(config.zero_time, 1654041600000 + 1728000000);
        assert_eq!(config.zero_slot, 86400);
        assert_eq!(config.slot_length, 1000);
        assert_eq!(config.start_epoch, 4);
        assert_eq!(config.epoch_length, 432000);
    }

    #[test]
    fn test_slot_to_begin_unix_time_mainnet() {
        let config = SlotConfig::mainnet();
        // At zero_slot, time should be zero_time
        assert_eq!(slot_to_begin_unix_time(4492800, &config), 1596059091000);
        // 1000 slots later (1000 seconds = 1000000 ms later)
        assert_eq!(slot_to_begin_unix_time(4493800, &config), 1596060091000);
    }

    #[test]
    fn test_unix_time_to_enclosing_slot_mainnet() {
        let config = SlotConfig::mainnet();
        // At zero_time, slot should be zero_slot
        assert_eq!(unix_time_to_enclosing_slot(1596059091000, &config), 4492800);
        // 1000000 ms later (1000 seconds = 1000 slots)
        assert_eq!(unix_time_to_enclosing_slot(1596060091000, &config), 4493800);
        // 1000000 ms later (1000 seconds = 1000 slots)
    }

    #[test]
    fn test_slot_time_roundtrip() {
        let config = SlotConfig::mainnet();
        let original_slot = 50000000u64;
        let time = slot_to_begin_unix_time(original_slot, &config);
        let recovered_slot = unix_time_to_enclosing_slot(time, &config);
        assert_eq!(original_slot, recovered_slot);
    }

    #[test]
    fn test_resolve_slot_no_mainnet() {
        // Test with a known timestamp
        let slot = resolve_slot_no(&Network::Mainnet, Some(1596059091000));
        assert_eq!(slot, Some("4492800".to_string()));
    }

    #[test]
    fn test_resolve_epoch_no_mainnet() {
        // At zero_time, epoch should be start_epoch (208)
        let epoch = resolve_epoch_no(&Network::Mainnet, Some(1596059091000));
        assert_eq!(epoch, Some(208));
    }

    #[test]
    fn test_resolve_epoch_no_preview() {
        // At zero_time, epoch should be start_epoch (0)
        let epoch = resolve_epoch_no(&Network::Preview, Some(1666656000000));
        assert_eq!(epoch, Some(0));
    }

    #[test]
    fn test_get_slot_config_custom_returns_none() {
        let custom_network = Network::Custom(vec![]);
        assert!(get_slot_config(&custom_network).is_none());
        assert!(resolve_slot_no(&custom_network, None).is_none());
        assert!(resolve_epoch_no(&custom_network, None).is_none());
    }

    // ==========================================
    // Mesh compatibility tests from time.test.ts
    // ==========================================

    mod resolve_slot_no_tests {
        use super::*;

        #[test]
        fn should_resolve_correct_mainnet_slot_number() {
            // Aug 07 2024 11:56:59 GMT+0800
            let res = resolve_slot_no(&Network::Mainnet, Some(1723003026421));
            assert_eq!(res, Some("131436735".to_string()));
        }

        #[test]
        fn should_resolve_correct_preprod_slot_number() {
            // Aug 07 2024 11:56:59 GMT+0800
            let res = resolve_slot_no(&Network::Preprod, Some(1723003026421));
            assert_eq!(res, Some("67319826".to_string()));
        }

        #[test]
        fn should_resolve_correct_preview_slot_number() {
            // Aug 07 2024 11:56:59 GMT+0800
            let res = resolve_slot_no(&Network::Preview, Some(1723003026421));
            assert_eq!(res, Some("56347026".to_string()));
        }
    }

    mod resolve_epoch_no_mainnet_tests {
        use super::*;

        #[test]
        fn should_resolve_correct_mainnet_epoch_number() {
            // Aug 07 2024 11:42:47 GMT+0800
            let res = resolve_epoch_no(&Network::Mainnet, Some(1723000771631));
            assert_eq!(res, Some(501));
        }

        #[test]
        fn should_resolve_correct_mainnet_epoch_number_may() {
            // May 27, 2024 05:44:51 GMT+0800
            let res = resolve_epoch_no(&Network::Mainnet, Some(1716759891000));
            assert_eq!(res, Some(487));
        }

        #[test]
        fn should_resolve_correct_epoch_number_at_lower_bound() {
            // Aug 3, 2024 05:44:51 AM GMT+0800
            let res = resolve_epoch_no(&Network::Mainnet, Some(1722663891000));
            assert_eq!(res, Some(501));
        }

        #[test]
        fn should_resolve_correct_epoch_number_at_upper_bound() {
            // Aug 8, 2024 05:44:51 AM GMT+0800 - 1ms
            let res = resolve_epoch_no(&Network::Mainnet, Some(1723067091000 - 1));
            assert_eq!(res, Some(501));
        }
    }

    mod resolve_epoch_no_preprod_tests {
        use super::*;

        #[test]
        fn should_resolve_correct_preprod_epoch_number() {
            // Aug 07 2024 11:42:47 GMT+0800
            let res = resolve_epoch_no(&Network::Preprod, Some(1723000771631));
            assert_eq!(res, Some(159));
        }

        #[test]
        fn should_resolve_correct_preprod_epoch_number_june() {
            // Jun 22, 2024 08:00:00 GMT+0800
            let res = resolve_epoch_no(&Network::Preprod, Some(1719014400000));
            assert_eq!(res, Some(150));
        }

        #[test]
        fn should_resolve_correct_epoch_number_at_lower_bound() {
            // Aug 4, 2024 08:00:00 GMT+0800
            let res = resolve_epoch_no(&Network::Preprod, Some(1722729600000));
            assert_eq!(res, Some(159));
        }

        #[test]
        fn should_resolve_correct_epoch_number_at_upper_bound() {
            // Aug 9, 2024 08:00:00 GMT+0800 - 1ms
            let res = resolve_epoch_no(&Network::Preprod, Some(1723161600000 - 1));
            assert_eq!(res, Some(159));
        }
    }

    mod resolve_epoch_no_preview_tests {
        use super::*;

        #[test]
        fn should_resolve_correct_preview_epoch_number() {
            // Aug 07 2024 11:42:47 GMT+0800
            let res = resolve_epoch_no(&Network::Preview, Some(1723000771631));
            assert_eq!(res, Some(652));
        }

        #[test]
        fn should_resolve_correct_preview_epoch_number_july() {
            // Jul 26, 2024 18:00:00 GMT+0800
            let res = resolve_epoch_no(&Network::Preview, Some(1721988000000));
            assert_eq!(res, Some(640));
        }

        #[test]
        fn should_resolve_correct_epoch_number_at_lower_bound() {
            // Aug 7, 2024 08:00:00 GMT+0800
            let res = resolve_epoch_no(&Network::Preview, Some(1722988800000));
            assert_eq!(res, Some(652));
        }

        #[test]
        fn should_resolve_correct_epoch_number_at_upper_bound() {
            // Aug 8, 2024 08:00:00 GMT+0800 - 1ms
            let res = resolve_epoch_no(&Network::Preview, Some(1723075200000 - 1));
            assert_eq!(res, Some(652));
        }
    }
}
