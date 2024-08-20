use sidan_csl_rs::{
    csl::JsError,
    model::{LanguageVersion, MintParameter},
};

use crate::builder::WRedeemer;

use super::{WhiskyScriptType, WhiskyTx};

impl WhiskyTx {
    pub fn mint_asset(
        &mut self,
        language_version: &LanguageVersion,
        mint_param: &MintParameter,
        redeemer: &WRedeemer,
    ) -> Result<&mut Self, JsError> {
        self.tx_builder
            .mint_plutus_script(language_version)
            .mint(
                mint_param.amount,
                &mint_param.policy_id,
                &mint_param.asset_name,
            )
            .mint_redeemer_value(redeemer);
        self.current_script_type = Some(WhiskyScriptType::Minting);
        Ok(self)
    }
}
