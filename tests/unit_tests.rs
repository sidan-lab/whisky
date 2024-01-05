mod tests {
    use serde_json::{json, to_string};
    use sidan_csl_rs::builder::{
        core::MeshTxBuilderCore,
        models::{Asset, Budget, Redeemer},
    };

    #[test]
    fn test_mesh_tx_builder_core() {
        let _mesh = MeshTxBuilderCore::new();
    }

    #[test]
    fn test_tx_in() {
        let mut mesh = MeshTxBuilderCore::new();
        let asset = Asset {
            unit: "lovelace".to_string(),
            quantity: "30000000".to_string(),
        };
        mesh.tx_in(
            "93fec6deaafabcc394a15552b57b1beca120d9ee90480d1e5cb42ff20118d40a".to_string(),
            1,
            vec![asset],
            "addr_test1vr3vljjxan0hl6u28fle2l4ds6ugc9t08lwevpauk38t3agx7rtq6".to_string(),
        );
    }

    #[test]
    fn test_script_tx_in() {
        let mut mesh = MeshTxBuilderCore::new();
        let asset = Asset {
            unit: "lovelace".to_string(),
            quantity: "30000000".to_string(),
        };

        let data = to_string(&json!({
            "constructor": 0,
            "fields": []
        }))
        .unwrap();

        mesh.spending_plutus_script_v2()
            .tx_in(
                "93fec6deaafabcc394a15552b57b1beca120d9ee90480d1e5cb42ff20118d40a".to_string(),
                1,
                vec![asset],
                "addr_test1vr3vljjxan0hl6u28fle2l4ds6ugc9t08lwevpauk38t3agx7rtq6".to_string(),
            )
            .spending_reference_tx_in_inline_datum_present()
            .spending_reference_tx_in_redeemer_value(Redeemer {
                data,
                ex_units: Budget {
                    mem: 3386819,
                    steps: 1048170931,
                },
            });
    }
}
