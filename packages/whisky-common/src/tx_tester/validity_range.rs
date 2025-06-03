use super::TxTester;

impl TxTester {
    /// ## Testing methods for validity range
    ///
    /// Checks if the transaction is valid after a specified timestamp.
    pub fn valid_after(&mut self, required_timestamp: u64) -> &mut Self {
        let invalid_before =
            if let Some(validity_range) = &self.tx_body.validity_range.invalid_hereafter {
                *validity_range
            } else {
                9999_999_999_999 // A very large number representing "no limit"
            };
        let is_valid_after =
            if let Some(validity_range) = &self.tx_body.validity_range.invalid_before {
                *validity_range < required_timestamp
            } else {
                true
            };

        if !is_valid_after {
            self.add_trace(
                "valid_after",
                &format!(
                    "tx invalid before {}, with required_timestamp {}",
                    invalid_before, required_timestamp
                ),
            );
        }

        self
    }

    /// ## Testing methods for validity range
    ///
    /// Checks if the transaction is valid before a specified timestamp.
    pub fn valid_before(&mut self, required_timestamp: u64) -> &mut Self {
        let invalid_hereafter =
            if let Some(validity_range) = &self.tx_body.validity_range.invalid_before {
                *validity_range
            } else {
                0 // Representing "no limit"
            };

        let is_valid_before =
            if let Some(validity_range) = &self.tx_body.validity_range.invalid_hereafter {
                *validity_range > required_timestamp
            } else {
                true
            };

        if !is_valid_before {
            self.add_trace(
                "valid_before",
                &format!(
                    "tx invalid after {}, with required_timestamp {}",
                    invalid_hereafter, required_timestamp
                ),
            );
        }

        self
    }
}
