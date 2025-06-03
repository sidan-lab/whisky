use whisky_common::Value;

use super::TxTester;

impl TxTester {
    pub fn get_all_value_from(&self, address: &str) -> Value {
        let mut value = Value::new();
        self.tx_body.inputs.iter().for_each(|input| {
            let utxo = input.to_utxo();
            if utxo.output.address == address {
                value.add_assets(&utxo.output.amount);
            }
        });
        value
    }
}
