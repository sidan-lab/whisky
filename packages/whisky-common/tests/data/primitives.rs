#[cfg(test)]
mod tests {
    use whisky_common::data::*;

    #[test]
    fn test_bool() {
        let correct_bool = "{\"constructor\":1,\"fields\":[]}";
        assert_eq!(bool(true).to_string(), correct_bool);
        assert_eq!(Bool::new(true).to_json_string(), correct_bool);
    }

    #[test]
    fn test_byte_string() {
        let correct_byte_string = "{\"bytes\":\"hello\"}";
        assert_eq!(byte_string("hello").to_string(), correct_byte_string);
        assert_eq!(
            ByteString::new("hello").to_json_string(),
            correct_byte_string
        );
    }

    #[test]
    fn test_integer() {
        let correct_integer = "{\"int\":1}";
        assert_eq!(integer(1).to_string(), correct_integer);
        assert_eq!(Int::new(1).to_json_string(), correct_integer);
    }

    #[test]
    fn test_list() {
        let correct_list = "{\"list\":[{\"int\":1},{\"int\":2},{\"int\":3}]}";
        assert_eq!(
            list(vec![integer(1), integer(2), integer(3)]).to_string(),
            correct_list
        );
        assert_eq!(
            List::new(&[Int::new(1), Int::new(2), Int::new(3)]).to_json_string(),
            correct_list
        );
    }

    #[test]
    fn test_tuple() {
        let correct_list = "{\"list\":[{\"int\":1},{\"int\":2},{\"int\":3}]}";
        assert_eq!(
            tuple(vec![integer(1), integer(2), integer(3)]).to_string(),
            correct_list
        );
        assert_eq!(
            Tuple::new((Int::new(1), Int::new(2), Int::new(3))).to_json_string(),
            correct_list
        );
    }

    #[test]
    fn test_tuple_2() {
        let correct_list = "{\"list\":[{\"int\":1},{\"int\":2},{\"bytes\":\"3\"}]}";
        assert_eq!(
            Tuple::new((Int::new(1), Int::new(2), ByteString::new("3"))).to_json_string(),
            correct_list
        );
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

        assert_eq!(
            Map::new(&[
                (ByteString::new("hello"), ByteString::new("world")),
                (ByteString::new("123"), ByteString::new("456"))
            ])
            .to_json_string(),
            correct_assoc_map
        );
    }

    #[test]
    fn test_pairs() {
        let correct_pairs =
      "{\"map\":[{\"k\":{\"bytes\":\"hello\"},\"v\":{\"bytes\":\"world\"}},{\"k\":{\"bytes\":\"123\"},\"v\":{\"bytes\":\"456\"}}]}";
        assert_eq!(
            pairs(vec![
                (byte_string("hello"), byte_string("world")),
                (byte_string("123"), byte_string("456"))
            ])
            .to_string(),
            correct_pairs
        );
        assert_eq!(
            Map::new(&[
                (ByteString::new("hello"), ByteString::new("world")),
                (ByteString::new("123"), ByteString::new("456"))
            ])
            .to_json_string(),
            correct_pairs
        );
    }

    // ==================== from_json round-trip tests ====================

    // Tuple struct wrapper tests
    #[test]
    fn test_tuple_struct_from_json_roundtrip() {
        let original = Tuple::new((Int::new(1), Int::new(2), Int::new(3)));
        let json_str = original.to_json_string();
        let parsed = Tuple::<(Int, Int, Int)>::from_json_string(&json_str).unwrap();
        assert_eq!(original.items.0.int, parsed.items.0.int);
        assert_eq!(original.items.1.int, parsed.items.1.int);
        assert_eq!(original.items.2.int, parsed.items.2.int);
    }

    #[test]
    fn test_tuple_struct_mixed_types_from_json_roundtrip() {
        let original = Tuple::new((Int::new(42), ByteString::new("test")));
        let json_str = original.to_json_string();
        let parsed = Tuple::<(Int, ByteString)>::from_json_string(&json_str).unwrap();
        assert_eq!(original.items.0.int, parsed.items.0.int);
        assert_eq!(original.items.1.bytes, parsed.items.1.bytes);
    }

    // Raw tuple tests (3+ elements)
    #[test]
    fn test_three_element_tuple_from_json_roundtrip() {
        let original = (ByteString::new("a"), ByteString::new("b"), ByteString::new("c"));
        let json_str = original.to_json_string();
        let parsed = <(ByteString, ByteString, ByteString)>::from_json_string(&json_str).unwrap();
        assert_eq!(original.0.bytes, parsed.0.bytes);
        assert_eq!(original.1.bytes, parsed.1.bytes);
        assert_eq!(original.2.bytes, parsed.2.bytes);
    }

    #[test]
    fn test_four_element_tuple_from_json_roundtrip() {
        let original = (Int::new(1), Int::new(2), Int::new(3), Int::new(4));
        let json_str = original.to_json_string();
        let parsed = <(Int, Int, Int, Int)>::from_json_string(&json_str).unwrap();
        assert_eq!(original.0.int, parsed.0.int);
        assert_eq!(original.1.int, parsed.1.int);
        assert_eq!(original.2.int, parsed.2.int);
        assert_eq!(original.3.int, parsed.3.int);
    }

