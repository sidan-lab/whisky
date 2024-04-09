use cardano_serialization_lib::address::Address;
use cardano_serialization_lib::MultiAsset;
use pallas::codec::utils::Bytes;
use pallas::ledger::traverse::{Era, MultiEraTx};
use pallas::ledger::primitives::babbage::{TransactionOutput, PostAlonzoTransactionOutput, Multiasset, Coin};
use crate::model::{Asset, UTxO};
use uplc::{tx::{eval_phase_two, ResolvedInput}, Hash, TransactionInput};

pub fn tx_eval(tx_hex: &str, inputs: Vec<UTxO>) -> Result<&str, &str> {
    let tx_bytes = hex::decode(tx_hex).expect("Invalid tx hex");
    let mtx = MultiEraTx::decode_for_era(Era::Babbage, &tx_bytes);
    let tx = match mtx {
        Ok(MultiEraTx::Babbage(tx)) => tx.into_owned(),
        _ => return Err("Invalid Tx Era"),
    };

    eval_phase_two(
        &tx,
        utxos,
        cost_mdls,
        initial_budget,
        slot_config,
        true,
        with_redeemer,
    );
    Ok("")
}

fn to_pallas_utxo(utxos: Vec<UTxO>) -> Result<Vec<ResolvedInput>, &str> {
    let mut resolved_inputs = Vec::new();
    for utxo in utxos {
        let mut resolved_input: ResolvedInput;
        let resolved_input = ResolvedInput {
            input: TransactionInput {
                transaction_id: Hash::from(hex::decode(utxo.input.tx_hash).unwrap().try_into().unwrap()),
                index: utxo.input.output_index.try_into().unwrap(),
            },
            output: TransactionOutput::PostAlonzo(PostAlonzoTransactionOutput {
                address: Bytes::from(Address::from_bech32(&utxo.output.address).unwrap().to_bytes()),

            })
        }
    }
    Ok(resolved_inputs)
}

fn to_pallas_value(assets: Vec<Asset>) -> Result<Option<MultiAsset>, &str> {
    Ok(None)
}
