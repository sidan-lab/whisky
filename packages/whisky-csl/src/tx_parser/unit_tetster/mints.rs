use super::TxTester;

impl TxTester {
    pub fn token_minted(&self, policy_id: &str, asset_name: &str, quantity: i128) -> bool {
        self.tx_body.mints.iter().any(|token| {
            let mint_param = token.get_mint_parameter();
            mint_param.policy_id == policy_id
                && mint_param.asset_name == asset_name
                && mint_param.amount == quantity
        })
    }

    pub fn only_token_minted(&self, policy_id: &str, asset_name: &str, quantity: i128) -> bool {
        self.tx_body.mints.len() == 1 && self.token_minted(policy_id, asset_name, quantity)
    }

    pub fn policy_only_minted_token(
        &self,
        policy_id: &str,
        asset_name: &str,
        quantity: i128,
    ) -> bool {
        let filtered_mints: Vec<_> = self
            .tx_body
            .mints
            .iter()
            .filter(|token| {
                let mint_param = token.get_mint_parameter();
                mint_param.policy_id == policy_id
            })
            .collect();
        filtered_mints.len() == 1 && self.token_minted(policy_id, asset_name, quantity)
    }

    pub fn check_policy_only_burn(&self, policy_id: &str) -> bool {
        let filtered_mints: Vec<_> = self
            .tx_body
            .mints
            .iter()
            .filter(|token| {
                let mint_param = token.get_mint_parameter();
                mint_param.policy_id == policy_id && mint_param.amount < 0
            })
            .collect();
        filtered_mints.len() == 1
    }
}
