#[cfg(test)]
mod tests {
    use whisky_common::data::*;

    #[test]
    fn test_currency_symbol() {
        let correct_currency_symbol = "{\"bytes\":\"hello\"}";
        assert_eq!(
            currency_symbol("hello").to_string(),
            correct_currency_symbol
        );
    }

    #[test]
    fn test_token_name() {
        let correct_token_name = "{\"bytes\":\"hello\"}";
        assert_eq!(token_name("hello").to_string(), correct_token_name);
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
    fn test_posix_time() {
        let correct_output_reference = "{\"int\":12}";
        assert_eq!(posix_time(12).to_string(), correct_output_reference);
    }

    #[test]
    fn test_single_element_tuple() {
        let byte_string = ByteString::new("test");
        let single_tuple = (byte_string,);
        let json = single_tuple.to_json();
        assert_eq!(json.to_string(), "[{\"bytes\":\"test\"}]");
    }

    #[test]
    fn test_two_element_tuple() {
        let byte_string1 = ByteString::new("hello");
        let byte_string2 = ByteString::new("world");
        let tuple = (byte_string1, byte_string2);
        let json = tuple.to_json();
        assert_eq!(
            json.to_string(),
            "[{\"bytes\":\"hello\"},{\"bytes\":\"world\"}]"
        );
    }

    // Round-trip tests for from_json / from_json_string

    #[test]
    fn test_int_from_json_roundtrip() {
        let original = Int::new(42);
        let json_str = original.to_json_string();
        let parsed = Int::from_json_string(&json_str).unwrap();
        assert_eq!(original.int, parsed.int);
    }

    #[test]
    fn test_int_from_json_negative() {
        let original = Int::new(-12345);
        let json_str = original.to_json_string();
        let parsed = Int::from_json_string(&json_str).unwrap();
        assert_eq!(original.int, parsed.int);
    }

    #[test]
    fn test_byte_string_from_json_roundtrip() {
        let original = ByteString::new("deadbeef");
        let json_str = original.to_json_string();
        let parsed = ByteString::from_json_string(&json_str).unwrap();
        assert_eq!(original.bytes, parsed.bytes);
    }

    #[test]
    fn test_bool_from_json_roundtrip() {
        let true_val = Bool::new(true);
        let false_val = Bool::new(false);

        let parsed_true = Bool::from_json_string(&true_val.to_json_string()).unwrap();
        let parsed_false = Bool::from_json_string(&false_val.to_json_string()).unwrap();

        assert_eq!(parsed_true, Bool::True);
        assert_eq!(parsed_false, Bool::False);
    }

    #[test]
    fn test_list_from_json_roundtrip() {
        let original = List::new(&[Int::new(1), Int::new(2), Int::new(3)]);
        let json_str = original.to_json_string();
        let parsed = List::<Int>::from_json_string(&json_str).unwrap();
        assert_eq!(original.items.len(), parsed.items.len());
        for (a, b) in original.items.iter().zip(parsed.items.iter()) {
            assert_eq!(a.int, b.int);
        }
    }

    #[test]
    fn test_map_from_json_roundtrip() {
        let original = Map::new(&[
            (ByteString::new("key1"), Int::new(100)),
            (ByteString::new("key2"), Int::new(200)),
        ]);
        let json_str = original.to_json_string();
        let parsed = Map::<ByteString, Int>::from_json_string(&json_str).unwrap();
        assert_eq!(original.map.len(), parsed.map.len());
        for ((k1, v1), (k2, v2)) in original.map.iter().zip(parsed.map.iter()) {
            assert_eq!(k1.bytes, k2.bytes);
            assert_eq!(v1.int, v2.int);
        }
    }

    #[test]
    fn test_constr0_from_json_roundtrip() {
        let original = Constr0::new(ByteString::new("test"));
        let json_str = original.to_json_string();
        let parsed = Constr0::<ByteString>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields.bytes, parsed.fields.bytes);
    }

