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
}
