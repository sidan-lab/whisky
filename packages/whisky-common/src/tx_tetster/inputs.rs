use crate::Value;

use super::TxTester;

impl TxTester {
    pub fn check_all_value_from(&mut self, address: &str, expected_value: Value) -> &Self {
        let mut value = Value::new();
        self.tx_body.inputs.iter().for_each(|input| {
            let utxo = input.to_utxo();
            if utxo.output.address == address {
                value.add_assets(&utxo.output.amount);
            }
        });

        let is_value_correct = value.eq(&expected_value);
        if !is_value_correct {
            self.add_trace(
                "check_all_value_from",
                &format!(
                    "tx outputs to {} have value {:?}, expected {:?}",
                    address, value, expected_value
                ),
            );
        }

        self
    }
}
