#[cfg(test)]
mod primitives {
    use whisky_common::data::*;

    #[test]
    fn test_bool() {
        let correct_bool = "{\"constructor\":1,\"fields\":[]}";
        assert_eq!(bool(true).to_string(), correct_bool);
    }

    #[test]
    fn test_byte_string() {
        let correct_byte_string = "{\"bytes\":\"hello\"}";
        assert_eq!(byte_string("hello").to_string(), correct_byte_string);
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
    }
}
