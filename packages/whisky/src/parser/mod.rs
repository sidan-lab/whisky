use std::collections::{HashMap, HashSet};

use whisky_common::{Fetcher, TxBuilderBody, TxParsable, TxTester, UTxO, WError};
use whisky_csl::WhiskyCSL;

pub struct TxParser {
    pub fetcher: Option<Box<dyn Fetcher>>,
    pub serializer: Box<dyn TxParsable>,
}

impl TxParser {
    pub fn new(fetcher: Option<Box<dyn Fetcher>>) -> Self {
        TxParser {
            fetcher: fetcher,
            serializer: Box::new(WhiskyCSL::new(None).unwrap()),
        }
    }

    pub async fn parse(
        &mut self,
        tx_hex: &str,
        provided_utxos: &[UTxO],
    ) -> Result<&mut Self, WError> {
        let mut resolved_utxos = provided_utxos.to_vec();

        // Create a set of already provided UTxOs for quick lookup
        let resolved_utxos_set: HashSet<String> = provided_utxos
            .iter()
            .map(|utxo| format!("{}#{}", utxo.input.tx_hash, utxo.input.output_index))
            .collect();

        let mut to_resolve_utxos: HashMap<String, Vec<u32>> = HashMap::new();

        // Get required inputs from the transaction
        let required_inputs = self.serializer.get_required_inputs(tx_hex)?;

        // Find missing UTxOs that need to be resolved
        for input in required_inputs {
            let utxo_key = format!("{}#{}", input.tx_hash, input.output_index);
            if !resolved_utxos_set.contains(&utxo_key) {
                to_resolve_utxos
                    .entry(input.tx_hash.clone())
                    .or_insert_with(Vec::new)
                    .push(input.output_index);
            }
        }

        // Fetch missing UTxOs
        for (tx_hash, output_indices) in to_resolve_utxos {
            let fetcher = self.fetcher.as_ref().ok_or_else(|| {
                WError::from_err("TxParser - parse")(
                    "Fetcher is not provided. Cannot resolve UTxOs without fetcher.",
                )
            })?;

            let utxos = fetcher
                .fetch_utxos(&tx_hash, None)
                .await
                .map_err(WError::from_err("TxParser - parse"))?;

            for output_index in output_indices {
                let utxo_data = utxos
                    .iter()
                    .find(|utxo| utxo.input.output_index == output_index)
                    .ok_or_else(|| {
                        WError::from_err("TxParser - parse")(format!(
                            "UTxO not found: {}:{}",
                            tx_hash, output_index
                        ))
                    })?;

                resolved_utxos.push(utxo_data.clone());
            }
        }

        self.serializer
            .parse(tx_hex, &resolved_utxos)
            .map_err(WError::from_err("TxParser - parse"))?;
        Ok(self)
    }

    pub fn to_tester(&self) -> TxTester {
        self.serializer.to_tester()
    }

    pub fn get_builder_body(&self) -> TxBuilderBody {
        self.serializer.get_builder_body()
    }

    pub fn get_builder_body_without_change(&self) -> TxBuilderBody {
        self.serializer.get_builder_body_without_change()
    }
}
