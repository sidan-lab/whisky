use std::{collections::HashMap, hash::Hash};

use pallas::{
    codec::utils::Set,
    ledger::{
        primitives::{
            conway::{TransactionBody, WitnessSet},
            Fragment, TransactionInput,
        },
        traverse::ComputeHash,
    },
};
use whisky_common::{
    DatumSource, InlineDatumSource, InlineScriptSource, InlineSimpleScriptSource,
    ProvidedDatumSource, ProvidedScriptSource, ProvidedSimpleScriptSource, Redeemer, UTxO, WError,
};

use crate::wrapper::{
    transaction_body::{Datum, DatumKind, ScriptRef},
    witness_set::redeemer::RedeemerTag,
};

#[derive(Debug, Clone)]
pub enum Script {
    ProvidedNative(ProvidedSimpleScriptSource),
    ProvidedPlutus(ProvidedScriptSource),
    ReferencedNative(InlineSimpleScriptSource),
    ReferencedPlutus(InlineScriptSource),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RedeemerIndex {
    pub tag: RedeemerTag,
    pub index: u32,
}

#[derive(Debug, Clone)]
pub struct ScriptWitness {
    pub datums: HashMap<String, DatumSource>,
    pub redeemers: HashMap<RedeemerIndex, Redeemer>,
    pub scripts: HashMap<String, Script>,
}

pub struct ParserContext {
    pub resolved_utxos: HashMap<TransactionInput, UTxO>,
    pub script_witnesses: ScriptWitness,
}

impl ParserContext {
    pub fn new() -> Self {
        ParserContext {
            resolved_utxos: HashMap::new(),
            script_witnesses: ScriptWitness {
                datums: HashMap::new(),
                redeemers: HashMap::new(),
                scripts: HashMap::new(),
            },
        }
    }

    pub fn fill_resolved_utxos(
        &mut self,
        tx_body: TransactionBody,
        resolved_utxos: &[UTxO],
    ) -> Result<(), WError> {
        let inputs = tx_body.inputs;
        let ref_inputs = tx_body.reference_inputs;
        let collateral_inputs = tx_body.collateral;
        let mut collected_inputs: Vec<TransactionInput> = Vec::new();

        for input in inputs.iter() {
            collected_inputs.push(input.clone());
        }
        match ref_inputs {
            Some(ref_input) => {
                for input in ref_input.iter() {
                    collected_inputs.push(input.clone());
                }
            }
            _ => {}
        }
        match collateral_inputs {
            Some(collateral_input) => {
                for input in collateral_input.iter() {
                    collected_inputs.push(input.clone());
                }
            }
            _ => {}
        }

        let collected_input_set = Set::from(collected_inputs);

        for collected_input in collected_input_set.iter() {
            self.resolved_utxos.insert(
                collected_input.clone(),
                resolved_utxos
                    .iter()
                    .find(|utxo| {
                        collected_input.transaction_id.to_string() == utxo.input.tx_hash
                            && collected_input.index as u32 == utxo.input.output_index
                    })
                    .ok_or_else(|| {
                        WError::new(
                            "WhiskyPallas - ParserContext - fill_resolved_utxos:",
                            &format!(
                                "UTxO not found for input: {}#{}",
                                collected_input.transaction_id.to_string(),
                                collected_input.index
                            ),
                        )
                    })?
                    .clone(),
            );
        }
        Ok(())
    }

