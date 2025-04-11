#[cfg(test)]
mod constructors {
    use serde_json::json;
    use whisky_common::data::*;

    #[test]
    fn test_con_str() {
        let correct_con_str = "{\"constructor\":10,\"fields\":[{\"bytes\":\"hello\"}]}";
        assert_eq!(
            con_str(10, json!([builtin_byte_string("hello")])).to_string(),
            correct_con_str
        );
    }

    #[test]
    fn test_con_str0() {
        let correct_con_str0 = "{\"constructor\":0,\"fields\":{\"bytes\":\"hello\"}}";
        assert_eq!(
            con_str0(builtin_byte_string("hello")).to_string(),
            correct_con_str0
        );
    }

    #[test]
    fn test_con_str1() {
        let correct_con_str1 = "{\"constructor\":1,\"fields\":{\"bytes\":\"hello\"}}";
        assert_eq!(
            con_str1(builtin_byte_string("hello")).to_string(),
            correct_con_str1
        );
    }

    #[test]
    fn test_con_str2() {
        let correct_con_str2 = "{\"constructor\":2,\"fields\":{\"bytes\":\"hello\"}}";
        assert_eq!(
            con_str2(builtin_byte_string("hello")).to_string(),
            correct_con_str2
        );
    }
}
