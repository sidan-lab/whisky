use super::TxBuilder;

impl TxBuilder {
    /// ## Transaction building method
    ///
    /// Add a transaction total collateral to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `collateral` - The total collateral
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn set_total_collateral(&mut self, collateral: &str) -> &mut Self {
        self.tx_builder_body.total_collateral = Some(collateral.to_string());
        self
    }

    /// ## Transaction building method
    ///
    /// Add a transaction collateral return address to the TxBuilder instance
    ///
    /// ### Arguments
    ///
    /// * `address` - The collateral return address
    ///
    /// ### Returns
    ///
    /// * `Self` - The TxBuilder instance
    pub fn set_collateral_return_address(&mut self, address: &str) -> &mut Self {
        self.tx_builder_body.collateral_return_address = Some(address.to_string());
        self
    }
}
