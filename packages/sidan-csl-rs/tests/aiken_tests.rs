mod aiken_tests {
    use sidan_csl_rs::{core::utils::apply_params_to_script, model::BuilderDataType};

    #[test]
    fn test_apply_params_to_script() {
        use serde_json::json;
        let script =
          "584501000032323232323222533300432323253330073370e900018041baa0011324a2600c0022c60120026012002600600229309b2b118021baa0015734aae7555cf2ba157441";
        let param = json!({ "bytes": "1234"}).to_string();

        let aiken_params: Vec<&str> = vec![&param];

        assert_eq!(
            apply_params_to_script(script, &aiken_params, BuilderDataType::JSON).unwrap(),
            "584f584d010000332323232323222533300432323253330073370e900018041baa0011324a2600c0022c60120026012002600600229309b2b118021baa0015734aae7555cf2ba157449801034212340001"
        );
    }
}
