use cardano_serialization_lib::{self as csl};
use pallas_codec::minicbor::data::Tag;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use whisky_common::{
    Budget, DatumSource, InlineDatumSource, InlineScriptSource, InlineSimpleScriptSource,
    LanguageVersion, ProvidedDatumSource, ProvidedScriptSource, ProvidedSimpleScriptSource,
    Redeemer, RefTxIn, UTxO, UtxoInput, WError,
};

use pallas_codec::minicbor::Encoder;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptWitness {
    pub datums: HashMap<String, DatumSource>,
    pub redeemers: HashMap<RedeemerIndex, Redeemer>,
    pub scripts: HashMap<csl::ScriptHash, Script>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParserContext {
    pub resolved_utxos: HashMap<csl::TransactionInput, UTxO>,
    pub script_witness: ScriptWitness,
}

impl ParserContext {
    pub fn new() -> Self {
        Self {
            resolved_utxos: HashMap::new(),
            script_witness: ScriptWitness {
                datums: HashMap::new(),
                redeemers: HashMap::new(),
                scripts: HashMap::new(),
            },
        }
    }

    pub fn fill_resolved_utxos(
        &mut self,
        tx_body: &csl::TransactionBody,
        resolved_utxos: &[UTxO],
    ) -> Result<(), String> {
        let inputs = tx_body.inputs();
        let ref_inputs = tx_body.reference_inputs();
        let collateral_inputs = tx_body.collateral();
        let all_transaction_inputs: Vec<csl::TransactionInput> = inputs
            .into_iter()
            .chain(
                ref_inputs
                    .unwrap_or(csl::TransactionInputs::new())
                    .into_iter(),
            )
            .chain(
                collateral_inputs
                    .unwrap_or(csl::TransactionInputs::new())
                    .into_iter(),
            )
            .map(|input| input.clone())
            .collect();

        let utxo_map = resolved_utxos
            .iter()
            .map(|utxo| (utxo.input.clone(), utxo.clone()))
            .collect::<HashMap<UtxoInput, UTxO>>();

        for input in all_transaction_inputs {
            let tx_hash = input.transaction_id().to_hex();
            let index = input.index();
            let utxo = utxo_map.get(&UtxoInput {
                tx_hash,
                output_index: index,
            });
            if let Some(utxo) = utxo {
                self.resolved_utxos.insert(input, utxo.clone());
            }
        }
        Ok(())
    }

    pub fn collect_script_witnesses_from_tx_witnesses_set(
        &mut self,
        tx_witnesses_set: csl::TransactionWitnessSet,
    ) -> Result<(), String> {
        let mut datums = HashMap::new();
        let mut redeemers = HashMap::new();
        let mut scripts = HashMap::new();

        if let Some(plutus_data_list) = tx_witnesses_set.plutus_data() {
            for i in 0..plutus_data_list.len() {
                let plutus_data = plutus_data_list.get(i);
                let data_hash = csl::hash_plutus_data(&plutus_data).to_hex();
                let datum_source = DatumSource::ProvidedDatumSource(ProvidedDatumSource {
                    data: plutus_data.to_hex(),
                });
                datums.insert(data_hash, datum_source);
            }
        }

        if let Some(redeemer_list) = tx_witnesses_set.redeemers() {
            for i in 0..redeemer_list.len() {
                let redeemer = redeemer_list.get(i);
                let tag = redeemer.tag().kind();
                let index = redeemer
                    .index()
                    .to_string()
                    .parse::<usize>()
                    .map_err(|e| format!("Failed to parse redeemer index: {:?}", e))?;
                let redeemer_index = match tag {
                    csl::RedeemerTagKind::Spend => RedeemerIndex::Spend(index),
                    csl::RedeemerTagKind::Mint => RedeemerIndex::Mint(index),
                    csl::RedeemerTagKind::Cert => RedeemerIndex::Cert(index),
                    csl::RedeemerTagKind::Reward => RedeemerIndex::Reward(index),
                    csl::RedeemerTagKind::Vote => RedeemerIndex::Vote(index),
                    csl::RedeemerTagKind::VotingProposal => RedeemerIndex::VotingProposal(index),
                };
                let whisky_redeemer = csl_redeemer_to_redeemer(redeemer);
                redeemers.insert(redeemer_index, whisky_redeemer);
            }
        }

        if let Some(native_scripts) = tx_witnesses_set.native_scripts() {
            for i in 0..native_scripts.len() {
                let script = native_scripts.get(i);
                let script_hash = script.hash();
                scripts.insert(
                    script_hash,
                    Script::ProvidedNative(csl_native_script_to_native_script(script)),
                );
            }
        }

        if let Some(plutus_scripts) = tx_witnesses_set.plutus_scripts() {
            for i in 0..plutus_scripts.len() {
                let script = plutus_scripts.get(i);
                let script_hash = script.hash();
                scripts.insert(
                    script_hash,
                    Script::ProvidedPlutus(csl_plutus_script_to_script(script)),
                );
            }
        }

        self.script_witness.datums.extend(datums);
        self.script_witness.redeemers.extend(redeemers);
        self.script_witness.scripts.extend(scripts);
        Ok(())
    }

    pub fn collect_script_witnesses_from_tx_body(
        &mut self,
        tx_body: csl::TransactionBody,
    ) -> Result<(), WError> {
        let inputs = tx_body.inputs();
        let ref_inputs = tx_body.reference_inputs();
        let all_transaction_inputs: Vec<csl::TransactionInput> = inputs
            .into_iter()
            .chain(
                ref_inputs
                    .unwrap_or(csl::TransactionInputs::new())
                    .into_iter(),
            )
            .map(|input| input.clone())
            .collect();

        for input in all_transaction_inputs {
            let utxo = self.resolved_utxos.get(&input).unwrap();
            let (simple_script_source, plutus_script_source, datum_source) =
                utxo_to_inline_sources(utxo).map_err(WError::from_err("utxo_to_inline_sources"))?;
            if let Some((datum_source, datum_hash)) = datum_source {
                self.script_witness
                    .datums
                    .insert(datum_hash, DatumSource::InlineDatumSource(datum_source));
            }
            if let Some((script_source, script_hash)) = simple_script_source {
                self.script_witness
                    .scripts
                    .insert(script_hash, Script::ReferencedNative(script_source));
            }
            if let Some((script_source, script_hash)) = plutus_script_source {
                self.script_witness
                    .scripts
                    .insert(script_hash, Script::ReferencedPlutus(script_source));
            }
        }

        Ok(())
    }
}

impl ScriptWitness {
    pub fn new() -> Self {
        Self {
            datums: HashMap::new(),
            redeemers: HashMap::new(),
            scripts: HashMap::new(),
        }
    }

    pub fn extend(&mut self, other: ScriptWitness) {
        self.datums.extend(other.datums);
        self.redeemers.extend(other.redeemers);
        self.scripts.extend(other.scripts);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RedeemerIndex {
    Spend(usize),
    Mint(usize),
    Cert(usize),
    Reward(usize),
    Vote(usize),
    VotingProposal(usize),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Script {
    ProvidedNative(ProvidedSimpleScriptSource),
    ProvidedPlutus(ProvidedScriptSource),
    ReferencedNative(InlineSimpleScriptSource),
    ReferencedPlutus(InlineScriptSource),
}

fn csl_redeemer_to_redeemer(redeemer: csl::Redeemer) -> Redeemer {
    Redeemer {
        data: redeemer.data().to_hex(),
        ex_units: Budget {
            mem: redeemer.ex_units().mem().to_str().parse::<u64>().unwrap(),
            steps: redeemer.ex_units().steps().to_str().parse::<u64>().unwrap(),
        },
    }
}

fn csl_plutus_script_to_script(script: csl::PlutusScript) -> ProvidedScriptSource {
    ProvidedScriptSource {
        script_cbor: script.to_hex(),
        language_version: csl_language_version_to_language_version(
            script.language_version().kind(),
        ),
    }
}

fn csl_native_script_to_native_script(script: csl::NativeScript) -> ProvidedSimpleScriptSource {
    ProvidedSimpleScriptSource {
        script_cbor: script.to_hex(),
    }
}

fn utxo_to_inline_sources(
    utxo: &UTxO,
) -> Result<
    (
        Option<(InlineSimpleScriptSource, csl::ScriptHash)>,
        Option<(InlineScriptSource, csl::ScriptHash)>,
        Option<(InlineDatumSource, String)>,
    ),
    String,
> {
    let csl_script_ref = if let Some(script_ref) = &utxo.output.script_ref {
        Some(normalize_script_ref(script_ref, &utxo.input)?)
    } else {
        None
    };

    let script_size = utxo
        .output
        .script_ref
        .as_ref()
        .map_or(0, |script_ref| script_ref.len() / 2);

    let ref_tx_in = RefTxIn {
        tx_hash: utxo.input.tx_hash.clone(),
        tx_index: utxo.input.output_index,
        script_size: Some(script_size),
    };

    let simple_script_source = if let Some(ref csl_script_ref) = csl_script_ref {
        if csl_script_ref.is_native_script() {
            let simple_script_hash = get_script_hash_from_script_ref(csl_script_ref);
            Some((
                InlineSimpleScriptSource {
                    ref_tx_in: ref_tx_in.clone(),
                    simple_script_hash: simple_script_hash.to_hex(),
                    script_size,
                },
                simple_script_hash,
            ))
        } else {
            None
        }
    } else {
        None
    };

    let plutus_script_source = if let Some(ref csl_script_ref) = csl_script_ref {
        if csl_script_ref.is_plutus_script() {
            let plutus_script = csl_script_ref.plutus_script().unwrap();

            let script_hash = get_script_hash_from_script_ref(csl_script_ref);
            Some((
                InlineScriptSource {
                    ref_tx_in: ref_tx_in.clone(),
                    script_hash: script_hash.to_hex(),
                    script_size,
                    language_version: csl_language_version_to_language_version(
                        plutus_script.language_version().kind(),
                    ),
                },
                script_hash,
            ))
        } else {
            None
        }
    } else {
        None
    };

    let datum_source = if let Some(datum) = &utxo.output.plutus_data {
        let data_hash = get_datum_hash_from_datum(datum, &utxo.output.data_hash)?;
        Some((
            InlineDatumSource {
                tx_hash: utxo.input.tx_hash.clone(),
                tx_index: utxo.input.output_index,
            },
            data_hash.to_hex(),
        ))
    } else {
        None
    };

    Ok((simple_script_source, plutus_script_source, datum_source))
}

fn get_script_hash_from_script_ref(script_ref: &csl::ScriptRef) -> csl::ScriptHash {
    if let Some(plutus_script) = script_ref.plutus_script() {
        plutus_script.hash()
    } else {
        script_ref.native_script().unwrap().hash()
    }
}

fn get_datum_hash_from_datum(
    datum: &String,
    datum_hash: &Option<String>,
) -> Result<csl::DataHash, String> {
    if let Some(datum_hash) = datum_hash {
        csl::DataHash::from_hex(datum_hash)
            .map_err(|e| format!("Failed to parse datum hash: {:?}", e))
    } else {
        let datum = csl::PlutusData::from_hex(datum)
            .map_err(|e| format!("Failed to parse datum: {:?}", e))?;
        Ok(csl::hash_plutus_data(&datum))
    }
}

fn csl_language_version_to_language_version(
    language_version: csl::LanguageKind,
) -> LanguageVersion {
    match language_version {
        csl::LanguageKind::PlutusV1 => LanguageVersion::V1,
        csl::LanguageKind::PlutusV2 => LanguageVersion::V2,
        csl::LanguageKind::PlutusV3 => LanguageVersion::V3,
    }
}

fn normalize_script_ref(
    script_ref: &String,
    tx_input: &UtxoInput,
) -> Result<csl::ScriptRef, String> {
    if script_ref.starts_with("82") {
        let bytes = hex::decode(script_ref.clone())
            .map_err(|e| format!("Failed to decode script ref hex: {:?}", e))?;
        let mut encoder = Encoder::new(Vec::new());
        encoder
            .tag(Tag::new(24))
            .map_err(|_| "Failed to write tag")?;
        encoder
            .bytes(&bytes)
            .map_err(|e| format!("Failed to encode script ref bytes: {:?}", e))?;
        let write_buffer = encoder.writer().clone();
        csl::ScriptRef::from_bytes(write_buffer)
            .map_err(|e| format!("Failed to decode script ref hex: {:?}", e))
    } else {
        csl::ScriptRef::from_hex(&script_ref).map_err(|e| {
            format!(
                "Failed to parse script ref: {:?} - {}#{} - with ref: {:?}",
                e, tx_input.tx_hash, tx_input.output_index, script_ref
            )
        })
    }
}
