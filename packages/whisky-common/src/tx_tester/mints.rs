use super::TxTester;

impl TxTester {
    /// ## Testing methods for mints
    ///
    /// Checks if a specific token is minted in the transaction.
    pub fn token_minted(&mut self, policy_id: &str, asset_name: &str, quantity: i128) -> &mut Self {
        let is_token_minted = self.token_minted_logic(policy_id, asset_name, quantity);

        if !is_token_minted {
            self.add_trace(
                "token_minted",
                &format!(
                    "Token with policy_id: {}, asset_name: {}, quantity: {} not found in mints.",
                    policy_id, asset_name, quantity
                ),
            );
        }

        self
    }

    /// ## Testing methods for mints
    ///
    /// Checks if a specific token is minted in the transaction and that it is the only mint.
    pub fn only_token_minted(
        &mut self,
        policy_id: &str,
        asset_name: &str,
        quantity: i128,
    ) -> &mut Self {
        let is_token_minted = self.token_minted_logic(policy_id, asset_name, quantity);
        let is_only_one_mint = self.tx_body.mints.len() == 1;

        if !is_token_minted {
            self.add_trace(
                "only_token_minted",
                &format!(
                    "Token with policy_id: {}, asset_name: {}, quantity: {} not found in mints",
                    policy_id, asset_name, quantity
                ),
            );
        }
        if !is_only_one_mint {
            self.add_trace(
                "only_token_minted",
                &format!(
                    "Expected only one mint, but found {} mints.",
                    self.tx_body.mints.len()
                ),
            );
        }
        self
    }

    /// ## Testing methods for mints
    ///
    /// Checks if a specific token is minted in the transaction, ensuring that it is the only mint for the given policy ID.
    pub fn policy_only_minted_token(
        &mut self,
        policy_id: &str,
        asset_name: &str,
        quantity: i128,
    ) -> &mut Self {
        let filtered_mints: Vec<_> = self
            .tx_body
            .mints
            .iter()
            .filter(|token| {
                let mint_param = token.get_mint_parameter();
                mint_param.policy_id == policy_id
            })
            .collect();

        let is_token_minted = self.token_minted_logic(policy_id, asset_name, quantity);
        let is_only_one_mint = filtered_mints.len() == 1;
        if !is_only_one_mint {
            self.add_trace(
                "policy_only_minted_token",
                &format!(
                    "Expected only one mint for policy_id: {}, but found {} mints.",
                    policy_id,
                    filtered_mints.len()
                ),
            );
        }
        if !is_token_minted {
            self.add_trace(
                "policy_only_minted_token",
                &format!(
                    "Token with policy_id: {}, asset_name: {}, quantity: {} not found in mints.",
                    policy_id, asset_name, quantity
                ),
            );
        }
        self
    }

    /// ## Testing methods for mints
    ///
    /// Checks if a specific policy ID is burned in the transaction, ensuring that it is the only minting (i.e. burning item).
    pub fn check_policy_only_burn(&self, policy_id: &str) -> bool {
        let filtered_mints: Vec<_> = self
            .tx_body
            .mints
            .iter()
            .filter(|token| {
                let mint_param = token.get_mint_parameter();
                mint_param.policy_id == policy_id && mint_param.amount > 0
            })
            .collect();
        filtered_mints.len() == 0
    }

    pub fn token_minted_logic(&self, policy_id: &str, asset_name: &str, quantity: i128) -> bool {
        self.tx_body.mints.iter().any(|token| {
            let mint_param = token.get_mint_parameter();
            mint_param.policy_id == policy_id
                && mint_param.asset_name == asset_name
                && mint_param.amount == quantity
        })
    }
}
