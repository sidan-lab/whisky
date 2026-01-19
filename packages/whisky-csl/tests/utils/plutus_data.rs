#[cfg(test)]
mod tests {
    use whisky_common::data::{Bool, ByteString, Constr, Constr0, Int, List, Map, PlutusData};
    use whisky_csl::PlutusDataCbor;

    #[test]
    fn test_int_round_trip() {
        let original = Int::new(42);
        let cbor = original.to_cbor().unwrap();
        let parsed = Int::from_cbor(&cbor).unwrap();
        assert_eq!(original.int, parsed.int);
    }

    #[test]
    fn test_negative_int_round_trip() {
        let original = Int::new(-12345);
        let cbor = original.to_cbor().unwrap();
        let parsed = Int::from_cbor(&cbor).unwrap();
        assert_eq!(original.int, parsed.int);
    }

    #[test]
    fn test_byte_string_round_trip() {
        let original = ByteString::new("deadbeef");
        let cbor = original.to_cbor().unwrap();
        let parsed = ByteString::from_cbor(&cbor).unwrap();
        assert_eq!(original.bytes, parsed.bytes);
    }

    #[test]
    fn test_bool_true_round_trip() {
        let original = Bool::True;
        let cbor = original.to_cbor().unwrap();
        let parsed = Bool::from_cbor(&cbor).unwrap();
        assert!(matches!(parsed, Bool::True));
    }

    #[test]
    fn test_bool_false_round_trip() {
        let original = Bool::False;
        let cbor = original.to_cbor().unwrap();
        let parsed = Bool::from_cbor(&cbor).unwrap();
        assert!(matches!(parsed, Bool::False));
    }

    #[test]
    fn test_list_round_trip() {
        let items = vec![Int::new(1), Int::new(2), Int::new(3)];
        let original = List::new(&items);
        let cbor = original.to_cbor().unwrap();
        let parsed = List::<Int>::from_cbor(&cbor).unwrap();
        assert_eq!(original.items.len(), parsed.items.len());
        for (a, b) in original.items.iter().zip(parsed.items.iter()) {
            assert_eq!(a.int, b.int);
        }
    }

    #[test]
    fn test_map_round_trip() {
        let mut original = Map::<ByteString, Int>::new(&[]);
        original.insert(ByteString::new("6b657931"), Int::new(100)); // "key1" in hex
        original.insert(ByteString::new("6b657932"), Int::new(200)); // "key2" in hex

        let cbor = original.to_cbor().unwrap();
        let parsed = Map::<ByteString, Int>::from_cbor(&cbor).unwrap();

        assert_eq!(original.map.len(), parsed.map.len());
    }

    #[test]
    fn test_constr_round_trip() {
        let original: Constr<Box<Int>> = Constr::new(0, Box::new(Int::new(999)));
        let cbor = original.to_cbor().unwrap();
        let parsed = Constr::<Box<Int>>::from_cbor(&cbor).unwrap();
        assert_eq!(original.tag, parsed.tag);
    }

    #[test]
    fn test_constr0_round_trip() {
        let original = Constr0::new(Int::new(123));
        let cbor = original.to_cbor().unwrap();
        let parsed = Constr0::<Int>::from_cbor(&cbor).unwrap();
        assert_eq!(original.fields.int, parsed.fields.int);
    }

    #[test]
    fn test_plutus_data_integer_round_trip() {
        let original = PlutusData::Integer(Int::new(9999));
        let cbor = original.to_cbor().unwrap();
        let parsed = PlutusData::from_cbor(&cbor).unwrap();
        match parsed {
            PlutusData::Integer(i) => assert_eq!(i.int, 9999),
            _ => panic!("Expected Integer"),
        }
    }

    #[test]
    fn test_plutus_data_bytes_round_trip() {
        let original = PlutusData::ByteString(ByteString::new("cafebabe"));
        let cbor = original.to_cbor().unwrap();
        let parsed = PlutusData::from_cbor(&cbor).unwrap();
        match parsed {
            PlutusData::ByteString(b) => assert_eq!(b.bytes, "cafebabe"),
            _ => panic!("Expected ByteString"),
        }
    }

    #[test]
    fn test_plutus_data_list_round_trip() {
        let items = vec![
            PlutusData::Integer(Int::new(1)),
            PlutusData::Integer(Int::new(2)),
        ];
        let original = PlutusData::List(List::new(&items));
        let cbor = original.to_cbor().unwrap();
        let parsed = PlutusData::from_cbor(&cbor).unwrap();
        match parsed {
            PlutusData::List(l) => assert_eq!(l.items.len(), 2),
            _ => panic!("Expected List"),
        }
    }

    #[test]
    fn test_from_known_cbor() {
        // CBOR for integer 42: 0x182a
        let cbor_hex = "182a";
        let parsed = Int::from_cbor(cbor_hex).unwrap();
        assert_eq!(parsed.int, 42);
    }

    #[test]
    fn test_complex_nested_structure() {
        // Create a complex nested structure
        let items = vec![Int::new(10), Int::new(20)];
        let inner_list = List::new(&items);
        let constr: Constr<Box<List<Int>>> = Constr::new(1, Box::new(inner_list));

        let cbor = constr.to_cbor().unwrap();
        let parsed = Constr::<Box<List<Int>>>::from_cbor(&cbor).unwrap();

        assert_eq!(parsed.tag, 1);
        assert_eq!(parsed.fields.items.len(), 2);
        assert_eq!(parsed.fields.items[0].int, 10);
        assert_eq!(parsed.fields.items[1].int, 20);
    }
}
