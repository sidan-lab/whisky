use whisky_common::{
    DatumSource, InlineDatumSource, PubKeyTxIn, RefTxIn, ScriptSource, ScriptTxIn,
    ScriptTxInParameter, SimpleScriptTxIn, SimpleScriptTxInParameter, TxIn, TxInParameter, UTxO,
    WError,
};

use cardano_serialization_lib as csl;

use super::context::{ParserContext, RedeemerIndex, Script};

pub fn utxo_to_ref_tx_in(
    tx_input: &csl::TransactionInput,
    context: &ParserContext,
) -> Result<RefTxIn, WError> {
    let utxo = context.resolved_utxos.get(tx_input).ok_or_else(|| {
        WError::new(
            "utxo_to_ref_tx_in",
            &format!("Failed to find UTxO for input: {:?}", tx_input),
        )
    })?;
    let tx_in_param = RefTxIn {
        tx_hash: tx_input.transaction_id().to_hex(),
        tx_index: tx_input.index(),
        script_size: utxo
            .output
            .script_ref
            .as_ref()
            .map(|script_ref| script_ref.len() / 2),
    };
    Ok(tx_in_param)
}

pub fn utxo_to_pub_key_tx_in(
    tx_input: &csl::TransactionInput,
    context: &ParserContext,
) -> Result<PubKeyTxIn, WError> {
    let utxo = context.resolved_utxos.get(tx_input).ok_or_else(|| {
        WError::new(
            "utxo_to_pub_key_tx_in",
            &format!("Failed to find UTxO for input: {:?}", tx_input),
        )
    })?;
    let tx_in_param = PubKeyTxIn {
        tx_in: TxInParameter {
            tx_hash: tx_input.transaction_id().to_hex(),
            tx_index: tx_input.index(),
            amount: Some(utxo.output.amount.clone()),
            address: Some(utxo.output.address.clone()),
        },
    };
    Ok(tx_in_param)
}

pub fn utxo_to_tx_in(
    tx_input: &csl::TransactionInput,
    context: &ParserContext,
    index: usize,
) -> Result<TxIn, WError> {
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

    let address = csl::Address::from_bech32(&utxo.output.address)
        .map_err(|e| WError::new("utxo_to_tx_in", &format!("Failed to parse address: {}", e)))?;
    let payment_cred = address
        .payment_cred()
        .ok_or_else(|| WError::new("utxo_to_tx_in", "Failed to get payment credential"))?;

    if payment_cred.to_keyhash().is_some() {
        return Ok(TxIn::PubKeyTxIn(PubKeyTxIn { tx_in: tx_in_param }));
    };

    if let Some(script_hash) = payment_cred.to_scripthash() {
        if let Some(script) = context.script_witness.scripts.get(&script_hash) {
            match script {
                Script::ProvidedNative(native_script) => {
                    return Ok(TxIn::SimpleScriptTxIn(SimpleScriptTxIn {
                        tx_in: tx_in_param,
                        simple_script_tx_in: SimpleScriptTxInParameter::ProvidedSimpleScriptSource(
                            native_script.clone(),
                        ),
                    }));
                }
                Script::ProvidedPlutus(plutus_script) => {
                    let datum_source = get_datum_for_output(utxo, context);

                    let script_source =
                        Some(ScriptSource::ProvidedScriptSource(plutus_script.clone()));

                    let redeemer = context
                        .script_witness
                        .redeemers
                        .get(&RedeemerIndex::Spend(index))
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
                        simple_script_tx_in: SimpleScriptTxInParameter::InlineSimpleScriptSource(
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
                        .script_witness
                        .redeemers
                        .get(&RedeemerIndex::Spend(index))
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
    } else {
        return Ok(TxIn::PubKeyTxIn(PubKeyTxIn { tx_in: tx_in_param }));
    }
}

fn get_datum_for_output(utxo: &UTxO, context: &ParserContext) -> Option<DatumSource> {
    if let Some(_) = &utxo.output.plutus_data {
        Some(DatumSource::InlineDatumSource(InlineDatumSource {
            tx_hash: utxo.input.tx_hash.clone(),
            tx_index: utxo.input.output_index,
        }))
    } else if let Some(datum_hash) = &utxo.output.data_hash {
        context.script_witness.datums.get(datum_hash).cloned()
    } else {
        None
    }
}
