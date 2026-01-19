#[cfg(test)]
mod tests {
    use serde_json::json;
    use whisky_common::data::*;

    #[test]
    fn test_empty_constr() {
        let correct_empty_constr = "{\"constructor\":0,\"fields\":[]}";
        assert_eq!(Constr::new(0, ()).to_json_string(), correct_empty_constr);
        assert_eq!(constr(0, json!([])).to_string(), correct_empty_constr);
    }

    #[test]
    fn test_constr() {
        let correct_constr = "{\"constructor\":10,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(
            constr(10, json!([byte_string("hello")])).to_string(),
            correct_constr
        );

        let constructor = Constr::new(10, ByteString::new("hello")).to_json_string();
        assert_eq!(constructor, correct_constr);
    }

    #[test]
    fn test_constr_2() {
        let correct_constr =
            "{\"constructor\":10,\"fields\":[{\"bytes\":\"hello\"},{\"int\":123}]}";
        assert_eq!(
            constr(10, json!([byte_string("hello"), integer(123)])).to_string(),
            correct_constr
        );

        let new_box = Box::new((ByteString::new("hello"), Int::new(123)));
        let constructor = Constr::new(10, new_box).to_json_string();
        assert_eq!(constructor, correct_constr);
    }

    #[test]
    fn test_constr0() {
        let correct_constr0 = "{\"constructor\":0,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(constr0([byte_string("hello")]).to_string(), correct_constr0);
    }

    #[test]
    fn test_constr1() {
        let correct_constr1 = "{\"constructor\":1,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(constr1([byte_string("hello")]).to_string(), correct_constr1);
    }

    #[test]
    fn test_constr2() {
        let correct_constr2 = "{\"constructor\":2,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(constr2([byte_string("hello")]).to_string(), correct_constr2);
    }

    // ==================== from_json round-trip tests ====================

    #[test]
    fn test_constr_from_json_roundtrip() {
        // Generic Constr with custom tag
        let original = Constr::new(10, ByteString::new("hello"));
        let json_str = original.to_json_string();
        let parsed = Constr::<ByteString>::from_json_string(&json_str).unwrap();
        assert_eq!(original.tag, parsed.tag);
        assert_eq!(original.fields.bytes, parsed.fields.bytes);
    }

    #[test]
    fn test_constr_empty_from_json_roundtrip() {
        let original = Constr::new(0, ());
        let json_str = original.to_json_string();
        let parsed = Constr::<()>::from_json_string(&json_str).unwrap();
        assert_eq!(original.tag, parsed.tag);
    }

    #[test]
    fn test_constr_multi_field_from_json_roundtrip() {
        let original = Constr::new(5, Box::new((ByteString::new("test"), Int::new(42))));
        let json_str = original.to_json_string();
        let parsed = Constr::<Box<(ByteString, Int)>>::from_json_string(&json_str).unwrap();
        assert_eq!(original.tag, parsed.tag);
        assert_eq!(original.fields.0.bytes, parsed.fields.0.bytes);
        assert_eq!(original.fields.1.int, parsed.fields.1.int);
    }

    #[test]
    fn test_constr2_from_json_roundtrip() {
        let original = Constr2::new(ByteString::new("data"));
        let json_str = original.to_json_string();
        let parsed = Constr2::<ByteString>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields.bytes, parsed.fields.bytes);
    }

    #[test]
    fn test_constr3_from_json_roundtrip() {
        let original = Constr3::new(Int::new(100));
        let json_str = original.to_json_string();
        let parsed = Constr3::<Int>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields.int, parsed.fields.int);
    }

    #[test]
    fn test_constr4_from_json_roundtrip() {
        let original = Constr4::new(Bool::new(true));
        let json_str = original.to_json_string();
        let parsed = Constr4::<Bool>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields, parsed.fields);
    }

    #[test]
    fn test_constr5_from_json_roundtrip() {
        let original = Constr5::new(ByteString::new("five"));
        let json_str = original.to_json_string();
        let parsed = Constr5::<ByteString>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields.bytes, parsed.fields.bytes);
    }

    #[test]
    fn test_constr6_from_json_roundtrip() {
        let original = Constr6::new(Int::new(6));
        let json_str = original.to_json_string();
        let parsed = Constr6::<Int>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields.int, parsed.fields.int);
    }

    #[test]
    fn test_constr7_from_json_roundtrip() {
        let original = Constr7::new(ByteString::new("seven"));
        let json_str = original.to_json_string();
        let parsed = Constr7::<ByteString>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields.bytes, parsed.fields.bytes);
    }

    #[test]
    fn test_constr8_from_json_roundtrip() {
        let original = Constr8::new(Int::new(8));
        let json_str = original.to_json_string();
        let parsed = Constr8::<Int>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields.int, parsed.fields.int);
    }

    #[test]
    fn test_constr9_from_json_roundtrip() {
        let original = Constr9::new(ByteString::new("nine"));
        let json_str = original.to_json_string();
        let parsed = Constr9::<ByteString>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields.bytes, parsed.fields.bytes);
    }

    #[test]
    fn test_constr10_from_json_roundtrip() {
        let original = Constr10::new(Int::new(10));
        let json_str = original.to_json_string();
        let parsed = Constr10::<Int>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields.int, parsed.fields.int);
    }

    #[test]
    fn test_box_constr0_from_json_roundtrip() {
        let original: Box<Constr0<ByteString>> = Box::new(Constr0::new(ByteString::new("boxed")));
        let json_str = original.to_json_string();
        let parsed = Box::<Constr0<ByteString>>::from_json_string(&json_str).unwrap();
        assert_eq!(original.fields.bytes, parsed.fields.bytes);
    }

    #[test]
    fn test_box_constr_from_json_roundtrip() {
        let original: Box<Constr<Int>> = Box::new(Constr::new(7, Int::new(777)));
        let json_str = original.to_json_string();
        let parsed = Box::<Constr<Int>>::from_json_string(&json_str).unwrap();
        assert_eq!(original.tag, parsed.tag);
        assert_eq!(original.fields.int, parsed.fields.int);
    }

    #[test]
    fn test_constr_wrong_tag_error() {
        // Try to parse a Constr1 JSON as Constr0 - should fail
        let constr1 = Constr1::new(ByteString::new("test"));
        let json_str = constr1.to_json_string();
        let result = Constr0::<ByteString>::from_json_string(&json_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_constr_missing_constructor_error() {
        let invalid_json = r#"{"fields": []}"#;
        let result = Constr::<()>::from_json_string(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_constr_missing_fields_error() {
        let invalid_json = r#"{"constructor": 0}"#;
        let result = Constr::<()>::from_json_string(invalid_json);
        assert!(result.is_err());
    }
}