    #[test]
    fn test_five_element_tuple_from_json_roundtrip() {
        let original = (
            ByteString::new("one"),
            Int::new(2),
            Bool::new(true),
            ByteString::new("four"),
            Int::new(5),
        );
        let json_str = original.to_json_string();
        let parsed = <(ByteString, Int, Bool, ByteString, Int)>::from_json_string(&json_str).unwrap();
        assert_eq!(original.0.bytes, parsed.0.bytes);
        assert_eq!(original.1.int, parsed.1.int);
        assert_eq!(original.2, parsed.2);
        assert_eq!(original.3.bytes, parsed.3.bytes);
        assert_eq!(original.4.int, parsed.4.int);
    }

    #[test]
    fn test_single_element_tuple_from_json_roundtrip() {
        let original = (ByteString::new("single"),);
        let json_str = original.to_json_string();
        let parsed = <(ByteString,)>::from_json_string(&json_str).unwrap();
        assert_eq!(original.0.bytes, parsed.0.bytes);
    }

    // PlutusData enum tests
    #[test]
    fn test_plutus_data_integer_from_json_roundtrip() {
        let original = PlutusData::Integer(Int::new(12345));
        let json_str = original.to_json_string();
        let parsed = PlutusData::from_json_string(&json_str).unwrap();
        match parsed {
            PlutusData::Integer(int) => assert_eq!(int.int, 12345),
            _ => panic!("Expected Integer variant"),
        }
    }

    #[test]
    fn test_plutus_data_bytestring_from_json_roundtrip() {
        let original = PlutusData::ByteString(ByteString::new("deadbeef"));
        let json_str = original.to_json_string();
        let parsed = PlutusData::from_json_string(&json_str).unwrap();
        match parsed {
            PlutusData::ByteString(bs) => assert_eq!(bs.bytes, "deadbeef"),
            _ => panic!("Expected ByteString variant"),
        }
    }

    #[test]
    fn test_plutus_data_list_from_json_roundtrip() {
        let original = PlutusData::List(List::new(&[
            PlutusData::Integer(Int::new(1)),
            PlutusData::Integer(Int::new(2)),
        ]));
        let json_str = original.to_json_string();
        let parsed = PlutusData::from_json_string(&json_str).unwrap();
        match parsed {
            PlutusData::List(list) => {
                assert_eq!(list.items.len(), 2);
                match &list.items[0] {
                    PlutusData::Integer(i) => assert_eq!(i.int, 1),
                    _ => panic!("Expected Integer"),
                }
            }
            _ => panic!("Expected List variant"),
        }
    }

    #[test]
    fn test_plutus_data_map_from_json_roundtrip() {
        let original = PlutusData::Map(Map::new(&[(
            PlutusData::ByteString(ByteString::new("key")),
            PlutusData::Integer(Int::new(100)),
        )]));
        let json_str = original.to_json_string();
        let parsed = PlutusData::from_json_string(&json_str).unwrap();
        match parsed {
            PlutusData::Map(map) => {
                assert_eq!(map.map.len(), 1);
            }
            _ => panic!("Expected Map variant"),
        }
    }

    #[test]
    fn test_plutus_data_bool_true_from_json_roundtrip() {
        let original = PlutusData::Bool(Bool::True);
        let json_str = original.to_json_string();
        let parsed = PlutusData::from_json_string(&json_str).unwrap();
        match parsed {
            PlutusData::Bool(b) => assert_eq!(b, Bool::True),
            _ => panic!("Expected Bool variant"),
        }
    }

    #[test]
    fn test_plutus_data_bool_false_from_json_roundtrip() {
        let original = PlutusData::Bool(Bool::False);
        let json_str = original.to_json_string();
        let parsed = PlutusData::from_json_string(&json_str).unwrap();
        match parsed {
            PlutusData::Bool(b) => assert_eq!(b, Bool::False),
            _ => panic!("Expected Bool variant"),
        }
    }

    #[test]
    fn test_plutus_data_constr_from_json_roundtrip() {
        let inner = Box::new(PlutusData::ByteString(ByteString::new("inner")));
        let original = PlutusData::Constr(Constr::new(5, inner));
        let json_str = original.to_json_string();
        let parsed = PlutusData::from_json_string(&json_str).unwrap();
        match parsed {
            PlutusData::Constr(constr) => {
                assert_eq!(constr.tag, 5);
                match constr.fields.as_ref() {
                    PlutusData::ByteString(bs) => assert_eq!(bs.bytes, "inner"),
                    _ => panic!("Expected ByteString inside Constr"),
                }
            }
            _ => panic!("Expected Constr variant"),
        }
    }

    #[test]
    fn test_plutus_data_unrecognized_format_error() {
        let invalid_json = r#"{"unknown": "format"}"#;
        let result = PlutusData::from_json_string(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_tuple_not_enough_elements_error() {
        // Try to parse a 2-element array as a 3-element tuple
        let json_str = r#"[{"int": 1}, {"int": 2}]"#;
        let result = <(Int, Int, Int)>::from_json_string(json_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_tuple_struct_missing_list_error() {
        let invalid_json = r#"{"wrong": "format"}"#;
        let result = Tuple::<(Int,)>::from_json_string(invalid_json);
        assert!(result.is_err());
    }
}
