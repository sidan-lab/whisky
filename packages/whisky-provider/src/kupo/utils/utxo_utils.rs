use whisky_common::{Asset, UTxO, UtxoInput, UtxoOutput, WError};

use crate::kupo::{
    models::{KupoUtxo, KupoValue},
    KupoProvider,
};

impl KupoProvider {
    pub async fn to_utxo(&self, utxo: &KupoUtxo) -> Result<UTxO, WError> {
        let utxo = UTxO {
            input: UtxoInput {
                output_index: utxo.output_index as u32,
                tx_hash: utxo.tx_hash.clone(),
            },
            output: UtxoOutput {
                address: utxo.address.clone(),
                amount: self.to_amount(&utxo.value)?,
                data_hash: utxo.datum.clone(),
                plutus_data: utxo.datum.clone(),
                script_ref: None,
                script_hash: utxo.script_hash.clone(),
            },
        };
        Ok(utxo)
    }

    pub fn to_amount(&self, value: &KupoValue) -> Result<Vec<Asset>, WError> {
        let mut amount = Vec::new();
        amount.push(Asset::new("lovelace".to_string(), value.coins.to_string()));

        for (unit, quantity) in &value.assets {
            let asset = unit.split(".").collect::<Vec<_>>();
            let (policy_id, asset_name) = if asset.len() == 2 {
                (asset[0].to_string(), asset[1].to_string())
            } else {
                (asset[0].to_string(), "".to_string())
            };
            amount.push(Asset::new(
                format!("{}{}", policy_id, asset_name),
                quantity.to_string(),
            ));
        }
        Ok(amount)
    }
}
