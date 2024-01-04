mod tests {
    use sidan_csl_rs::builder::{core::MeshTxBuilderCore, models::Asset};

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
        mesh.tx_in("93fec6deaafabcc394a15552b57b1beca120d9ee90480d1e5cb42ff20118d40a".to_string(), 1, vec![asset], "addr_test1vr3vljjxan0hl6u28fle2l4ds6ugc9t08lwevpauk38t3agx7rtq6".to_string());
    }
}