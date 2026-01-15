use pallas::ledger::{addresses::Address, primitives::conway::Tx};
use whisky_common::{
    DatumSource, InlineDatumSource, PubKeyTxIn, ScriptSource, ScriptTxIn, ScriptTxInParameter,
    SimpleScriptTxIn, SimpleScriptTxInParameter, TxIn, TxInParameter, UTxO, WError,
};

use crate::{
    tx_parser::context::{ParserContext, RedeemerIndex, Script},
    wrapper::witness_set::redeemer::RedeemerTag,
};

pub fn extract_inputs(pallas_tx: &Tx, parser_context: &ParserContext) -> Result<Vec<TxIn>, WError> {
    let mut inputs_vec: Vec<TxIn> = Vec::new();
    let inputs = &pallas_tx.transaction_body.inputs;
    for (index, input) in inputs.iter().enumerate() {
        let tx_in = utxo_to_tx_in(input, parser_context, index)?;
        inputs_vec.push(tx_in);
    }
    Ok(inputs_vec)
}

pub fn utxo_to_tx_in(
    tx_input: &pallas::ledger::primitives::conway::TransactionInput,
    context: &ParserContext,
    index: usize,
) -> Result<whisky_common::TxIn, WError> {
    let utxo = context.resolved_utxos.get(tx_input).ok_or_else(|| {
        WError::new(
            "utxo_to_tx_in",
            &format!("Failed to find UTxO for input: {:?}", tx_input),
        )
    })?;

    let tx_in_param = TxInParameter {
        tx_hash: utxo.input.tx_hash.clone(),
        tx_index: utxo.input.output_index,
        amount: Some(utxo.output.amount.clone()),
        address: Some(utxo.output.address.clone()),
    };

    let address = Address::from_bech32(&utxo.output.address).map_err(|e| {
        WError::new(
            "utxo_to_tx_in",
            &format!("Failed to parse address from bech32: {:?}", e),
        )
    })?;

    match address {
        Address::Byron(_byron_address) => todo!(),
        Address::Shelley(shelley_address) => {
            let payment_cred = shelley_address.payment();
            match payment_cred {
                pallas::ledger::addresses::ShelleyPaymentPart::Key(_) => {
                    return Ok(TxIn::PubKeyTxIn(PubKeyTxIn { tx_in: tx_in_param }))
                }
                pallas::ledger::addresses::ShelleyPaymentPart::Script(hash) => {
                    if let Some(script) = context.script_witnesses.scripts.get(&hash.to_string()) {
                        match script {
                            Script::ProvidedNative(native_script) => {
                                return Ok(TxIn::SimpleScriptTxIn(SimpleScriptTxIn {
                                    tx_in: tx_in_param,
                                    simple_script_tx_in:
                                        SimpleScriptTxInParameter::ProvidedSimpleScriptSource(
                                            native_script.clone(),
                                        ),
                                }));
                            }
                            Script::ProvidedPlutus(plutus_script) => {
                                let datum_source = get_datum_for_output(utxo, context);
                                let script_source =
                                    Some(ScriptSource::ProvidedScriptSource(plutus_script.clone()));
                                let redeemer = context
                                    .script_witnesses
                                    .redeemers
                                    .get(&RedeemerIndex {
                                        tag: RedeemerTag::Spend,
                                        index: index as u32,
                                    })
                                    .cloned();

                                return Ok(TxIn::ScriptTxIn(ScriptTxIn {
                                    tx_in: tx_in_param,
                                    script_tx_in: ScriptTxInParameter {
                                        script_source,
                                        datum_source,
                                        redeemer,
                                    },
                                }));
                            }
                            Script::ReferencedNative(inline_script_source) => {
                                return Ok(TxIn::SimpleScriptTxIn(SimpleScriptTxIn {
                                    tx_in: tx_in_param,
                                    simple_script_tx_in:
                                        SimpleScriptTxInParameter::InlineSimpleScriptSource(
                                            inline_script_source.clone(),
                                        ),
                                }))
                            }
                            Script::ReferencedPlutus(inline_script_source) => {
                                let datum_source = get_datum_for_output(utxo, context);

                                let script_source = Some(ScriptSource::InlineScriptSource(
                                    inline_script_source.clone(),
                                ));

                                let redeemer = context
                                    .script_witnesses
                                    .redeemers
                                    .get(&RedeemerIndex {
                                        tag: RedeemerTag::Spend,
                                        index: index as u32,
                                    })
                                    .cloned();

                                return Ok(TxIn::ScriptTxIn(ScriptTxIn {
                                    tx_in: tx_in_param,
                                    script_tx_in: ScriptTxInParameter {
                                        script_source,
                                        datum_source,
                                        redeemer,
                                    },
                                }));
                            }
                        }
                    } else {
                        return Ok(TxIn::PubKeyTxIn(PubKeyTxIn { tx_in: tx_in_param }));
                    }
                }
            }
        }
        Address::Stake(stake_address) => {
            return Err(WError::new(
                "utxo_to_tx_in",
                &format!(
                    "Stake addresses are not supported for inputs: {:?}",
                    stake_address
                ),
            ));
        }
    }
}

fn get_datum_for_output(
    utxo: &UTxO,
    context: &ParserContext,
) -> Option<whisky_common::DatumSource> {
    if let Some(_) = &utxo.output.plutus_data {
        Some(DatumSource::InlineDatumSource(InlineDatumSource {
            tx_hash: utxo.input.tx_hash.clone(),
            tx_index: utxo.input.output_index,
        }))
    } else if let Some(datum_hash) = &utxo.output.data_hash {
        context.script_witnesses.datums.get(datum_hash).cloned()
    } else {
        None
    }
}
