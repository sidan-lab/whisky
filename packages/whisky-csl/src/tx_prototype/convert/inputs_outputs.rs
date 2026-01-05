use cardano_serialization_lib as csl;
use whisky_common::WError;

use super::native_script::proto_to_script_ref;
use super::plutus_data::proto_to_data_option;
use super::value::proto_to_value;
use crate::tx_prototype::types::*;

/// Convert TransactionInputPrototype to CSL TransactionInput
pub fn proto_to_transaction_input(
    input: &TransactionInputPrototype,
) -> Result<csl::TransactionInput, WError> {
    let tx_hash = csl::TransactionHash::from_hex(&input.transaction_id).map_err(
        WError::from_err("proto_to_transaction_input - invalid transaction_id"),
    )?;
    Ok(csl::TransactionInput::new(&tx_hash, input.index))
}

/// Convert Vec<TransactionInputPrototype> to CSL TransactionInputs
pub fn proto_to_transaction_inputs(
    inputs: &[TransactionInputPrototype],
) -> Result<csl::TransactionInputs, WError> {
    let mut result = csl::TransactionInputs::new();
    for input in inputs {
        result.add(&proto_to_transaction_input(input)?);
    }
    Ok(result)
}

/// Convert TransactionOutputPrototype to CSL TransactionOutput
pub fn proto_to_transaction_output(
    output: &TransactionOutputPrototype,
) -> Result<csl::TransactionOutput, WError> {
    let address = csl::Address::from_bech32(&output.address)
        .or_else(|_| {
            // Try as Byron address
            csl::ByronAddress::from_base58(&output.address).map(|byron| byron.to_address())
        })
        .map_err(WError::from_err(
            "proto_to_transaction_output - invalid address",
        ))?;

    let amount = proto_to_value(&output.amount)?;

    let mut tx_output = csl::TransactionOutput::new(&address, &amount);

    // Handle datum
    if let Some(data_option) = &output.plutus_data {
        let (data_hash, plutus_data) = proto_to_data_option(data_option)?;
        if let Some(hash) = data_hash {
            tx_output.set_data_hash(&hash);
        }
        if let Some(data) = plutus_data {
            tx_output.set_plutus_data(&data);
        }
    }

    // Handle script_ref
    if let Some(script_ref_proto) = &output.script_ref {
        let script_ref = proto_to_script_ref(script_ref_proto)?;
        tx_output.set_script_ref(&script_ref);
    }

    Ok(tx_output)
}

/// Convert Vec<TransactionOutputPrototype> to CSL TransactionOutputs
pub fn proto_to_transaction_outputs(
    outputs: &[TransactionOutputPrototype],
) -> Result<csl::TransactionOutputs, WError> {
    let mut result = csl::TransactionOutputs::new();
    for output in outputs {
        result.add(&proto_to_transaction_output(output)?);
    }
    Ok(result)
}
