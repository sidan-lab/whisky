use cardano_serialization_lib as csl;
use whisky_common::WError;

use super::native_script::proto_to_native_script;
use super::plutus_data::proto_to_plutus_data_from_variant;
use super::primitives::{proto_to_bignum, proto_to_ex_units, proto_to_redeemer_tag};
use crate::tx_prototype::types::*;

/// Convert TransactionWitnessSetPrototype to CSL TransactionWitnessSet
pub fn proto_to_transaction_witness_set(
    ws: &TransactionWitnessSetPrototype,
) -> Result<csl::TransactionWitnessSet, WError> {
    let mut result = csl::TransactionWitnessSet::new();

    // Vkeys
    if let Some(vkeys) = &ws.vkeys {
        let mut vkey_witnesses = csl::Vkeywitnesses::new();
        for vkey_witness in vkeys {
            vkey_witnesses.add(&proto_to_vkeywitness(vkey_witness)?);
        }
        result.set_vkeys(&vkey_witnesses);
    }

    // Native scripts
    if let Some(native_scripts) = &ws.native_scripts {
        let mut scripts = csl::NativeScripts::new();
        for script in native_scripts {
            scripts.add(&proto_to_native_script(script)?);
        }
        result.set_native_scripts(&scripts);
    }

    // Bootstrap witnesses
    if let Some(bootstraps) = &ws.bootstraps {
        let mut bootstrap_witnesses = csl::BootstrapWitnesses::new();
        for bootstrap in bootstraps {
            bootstrap_witnesses.add(&proto_to_bootstrap_witness(bootstrap)?);
        }
        result.set_bootstraps(&bootstrap_witnesses);
    }

    // Plutus scripts
    if let Some(plutus_scripts) = &ws.plutus_scripts {
        let mut scripts = csl::PlutusScripts::new();
        for script_hex in plutus_scripts {
            let script = csl::PlutusScript::from_hex(script_hex).map_err(WError::from_err(
                "proto_to_transaction_witness_set - invalid plutus script hex",
            ))?;
            scripts.add(&script);
        }
        result.set_plutus_scripts(&scripts);
    }

    // Plutus data (datum witness set)
    if let Some(plutus_data) = &ws.plutus_data {
        let mut datum_list = csl::PlutusList::new();
        for datum_hex in &plutus_data.elems {
            let datum = csl::PlutusData::from_hex(datum_hex).map_err(WError::from_err(
                "proto_to_transaction_witness_set - invalid plutus data hex",
            ))?;
            datum_list.add(&datum);
        }
        result.set_plutus_data(&datum_list);
    }

    // Redeemers
    if let Some(redeemers) = &ws.redeemers {
        let mut redeemer_list = csl::Redeemers::new();
        for redeemer in redeemers {
            redeemer_list.add(&proto_to_redeemer(redeemer)?);
        }
        result.set_redeemers(&redeemer_list);
    }

    Ok(result)
}

/// Convert VkeywitnessPrototype to CSL Vkeywitness
fn proto_to_vkeywitness(vkey_witness: &VkeywitnessPrototype) -> Result<csl::Vkeywitness, WError> {
    let vkey = csl::Vkey::new(
        &csl::PublicKey::from_hex(&vkey_witness.vkey)
            .map_err(WError::from_err("proto_to_vkeywitness - invalid vkey"))?,
    );
    let signature = csl::Ed25519Signature::from_hex(&vkey_witness.signature)
        .map_err(WError::from_err("proto_to_vkeywitness - invalid signature"))?;
    Ok(csl::Vkeywitness::new(&vkey, &signature))
}

/// Convert BootstrapWitnessPrototype to CSL BootstrapWitness
fn proto_to_bootstrap_witness(
    bootstrap: &BootstrapWitnessPrototype,
) -> Result<csl::BootstrapWitness, WError> {
    let vkey = csl::Vkey::new(&csl::PublicKey::from_hex(&bootstrap.vkey).map_err(
        WError::from_err("proto_to_bootstrap_witness - invalid vkey"),
    )?);
    let signature = csl::Ed25519Signature::from_hex(&bootstrap.signature).map_err(
        WError::from_err("proto_to_bootstrap_witness - invalid signature"),
    )?;

    Ok(csl::BootstrapWitness::new(
        &vkey,
        &signature,
        bootstrap.chain_code.clone(),
        bootstrap.attributes.clone(),
    ))
}

/// Convert RedeemerPrototype to CSL Redeemer
fn proto_to_redeemer(redeemer: &RedeemerPrototype) -> Result<csl::Redeemer, WError> {
    let tag = proto_to_redeemer_tag(&redeemer.tag);
    let index = proto_to_bignum(&redeemer.index)?;
    let data = proto_to_plutus_data_from_variant(&redeemer.data)?;
    let ex_units = proto_to_ex_units(&redeemer.ex_units)?;
    Ok(csl::Redeemer::new(&tag, &index, &data, &ex_units))
}
