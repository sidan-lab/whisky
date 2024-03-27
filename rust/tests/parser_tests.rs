mod parser_tests {
    use sidan_csl_rs::core::common::*;

    #[test]
    fn test_bytes_to_hex() {
        let bytes = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(bytes_to_hex(&bytes), "000102030405");
    }

    #[test]
    fn test_hex_to_bytes() {
        let bytes = vec![0, 1, 2, 3, 4, 255];
        assert_eq!(hex_to_bytes("0001020304ff").unwrap(), bytes);
    }

    #[test]
    fn test_string_to_hex() {
        assert_eq!(string_to_hex("DELTA"), "44454c5441");
    }

    #[test]
    fn test_hex_to_string() {
        assert_eq!(hex_to_string("44454c5441").unwrap(), "DELTA");
    }
}
