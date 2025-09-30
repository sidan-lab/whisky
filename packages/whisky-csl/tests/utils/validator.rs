mod tests {
    use cquisitor_lib::{
        common::{Asset, CostModels, ExUnitPrices, ExUnits, SubCoin, TxInput, TxOutput, UTxO},
        validators::{
            common::NetworkType,
            input_contexts::{UtxoInputContext, ValidationInputContext},
            protocol_params::ProtocolParameters,
        },
    };
    use serde_json::json;
    use whisky_common::constants::get_preprod_cost_models;
    use whisky_csl::csl;
    use whisky_csl::validate_tx;

    #[test]
    fn test_validate_tx() {
        let tx_hex = "84a80082825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad9800825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98010d81825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad9801128182582004b9070a30bd63abaaf59a3c48a1575c4127bb0edb00ecd5141fd18a85c721aa000181a200581d601fd5bab167338971d92b4d8f0bdf57d889903e6e934e7ea38c7dadf1011b00000002529898c810a200581d601fd5bab167338971d92b4d8f0bdf57d889903e6e934e7ea38c7dadf1011b0000000252882db4111a000412f1021a0002b74b0b5820775d0cf3c95993f6210e4410e92f72ebc3942ce9c1433694749aa239e5d13387a200818258201557f444f3ae6e61dfed593ae15ec8dbd57b8138972bf16fde5b4c559f41549b5840729f1f14ef05b7cf9b0d7583e6777674f80ae64a35bbd6820cc3c82ddf0412ca1d751b7d886eece3c6e219e1c5cc9ef3d387a8d2078f47125d54b474fbdfbd0105818400000182190b111a000b5e35f5f6";
        let validation_context = ValidationInputContext::new(
            vec![
                UtxoInputContext {
                    utxo: UTxO {
                        input: TxInput {
                            tx_hash:
                                "604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98"
                                    .to_string(),
                            output_index: 0,
                        },
                        output: TxOutput {
                            address:
                                "addr_test1wzlwsgq97vchypqzk8u8lz30w932tvx7akcj7csm02scl7qlghd97"
                                    .to_string(),
                            amount: vec![Asset {
                                unit: "lovelace".to_string(),
                                quantity: "986990".to_string(),
                            }],
                            data_hash: None,
                            plutus_data: Some(
                                csl::PlutusData::from_json(
                                    &json!({
                                        "constructor": 0,
                                        "fields": []
                                    })
                                    .to_string(),
                                    csl::PlutusDatumSchema::DetailedSchema,
                                )
                                .unwrap()
                                .to_hex(),
                            ),
                            script_hash: None,
                            script_ref: None,
                        },
                    },
                    is_spent: false,
                },
                UtxoInputContext {
                    utxo: UTxO {
                        input: TxInput {
                            tx_hash:
                                "604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98"
                                    .to_string(),
                            output_index: 1,
                        },
                        output: TxOutput {
                            address:
                                "addr_test1vq0atw43vuecjuwe9dxc7z7l2lvgnyp7d6f5ul4r3376mug8v67h5"
                                    .to_string(),
                            amount: vec![Asset {
                                unit: "lovelace".to_string(),
                                quantity: "9974857893".to_string(),
                            }],
                            data_hash: None,
                            plutus_data: None,
                            script_hash: None,
                            script_ref: None,
                        },
                    },
                    is_spent: false,
                },
                UtxoInputContext {
                    utxo: UTxO {
                        input: TxInput {
                            tx_hash:
                                "04b9070a30bd63abaaf59a3c48a1575c4127bb0edb00ecd5141fd18a85c721aa"
                                    .to_string(),
                            output_index: 0,
                        },
                        output: TxOutput {
                            address:
                                "addr_test1wzlwsgq97vchypqzk8u8lz30w932tvx7akcj7csm02scl7qlghd97"
                                    .to_string(),
                            amount: vec![Asset {
                                unit: "lovelace".to_string(),
                                quantity: "986990".to_string(),
                            }],
                            data_hash: None,
                            plutus_data: None,
                            script_hash: None,
                            script_ref: Some(
                                "82025655010000322223253330054a229309b2b1bad0025735".to_string(),
                            ),
                        },
                    },
                    is_spent: false,
                },
            ],
            ProtocolParameters {
                min_fee_coefficient_a: 44,
                min_fee_constant_b: 155381,
                max_block_body_size: 98304,
                max_transaction_size: 16384,
                max_block_header_size: 1100,
                stake_key_deposit: 2000000,
                stake_pool_deposit: 500000000,
                max_epoch_for_pool_retirement: 18,
                protocol_version: (10, 0),
                min_pool_cost: 170000000,
                ada_per_utxo_byte: 4310,
                cost_models: CostModels {
                    plutus_v1: get_preprod_cost_models().get(0).cloned(),
                    plutus_v2: get_preprod_cost_models().get(1).cloned(),
                    plutus_v3: get_preprod_cost_models().get(2).cloned(),
                },
                execution_prices: ExUnitPrices {
                    mem_price: SubCoin {
                        numerator: 577,
                        denominator: 10000,
                    },
                    step_price: SubCoin {
                        numerator: 721,
                        denominator: 10000000,
                    },
                },
                max_tx_execution_units: ExUnits {
                    mem: 16000000,
                    steps: 10000000000,
                },
                max_block_execution_units: ExUnits {
                    mem: 80000000,
                    steps: 40000000000,
                },
                max_value_size: 5000,
                collateral_percentage: 150,
                max_collateral_inputs: 3,
                governance_action_deposit: 100000000000,
                drep_deposit: 500000000,
                reference_script_cost_per_byte: SubCoin {
                    numerator: 15,
                    denominator: 1,
                },
            },
            0,
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            0,
            NetworkType::Preprod,
            vec![],
            vec![],
        );
        let result = validate_tx(tx_hex, validation_context);
        assert!(result.is_ok());
    }
}
