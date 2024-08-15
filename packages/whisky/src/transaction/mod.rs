use cardano_serialization_lib::JsError;

use crate::builder::MeshTxBuilder;

pub mod inputs;
pub mod mints;
pub mod withdrawal;

#[derive(Debug, Clone)]
pub enum WhiskyScriptType {
    Spending,
    Minting,
    Withdrawal,
}

pub struct WhiskyTx {
    pub tx_builder: MeshTxBuilder,
    pub current_script_type: Option<WhiskyScriptType>,
}

#[derive(Debug, Clone)]
pub struct RefScriptInput {
    pub tx_hash: String,
    pub tx_index: u32,
    pub script_hash: String,
    pub script_size: usize,
}

impl WhiskyTx {
    pub fn new() -> Self {
        Self {
            tx_builder: MeshTxBuilder::default(),
            current_script_type: None,
        }
    }

    pub fn provide_script(&mut self, script_cbor: &str) -> Result<&mut Self, JsError> {
        match self.current_script_type {
            Some(WhiskyScriptType::Spending) => self.tx_builder.tx_in_script(script_cbor),
            Some(WhiskyScriptType::Minting) => self.tx_builder.minting_script(script_cbor),
            Some(WhiskyScriptType::Withdrawal) => self.tx_builder.withdrawal_script(script_cbor),
            None => return Err(JsError::from_str("No script type can be inferred, script must be provided after an indicating apis: unlock_from_script, mint_assets, withdraw_from_script")),
        };
        Ok(self)
    }

    pub fn inline_script(
        &mut self,
        tx_hash: &str,
        tx_index: u32,
        script_hash: &str,
        script_size: usize,
    ) -> Result<&mut Self, JsError> {
        match self.current_script_type {
            Some(WhiskyScriptType::Spending) => self.tx_builder.spending_tx_in_reference(
                tx_hash,
                tx_index,
                script_hash,
                script_size,
            ),
            Some(WhiskyScriptType::Minting) => {
                self.tx_builder
                    .mint_tx_in_reference(tx_hash, tx_index, script_hash, script_size)
            }
            Some(WhiskyScriptType::Withdrawal) => self.tx_builder.withdrawal_tx_in_reference(
                tx_hash,
                tx_index,
                script_hash,
                script_size,
            ),
            None => return Err(JsError::from_str("No script type can be inferred, script must be provided after an indicating apis: unlock_from_script, mint_assets, withdraw_from_script")),
        };
        Ok(self)
    }

    pub async fn build(&mut self) -> Result<String, JsError> {
        self.tx_builder.complete(None).await?;
        Ok(self.tx_builder.tx_hex())
    }
}

impl Default for WhiskyTx {
    fn default() -> Self {
        Self::new()
    }
}