    #[test]
    fn test_constr1_from_json_roundtrip() {
        let original = Constr1::new(Int::new(999));
        let json_str = original.to_json_string();
        let parsed = Constr1::<Int>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields.int, parsed.fields.int);
    }

    #[test]
    fn test_tuple_from_json_roundtrip() {
        let original = (ByteString::new("hello"), Int::new(42));
        let json_str = original.to_json_string();
        let parsed = <(ByteString, Int)>::from_json_string(&json_str).unwrap();
        assert_eq!(original.0.bytes, parsed.0.bytes);
        assert_eq!(original.1.int, parsed.1.int);
    }

    #[test]
    fn test_nested_structure_from_json_roundtrip() {
        // List of Constr0<ByteString>
        let original = List::new(&[
            Constr0::new(ByteString::new("first")),
            Constr0::new(ByteString::new("second")),
        ]);
        let json_str = original.to_json_string();
        let parsed = List::<Constr0<ByteString>>::from_json_string(&json_str).unwrap();
        assert_eq!(original.items.len(), parsed.items.len());
        assert_eq!(original.items[0].fields.bytes, parsed.items[0].fields.bytes);
        assert_eq!(original.items[1].fields.bytes, parsed.items[1].fields.bytes);
    }

    #[test]
    fn test_value_from_json_roundtrip() {
        let mut original = Value::new();
        original.add_asset("lovelace", 5000000);
        original.add_asset("abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234token1", 100);

        let json_str = original.to_json_string();
        let parsed = Value::from_json_string(&json_str).unwrap();

        assert_eq!(original.get("lovelace"), parsed.get("lovelace"));
        assert_eq!(
            original.get("abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234token1"),
            parsed.get("abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234abcd1234token1")
        );
    }

    #[test]
    fn test_from_json_error_missing_field() {
        let invalid_json = r#"{"wrong": 42}"#;
        let result = Int::from_json_string(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_json_error_invalid_constructor() {
        let invalid_json = r#"{"constructor": 5, "fields": []}"#;
        let result = Bool::from_json_string(invalid_json);
        assert!(result.is_err());
    }

    // Box<T> variant tests
    #[test]
    fn test_box_int_from_json_roundtrip() {
        let original: Box<Int> = Box::new(Int::new(999));
        let json_str = original.to_json_string();
        let parsed = Box::<Int>::from_json_string(&json_str).unwrap();
        assert_eq!(original.int, parsed.int);
    }

    #[test]
    fn test_box_bytestring_from_json_roundtrip() {
        let original: Box<ByteString> = Box::new(ByteString::new("boxed_bytes"));
        let json_str = original.to_json_string();
        let parsed = Box::<ByteString>::from_json_string(&json_str).unwrap();
        assert_eq!(original.bytes, parsed.bytes);
    }

    #[test]
    fn test_box_bool_from_json_roundtrip() {
        let original: Box<Bool> = Box::new(Bool::new(true));
        let json_str = original.to_json_string();
        let parsed = Box::<Bool>::from_json_string(&json_str).unwrap();
        assert_eq!(*original, *parsed);
    }

    #[test]
    fn test_box_list_from_json_roundtrip() {
        let original: Box<List<Int>> = Box::new(List::new(&[Int::new(1), Int::new(2), Int::new(3)]));
        let json_str = original.to_json_string();
        let parsed = Box::<List<Int>>::from_json_string(&json_str).unwrap();
        assert_eq!(original.items.len(), parsed.items.len());
        for (a, b) in original.items.iter().zip(parsed.items.iter()) {
            assert_eq!(a.int, b.int);
        }
    }

    #[test]
    fn test_box_plutus_data_from_json_roundtrip() {
        let original: Box<PlutusData> = Box::new(PlutusData::Integer(Int::new(42)));
        let json_str = original.to_json_string();
        let parsed = Box::<PlutusData>::from_json_string(&json_str).unwrap();
        match parsed.as_ref() {
            PlutusData::Integer(i) => assert_eq!(i.int, 42),
            _ => panic!("Expected Integer variant"),
        }
    }

    #[test]
    fn test_box_tuple_from_json_roundtrip() {
        let original: Box<(ByteString, Int)> = Box::new((ByteString::new("key"), Int::new(123)));
        let json_str = original.to_json_string();
        let parsed = Box::<(ByteString, Int)>::from_json_string(&json_str).unwrap();
        assert_eq!(original.0.bytes, parsed.0.bytes);
        assert_eq!(original.1.int, parsed.1.int);
    }
}
