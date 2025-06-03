use whisky_common::Value;

use super::TxTester;

impl TxTester {
    pub fn get_all_value_to(&self, address: &str) -> Value {
        let mut value = Value::new();
        self.tx_body.outputs.iter().for_each(|output| {
            if output.address == address {
                value.add_assets(&output.amount);
            }
        });
        value
    }
}
