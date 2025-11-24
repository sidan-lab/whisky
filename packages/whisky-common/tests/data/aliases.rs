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
        assert_eq!(json.to_string(), "[{\"bytes\":\"hello\"},{\"bytes\":\"world\"}]");
    }
}
