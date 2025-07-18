#[cfg(test)]
mod tests {
    use whisky_common::{
        data::{ByteString, Int, Map, PlutusDataJson, Value},
        *,
    };
    // Operator tests
    #[test]
    fn test_add_asset() {
        let mut assets = Value::new();
        assets.0.insert("lovelace".to_string(), 50);
        assets.0.insert("asset1".to_string(), 60);
        assets.add_asset("lovelace", 100);
        assert_eq!(assets.0.get("lovelace").unwrap(), &150);
        assert_eq!(assets.0.get("asset1").unwrap(), &60);
    }

    #[test]
    fn test_add_assets() {
        let mut assets = Value::new();
        let other = vec![
            Asset::new_from_str("lovelace", "50"),
            Asset::new_from_str("asset1", "60"),
        ];
        assets.add_assets(&other);
        assert_eq!(assets.0.get("lovelace").unwrap(), &50);
        assert_eq!(assets.0.get("asset1").unwrap(), &60);
    }

    #[test]
    fn test_merge_assets() {
        let mut assets = Value::new();
        let mut other = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        other.0.insert("lovelace".to_string(), 100);
        assets.merge(&other);
        assert_eq!(assets.0.get("lovelace").unwrap(), &200);
    }

    #[test]
    fn test_merge_multiple_assets() {
        let mut assets = Value::new();
        let mut other = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        other.0.insert("lovelace".to_string(), 100);
        assets.0.insert("asset1".to_string(), 100);
        other.0.insert("asset2".to_string(), 50);
        assets.merge(&other);
        assert_eq!(assets.0.get("lovelace").unwrap(), &200);
        assert_eq!(assets.0.get("asset1").unwrap(), &100);
        assert_eq!(assets.0.get("asset2").unwrap(), &50);
    }

    #[test]
    fn test_negate_asset() {
        let mut assets = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        assets.negate_asset("lovelace", 65);
        assert_eq!(assets.0.get("lovelace").unwrap(), &35);
    }

    #[test]
    fn test_negate_asset_to_zero() {
        let mut assets = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        assets.negate_asset("lovelace", 101);
        assert_eq!(assets.0.get("lovelace"), None);
    }

    #[test]
    fn test_negate_value() {
        let mut assets = Value::new();
        let mut other = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        other.0.insert("lovelace".to_string(), 65);
        assets.negate_value(&other);
        assert_eq!(assets.0.get("lovelace").unwrap(), &35);
    }

    #[test]
    fn test_negate_assets() {
        let mut assets = Value::new();
        let other = vec![Asset::new_from_str("lovelace", "65")];
        assets.0.insert("lovelace".to_string(), 100);
        assets.negate_assets(&other);
        assert_eq!(assets.0.get("lovelace").unwrap(), &35);
    }

    #[test]
    fn test_negate_value_to_zero() {
        let mut assets = Value::new();
        let mut other = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        other.0.insert("lovelace".to_string(), 101);
        assets.negate_value(&other);
        assert_eq!(assets.0.get("lovelace"), None);
    }

    #[test]
    fn test_negate_assets_to_zero() {
        let mut assets = Value::new();
        let other = vec![Asset::new_from_str("lovelace", "101")];
        assets.0.insert("lovelace".to_string(), 100);
        assets.negate_assets(&other);
        assert_eq!(assets.0.get("lovelace"), None);
    }

    #[test]
    fn test_negate_value_multiple() {
        let mut assets = Value::new();
        let mut other = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        other.0.insert("lovelace".to_string(), 65);
        assets.0.insert("asset1".to_string(), 100);
        other.0.insert("asset2".to_string(), 50);
        assets.negate_value(&other);
        assert_eq!(assets.0.get("lovelace").unwrap(), &35);
        assert_eq!(assets.0.get("asset1").unwrap(), &100);
        assert_eq!(assets.0.get("asset2"), None);
    }

    #[test]
    fn test_negate_assets_multiple() {
        let mut assets = Value::new();
        let other = vec![
            Asset::new_from_str("lovelace", "65"),
            Asset::new_from_str("asset2", "50"),
        ];
        assets.0.insert("lovelace".to_string(), 100);
        assets.0.insert("asset1".to_string(), 100);
        assets.negate_assets(&other);
        assert_eq!(assets.0.get("lovelace").unwrap(), &35);
        assert_eq!(assets.0.get("asset1").unwrap(), &100);
        assert_eq!(assets.0.get("asset2"), None);
    }

    // Accessor tests
    #[test]
    fn test_get() {
        let mut assets = Value::new();
        assets.0.insert("lovelace".to_string(), 100);
        assert_eq!(assets.get("lovelace"), 100);
    }

    // Comparison function tests
    #[test]
    fn test_geq() {
        let mut first_assets = Value::new();
        first_assets
            .add_asset("lovelace", 1_012_760)
            .add_asset("asset1", 100);

        let mut second_assets = Value::new();
        second_assets
            .add_asset("lovelace", 1_000_000)
            .add_asset("asset1", 100);

        assert!(first_assets.geq(&second_assets));
    }

    #[test]
    fn test_leq() {
        let mut first_assets = Value::new();
        first_assets
            .add_asset("lovelace", 1_000_000)
            .add_asset("asset1", 100);

        let mut second_assets = Value::new();
        second_assets
            .add_asset("lovelace", 1_012_760)
            .add_asset("asset1", 100);

        assert!(first_assets.leq(&second_assets));
    }

    #[test]
    fn test_is_empty() {
        let assets = Value::new();
        assert!(assets.is_empty());
    }

    #[test]
    fn test_to_json() {
        let assets = Value::new();
        let val_map: Map<(), ()> = Map::new(&[]);
        assert_eq!(assets.to_json_string(), val_map.to_json_string());
    }

    #[test]
    fn test_simple_token_value() {
        let val: Vec<Asset> = vec![Asset::new_from_str(
            "baefdc6c5b191be372a794cd8d40d839ec0dbdd3c28957267dc8170074657374696e676e657777616c2e616461",
            "345",
        )];
        let datum: Value = Value::from_asset_vec(&val);
        let name_map = Map::new(&[(
            ByteString::new("74657374696e676e657777616c2e616461"),
            Int::new(345),
        )]);
        let val_map = Map::new(&[(
            ByteString::new("baefdc6c5b191be372a794cd8d40d839ec0dbdd3c28957267dc81700"),
            name_map,
        )]);
        assert_eq!(datum.to_json_string(), val_map.to_json_string());
    }

    #[test]
    fn test_to_json_complex_value() {
        let val: Vec<Asset> = vec![
            Asset::new_from_str("lovelace", "1000000"),
            Asset::new_from_str(
                "baefdc6c5b191be372a794cd8d40d839ec0dbdd3c28957267dc8170074657374696e676e657777616c2e616461",
                "345",
            ),
            Asset::new_from_str("baefdc6c5b191be372a794cd8d40d839ec0dbdd3c28957267dc817001234", "567"),
        ];
        let datum: Value = Value::from_asset_vec(&val);

        let name_map = Map::new(&[
            (ByteString::new("1234"), Int::new(567)),
            (
                ByteString::new("74657374696e676e657777616c2e616461"),
                Int::new(345),
            ),
        ]);
        let val_map = Map::new(&[
            (
                ByteString::new(""),
                Map::new(&[(ByteString::new(""), Int::new(1000000))]),
            ),
            (
                ByteString::new("baefdc6c5b191be372a794cd8d40d839ec0dbdd3c28957267dc81700"),
                name_map,
            ),
        ]);
        assert_eq!(datum.to_json_string(), val_map.to_json_string());
    }
}
