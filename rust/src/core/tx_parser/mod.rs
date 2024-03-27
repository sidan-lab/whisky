use crate::model::{JsVecString, MeshTxBuilderBody, ValidityRange};

pub struct MeshTxParser {
    pub tx_hex: String,
    pub tx_fee_lovelace: u64,
    pub tx_body: MeshTxBuilderBody,
}

pub trait MeshTxParserTrait {
    fn new(s: &str) -> Self;
    // TODO: add testing method lists here
}

impl MeshTxParserTrait for MeshTxParser {
    // Constructor method
    fn new(s: &str) -> MeshTxParser {
        // TODO: Deserialized into the tx_body
        let tx_body = MeshTxBuilderBody {
            inputs: vec![],
            outputs: vec![],
            collaterals: vec![],
            required_signatures: JsVecString::new(),
            reference_inputs: vec![],
            mints: vec![],
            change_address: "".to_string(),
            change_datum: None,
            metadata: vec![],
            validity_range: ValidityRange {
                invalid_before: None,
                invalid_hereafter: None,
            },
            signing_key: JsVecString::new(),
        };
        MeshTxParser {
            tx_hex: s.to_string(),
            tx_fee_lovelace: 0,
            tx_body,
        }
    }
}
