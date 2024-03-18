use crate::model::{MeshTxBuilderBody, ValidityRange};

pub struct MeshTxTester {
    pub tx_hex: String,
    pub tx_body: MeshTxBuilderBody,
}

pub trait MeshTxTesterTrait {
    fn new(s: &str) -> Self;
    // TODO: add testing method lists here
}

impl MeshTxTesterTrait for MeshTxTester {
    // Constructor method
    fn new(s: &str) -> MeshTxTester {
        // TODO: Deserialized into the tx_body
        let tx_body = MeshTxBuilderBody {
            inputs: vec![],
            outputs: vec![],
            collaterals: vec![],
            required_signatures: vec![],
            reference_inputs: vec![],
            mints: vec![],
            change_address: "".to_string(),
            change_datum: None,
            metadata: vec![],
            validity_range: ValidityRange {
                invalid_before: None,
                invalid_hereafter: None,
            },
            signing_key: vec![],
        };
        MeshTxTester {
            tx_hex: s.to_string(),
            tx_body,
        }
    }
}