    pub fn collect_script_witnesses_from_tx_witnesses_set(
        &mut self,
        witness_set: WitnessSet,
    ) -> Result<(), WError> {
        let datums = witness_set.plutus_data;
        let redeemers = witness_set.redeemer;
        let plutus_v1_scripts = witness_set.plutus_v1_script;
        let plutus_v2_scripts = witness_set.plutus_v2_script;
        let plutus_v3_scripts = witness_set.plutus_v3_script;
        let native_scripts = witness_set.native_script;

        match datums {
            Some(datum) => {
                for data in datum.iter() {
                    self.script_witnesses.datums.insert(
                        data.compute_hash().to_string(),
                        DatumSource::ProvidedDatumSource(ProvidedDatumSource {
                            data: hex::encode(data.encode_fragment().map_err(|_| {
                                WError::new("Whisky Pallas Parser - ", "Error parsing datum source")
                            })?),
                        }),
                    );
                }
            }
            None => {}
        }

        match redeemers {
            Some(redeemer_set) => match redeemer_set.unwrap() {
                pallas::ledger::primitives::conway::Redeemers::List(redeemers) => {
                    for redeemer in redeemers {
                        let tag = match redeemer.tag {
                            pallas::ledger::primitives::conway::RedeemerTag::Spend => {
                                RedeemerTag::Spend
                            }
                            pallas::ledger::primitives::conway::RedeemerTag::Mint => {
                                RedeemerTag::Mint
                            }
                            pallas::ledger::primitives::conway::RedeemerTag::Cert => {
                                RedeemerTag::Cert
                            }
                            pallas::ledger::primitives::conway::RedeemerTag::Reward => {
                                RedeemerTag::Reward
                            }
                            pallas::ledger::primitives::conway::RedeemerTag::Vote => {
                                RedeemerTag::Vote
                            }
                            pallas::ledger::primitives::conway::RedeemerTag::Propose => {
                                RedeemerTag::Propose
                            }
                        };
                        self.script_witnesses.redeemers.insert(
                            RedeemerIndex {
                                tag,
                                index: redeemer.index,
                            },
                            Redeemer {
                                data: hex::encode(redeemer.data.encode_fragment().map_err(
                                    |_| {
                                        WError::new(
                                            "Whisky Pallas Parser - ",
                                            "Error parsing redeemer source",
                                        )
                                    },
                                )?),
                                ex_units: whisky_common::Budget {
                                    mem: redeemer.ex_units.mem as u64,
                                    steps: redeemer.ex_units.steps as u64,
                                },
                            },
                        );
                    }
                }
                pallas::ledger::primitives::conway::Redeemers::Map(redeemer_map) => {
                    for (key, value) in redeemer_map.iter() {
                        let redeemer_index = match key.tag {
                            pallas::ledger::primitives::conway::RedeemerTag::Spend => {
                                RedeemerIndex {
                                    tag: RedeemerTag::Spend,
                                    index: key.index,
                                }
                            }
                            pallas::ledger::primitives::conway::RedeemerTag::Mint => {
                                RedeemerIndex {
                                    tag: RedeemerTag::Mint,
                                    index: key.index,
                                }
                            }
                            pallas::ledger::primitives::conway::RedeemerTag::Cert => {
                                RedeemerIndex {
                                    tag: RedeemerTag::Cert,
                                    index: key.index,
                                }
                            }
                            pallas::ledger::primitives::conway::RedeemerTag::Reward => {
                                RedeemerIndex {
                                    tag: RedeemerTag::Reward,
                                    index: key.index,
                                }
                            }
                            pallas::ledger::primitives::conway::RedeemerTag::Vote => {
                                RedeemerIndex {
                                    tag: RedeemerTag::Vote,
                                    index: key.index,
                                }
                            }
                            pallas::ledger::primitives::conway::RedeemerTag::Propose => {
                                RedeemerIndex {
                                    tag: RedeemerTag::Propose,
                                    index: key.index,
                                }
                            }
                        };
                        let data = value.data.encode_fragment().map_err(|_| {
                            WError::new("Whisky Pallas Parser - ", "Error parsing redeemer source")
                        })?;
                        let ex_units = whisky_common::Budget {
                            mem: value.ex_units.mem as u64,
                            steps: value.ex_units.steps as u64,
                        };
                        self.script_witnesses.redeemers.insert(
                            redeemer_index,
                            Redeemer {
                                data: hex::encode(data),
                                ex_units,
                            },
                        );
                    }
                }
            },
            None => {}
        }

        match plutus_v1_scripts {
            Some(scripts) => {
                for script in scripts.iter() {
                    self.script_witnesses.scripts.insert(
                        script.compute_hash().to_string(),
                        Script::ProvidedPlutus(ProvidedScriptSource {
                            script_cbor: hex::encode(script.encode_fragment().map_err(|_| {
                                WError::new(
                                    "Whisky Pallas Parser - ",
                                    "Error parsing plutus v1 script source",
                                )
                            })?),
                            language_version: whisky_common::LanguageVersion::V1,
                        }),
                    );
                }
            }
            None => {}
        }

        match plutus_v2_scripts {
            Some(scripts) => {
                for script in scripts.iter() {
                    self.script_witnesses.scripts.insert(
                        script.compute_hash().to_string(),
                        Script::ProvidedPlutus(ProvidedScriptSource {
                            script_cbor: hex::encode(script.encode_fragment().map_err(|_| {
                                WError::new(
                                    "Whisky Pallas Parser - ",
                                    "Error parsing plutus v2 script source",
                                )
                            })?),
                            language_version: whisky_common::LanguageVersion::V2,
                        }),
                    );
                }
            }
            None => {}
        }

        match plutus_v3_scripts {
            Some(scripts) => {
                for script in scripts.iter() {
                    self.script_witnesses.scripts.insert(
                        script.compute_hash().to_string(),
                        Script::ProvidedPlutus(ProvidedScriptSource {
                            script_cbor: hex::encode(script.encode_fragment().map_err(|_| {
                                WError::new(
                                    "Whisky Pallas Parser - ",
                                    "Error parsing plutus v3 script source",
                                )
                            })?),
                            language_version: whisky_common::LanguageVersion::V3,
                        }),
                    );
                }
            }
            None => {}
        }

        match native_scripts {
            Some(scripts) => {
                for script in scripts.iter() {
                    self.script_witnesses.scripts.insert(
                        script.compute_hash().to_string(),
                        Script::ProvidedNative(ProvidedSimpleScriptSource {
                            script_cbor: hex::encode(script.encode_fragment().map_err(|_| {
                                WError::new(
                                    "Whisky Pallas Parser - ",
                                    "Error parsing native script source",
                                )
                            })?),
                        }),
                    );
                }
            }
            None => {}
        }
        Ok(())
    }

