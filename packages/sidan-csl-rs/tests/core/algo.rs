#[cfg(test)]
mod algo_tests {
    use sidan_csl_rs::core::algo::select_utxos;
    use sidan_csl_rs::model::{Asset, UTxO, UtxoInput, UtxoOutput, Value};

    #[test]
    fn test_basic_selection() {
        let utxo_list = vec![UTxO {
            input: UtxoInput {
                output_index: 0,
                tx_hash: "test".to_string(),
            },
            output: UtxoOutput {
                address: "test".to_string(),
                amount: vec![Asset::new_from_str("lovelace", "10000000")],
                data_hash: None,
                plutus_data: None,
                script_ref: None,
                script_hash: None,
            },
        }];

        let mut required_assets: Value = Value::new();
        required_assets.add_asset("lovelace", 5_000_000);
        let selected_list = select_utxos(&utxo_list, required_assets, "5000000").unwrap();
        assert_eq!(utxo_list, selected_list);
    }
}
