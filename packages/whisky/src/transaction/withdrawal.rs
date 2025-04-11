use crate::*;

use crate::builder::WRedeemer;

use super::{WhiskyScriptType, WhiskyTx};

impl WhiskyTx {
    pub fn withdraw_from_script(
        &mut self,
        language_version: &LanguageVersion,
        stake_address: &str,
        withdrawal_amount: u64,
        redeemer: &WRedeemer,
    ) -> Result<&mut Self, WError> {
        self.tx_builder
            .withdrawal_plutus_script(language_version)
            .withdrawal(stake_address, withdrawal_amount)
            .withdrawal_redeemer_value(redeemer);
        self.current_script_type = Some(WhiskyScriptType::Withdrawal);
        Ok(self)
    }
}
