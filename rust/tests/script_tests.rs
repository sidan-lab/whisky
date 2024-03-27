mod script_tests {
    use sidan_csl_rs::core::utils::get_v2_script_hash;

    #[test]
    fn test_get_v2_script_hash() {
        let correct_hash = "cc1bff3c00536918d99a78bd7548e864ffad95c8b6de562f709f0114";
        let compiled_code = "584501000032323232323222533300432323253330073370e900018041baa0011324a2600c0022c60120026012002600600229309b2b118021baa0015734aae7555cf2ba157441";
        assert_eq!(get_v2_script_hash(compiled_code), correct_hash);
    }
}
