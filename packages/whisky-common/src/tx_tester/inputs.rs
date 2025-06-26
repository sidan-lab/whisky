use crate::data::Value;

use super::TxTester;

impl TxTester {
    /// ## Filtering methods for testing inputs
    ///
    /// Not apply filter to inputs
    pub fn all_inputs(&mut self) -> &mut Self {
        self.inputs_evaluating = self.tx_body.inputs.clone();
        self
    }

    /// ## Filtering methods for testing inputs
    ///
    /// Filter inputs by address
    pub fn inputs_at(&mut self, address: &str) -> &mut Self {
        self.inputs_evaluating = self
            .tx_body
            .inputs
            .iter()
            .filter(|input| input.to_utxo().output.address == address)
            .cloned()
            .collect();
        self
    }

    /// ## Filtering methods for testing inputs
    ///
    /// Filter inputs by unit
    pub fn inputs_with(&mut self, unit: &str) -> &mut Self {
        self.inputs_evaluating = self
            .tx_body
            .inputs
            .iter()
            .filter(|input| {
                let input_value = Value::from_asset_vec(&input.to_utxo().output.amount.clone());
                let quantity = input_value.get(unit);
                quantity > 0
            })
            .cloned()
            .collect();
        self
    }

    /// ## Filtering methods for testing inputs
    ///
    /// Filter inputs by policy ID
    pub fn inputs_with_policy(&mut self, policy_id: &str) -> &mut Self {
        self.inputs_evaluating = self
            .tx_body
            .inputs
            .iter()
            .filter(|input| {
                let input_value = Value::from_asset_vec(&input.to_utxo().output.amount.clone());
                let assets = input_value.get_policy_assets(policy_id);
                assets.len() > 0
            })
            .cloned()
            .collect();
        self
    }

    /// ## Filtering methods for testing inputs
    ///
    /// Filter inputs by address and policy ID
    pub fn inputs_at_with_policy(&mut self, address: &str, policy_id: &str) -> &mut Self {
        self.inputs_evaluating = self
            .tx_body
            .inputs
            .iter()
            .filter(|input| {
                let utxo = input.to_utxo();
                let input_value = Value::from_asset_vec(&utxo.output.amount.clone());
                let assets = input_value.get_policy_assets(policy_id);
                utxo.output.address == address && assets.len() > 0
            })
            .cloned()
            .collect();
        self
    }

    /// ## Filtering methods for testing inputs
    ///
    /// Filter inputs by address and unit
    pub fn inputs_at_with(&mut self, address: &str, unit: &str) -> &mut Self {
        self.inputs_evaluating = self
            .tx_body
            .inputs
            .iter()
            .filter(|input| {
                let utxo = input.to_utxo();
                let input_value = Value::from_asset_vec(&utxo.output.amount.clone());
                let quantity = input_value.get(unit);
                utxo.output.address == address && quantity > 0
            })
            .cloned()
            .collect();
        self
    }

    /// ## Testing methods for inputs
    ///
    /// *Reminder - It must be called after filtering methods for inputs*
    ///
    /// Check if inputs contain the expected value.
    pub fn inputs_value(&mut self, expected_value: Value) -> &mut Self {
        let mut value = Value::new();
        self.inputs_evaluating.iter().for_each(|input| {
            let utxo = input.to_utxo();
            value.add_assets(&utxo.output.amount);
        });
        let is_value_correct = value.eq(&expected_value);
        if !is_value_correct {
            self.add_trace(
                "inputs_value",
                &format!(
                    "inputs {:?} have value {:?}, expect {:?}",
                    self.inputs_evaluating, value, expected_value
                ),
            );
        }

        self
    }

    /// ## Testing methods for inputs
    ///
    /// *Reminder - It must be called after filtering methods for inputs*
    ///
    /// Check if inputs contain a specific inline datum.
    pub fn inputs_inline_datum_exist(&mut self, datum_cbor: &str) -> &mut Self {
        let inputs_with_inline_datum: Vec<_> = self
            .inputs_evaluating
            .iter()
            .filter(|input| {
                let utxo = input.to_utxo();
                if let Some(datum) = utxo.output.plutus_data {
                    datum == *datum_cbor
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        if inputs_with_inline_datum.is_empty() {
            self.add_trace(
                "inputs_inline_datum_exist",
                &format!("No inputs with inline datum matching: {}", datum_cbor),
            );
        }
        self
    }
}
