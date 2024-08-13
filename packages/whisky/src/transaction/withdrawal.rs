use cardano_serialization_lib::JsError;
use sidan_csl_rs::model::LanguageVersion;

use crate::builder::WRedeemer;

use super::{WhiskyScriptType, WhiskyTx};

impl WhiskyTx {
    pub fn withdraw_from_script(
        &mut self,
        language_version: &LanguageVersion,
        stake_address: &str,
        withdrawal_amount: u64,
        redeemer: &WRedeemer,
    ) -> Result<&mut Self, JsError> {
        self.tx_builder
            .withdrawal_plutus_script(language_version)
            .withdrawal(stake_address, withdrawal_amount)
            .withdrawal_redeemer_value(redeemer);
        self.current_script_type = Some(WhiskyScriptType::Withdrawal);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use sidan_csl_rs::{core::common::con_str0, model::Budget, model::LanguageVersion};

    use crate::builder::{WData, WRedeemer};

    use super::*;

    fn test_fn(tx: &mut WhiskyTx) -> Result<&mut WhiskyTx, JsError> {
        let res = tx
            .withdraw_from_script(
                &LanguageVersion::V2,
                "stake_address",
                123,
                &WRedeemer {
                    ex_units: Budget::default(),
                    data: WData::JSON(con_str0(json!([])).to_string()),
                },
            )?
            .provide_script("123")?;
        Ok(res)
    }

    #[test]
    fn test_whisky_tx() {
        let mut whisky_tx = WhiskyTx::new();
        let res = test_fn(&mut whisky_tx);
        match res {
            Ok(tx) => println!("{:?}", tx.tx_builder.tx_hex()),
            Err(e) => panic!("{:?}", e),
        }
    }
}