    pub fn collect_script_witnesses_from_tx_body(
        &mut self,
        tx_body: TransactionBody,
    ) -> Result<(), WError> {
        let inputs = tx_body.inputs;
        let ref_inputs = tx_body.reference_inputs;
        let collateral_inputs = tx_body.collateral;
        let mut collected_inputs: Vec<TransactionInput> = Vec::new();

        for input in inputs.iter() {
            collected_inputs.push(input.clone());
        }
        match ref_inputs {
            Some(ref_input) => {
                for input in ref_input.iter() {
                    collected_inputs.push(input.clone());
                }
            }
            _ => {}
        }
        match collateral_inputs {
            Some(collateral_input) => {
                for input in collateral_input.iter() {
                    collected_inputs.push(input.clone());
                }
            }
            _ => {}
        }

        let collected_input_set = Set::from(collected_inputs);

        for input in collected_input_set.iter() {
            let utxo_option = self.resolved_utxos.get(input);
            match utxo_option {
                Some(utxo) => {}
                None => {
                    return Err(WError::new(
                        "WhiskyPallas - ParserContext - collect_script_witnesses_from_tx_body:",
                        &format!(
                            "UTxO not found for input: {}#{}",
                            input.transaction_id.to_string(),
                            input.index
                        ),
                    ));
                }
            }
        }
        Ok(())
    }
}

fn utxo_to_inline_sources(
    utxo: &UTxO,
) -> Result<(Option<(String, DatumSource)>, Option<(String, Script)>), WError> {
    let datum_option: Option<(String, DatumSource)> = match &utxo.output.plutus_data {
        Some(inline_datum) => {
            let pallas_datum = Datum::new(DatumKind::Data {
                plutus_data_hex: inline_datum.to_string(),
            })?;
            let datum_hash = pallas_datum.hash()?;
            Some((
                datum_hash,
                DatumSource::InlineDatumSource(InlineDatumSource {
                    tx_hash: utxo.input.tx_hash.clone(),
                    tx_index: utxo.input.output_index,
                }),
            ))
        }
        None => None,
    };
    let script_option: Option<(String, Script)> = match &utxo.output.script_ref {
        Some(script_ref) => {
            let script_bytes = hex::decode(script_ref).map_err(|_| {
                WError::new("Whisky Pallas Parser - ", "Error decoding script_ref hex")
            })?;
            let pallas_script_ref = ScriptRef::decode_bytes(&script_bytes)
                .map_err(|_| WError::new("Whisky Pallas Parser - ", "Error decoding script ref"))?;
            match pallas_script_ref.inner {
                pallas::ledger::primitives::conway::ScriptRef::NativeScript(native_script) => {
                    Some((
                        native_script.compute_hash().to_string(),
                        Script::ProvidedNative(ProvidedSimpleScriptSource {
                            script_cbor: hex::encode(native_script.encode_fragment().map_err(
                                |_| {
                                    WError::new(
                                        "Whisky Pallas Parser - ",
                                        "Error parsing native script source from script ref",
                                    )
                                },
                            )?),
                        }),
                    ))
                }
                pallas::ledger::primitives::conway::ScriptRef::PlutusV1Script(plutus_script) => {
                    Some((
                        plutus_script.compute_hash().to_string(),
                        Script::ProvidedPlutus(ProvidedScriptSource {
                            script_cbor: hex::encode(plutus_script.encode_fragment().map_err(
                                |_| {
                                    WError::new(
                                        "Whisky Pallas Parser - ",
                                        "Error parsing plutus v1 script source from script ref",
                                    )
                                },
                            )?),
                            language_version: whisky_common::LanguageVersion::V1,
                        }),
                    ))
                }
                pallas::ledger::primitives::conway::ScriptRef::PlutusV2Script(plutus_script) => {
                    Some((
                        plutus_script.compute_hash().to_string(),
                        Script::ProvidedPlutus(ProvidedScriptSource {
                            script_cbor: hex::encode(plutus_script.encode_fragment().map_err(
                                |_| {
                                    WError::new(
                                        "Whisky Pallas Parser - ",
                                        "Error parsing plutus v2 script source from script ref",
                                    )
                                },
                            )?),
                            language_version: whisky_common::LanguageVersion::V2,
                        }),
                    ))
                }
                pallas::ledger::primitives::conway::ScriptRef::PlutusV3Script(plutus_script) => {
                    Some((
                        plutus_script.compute_hash().to_string(),
                        Script::ProvidedPlutus(ProvidedScriptSource {
                            script_cbor: hex::encode(plutus_script.encode_fragment().map_err(
                                |_| {
                                    WError::new(
                                        "Whisky Pallas Parser - ",
                                        "Error parsing plutus v3 script source from script ref",
                                    )
                                },
                            )?),
                            language_version: whisky_common::LanguageVersion::V3,
                        }),
                    ))
                }
            }
        }
        None => None,
    };
    Ok((datum_option, script_option))
}
