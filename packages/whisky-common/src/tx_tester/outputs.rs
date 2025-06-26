use crate::{data::Value, Datum};

use super::TxTester;

impl TxTester {
    /// ## Filtering methods for testing outputs
    ///
    /// Not apply filter to outputs
    pub fn all_outputs(&mut self) -> &mut Self {
        self.outputs_evaluating = self.tx_body.outputs.clone();
        self
    }

    /// ## Filtering methods for testing outputs
    ///
    /// Filter outputs by address
    pub fn outputs_at(&mut self, address: &str) -> &mut Self {
        self.outputs_evaluating = self
            .tx_body
            .outputs
            .iter()
            .filter(|output| output.address == address)
            .cloned()
            .collect();
        self
    }

    /// ## Filtering methods for testing outputs
    ///
    /// Filter outputs by unit
    pub fn outputs_with(&mut self, unit: &str) -> &mut Self {
        self.outputs_evaluating = self
            .tx_body
            .outputs
            .iter()
            .filter(|output| {
                let output_value = Value::from_asset_vec(&output.amount);
                let quantity = output_value.get(unit);
                quantity > 0
            })
            .cloned()
            .collect();
        self
    }

    /// ## Filtering methods for testing outputs
    ///
    /// Filter outputs by policy ID
    pub fn outputs_with_policy(&mut self, policy_id: &str) -> &mut Self {
        self.outputs_evaluating = self
            .tx_body
            .outputs
            .iter()
            .filter(|output| {
                let output_value = Value::from_asset_vec(&output.amount);
                let assets = output_value.get_policy_assets(policy_id);
                assets.len() > 0
            })
            .cloned()
            .collect();
        self
    }

    /// ## Filtering methods for testing outputs
    ///
    /// Filter outputs by address and policy ID
    pub fn outputs_at_with_policy(&mut self, address: &str, policy_id: &str) -> &mut Self {
        self.outputs_evaluating = self
            .tx_body
            .outputs
            .iter()
            .filter(|output| {
                let output_value = Value::from_asset_vec(&output.amount);
                let assets = output_value.get_policy_assets(policy_id);
                output.address == address && assets.len() > 0
            })
            .cloned()
            .collect();
        self
    }

    /// ## Filtering methods for testing outputs
    ///
    /// Filter outputs by address and unit
    pub fn outputs_at_with(&mut self, address: &str, unit: &str) -> &mut Self {
        self.outputs_evaluating = self
            .tx_body
            .outputs
            .iter()
            .filter(|output| {
                let output_value = Value::from_asset_vec(&output.amount);
                let quantity = output_value.get(unit);
                output.address == address && quantity > 0
            })
            .cloned()
            .collect();
        self
    }

    /// ## Testing methods for outputs
    ///
    /// *Reminder - It must be called after filtering methods for outputs*
    ///
    /// Check if outputs contain the expected value.
    pub fn outputs_value(&mut self, expected_value: Value) -> &mut Self {
        let mut value = Value::new();
        self.outputs_evaluating.iter().for_each(|output| {
            value.add_assets(&output.amount);
        });
        let is_value_correct = value.eq(&expected_value);
        if !is_value_correct {
            self.add_trace(
                "outputs_value",
                &format!(
                    "tx outputs {:?} have value {:?}, expected {:?}",
                    self.outputs_evaluating, value, expected_value
                ),
            );
        }
        self
    }

    /// ## Testing methods for outputs
    ///
    /// *Reminder - It must be called after filtering methods for outputs*
    ///
    /// Check if outputs contain a specific inline datum.
    pub fn outputs_inline_datum_exist(&mut self, datum_cbor: &str) -> &mut Self {
        let outputs_with_inline_datum: Vec<_> = self
            .outputs_evaluating
            .iter()
            .filter(|output| {
                if let Some(Datum::Inline(datum)) = &output.datum {
                    *datum == *datum_cbor
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        if outputs_with_inline_datum.is_empty() {
            self.add_trace(
                "outputs_inline_datum_exist",
                &format!("No outputs with inline datum matching: {}", datum_cbor),
            );
        }
        self
    }
}
