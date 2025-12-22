use cardano_serialization_lib as csl;
use whisky_common::WError;

use super::certificates::proto_to_certificates;
use super::governance::{
    proto_to_protocol_param_update_from_prototype, proto_to_voting_procedures,
    proto_to_voting_proposals,
};
use super::inputs_outputs::{
    proto_to_transaction_inputs, proto_to_transaction_output, proto_to_transaction_outputs,
};
use super::primitives::{proto_to_bignum, proto_to_network_id};
use super::value::proto_to_mint;
use crate::tx_prototype::types::*;

/// Convert TransactionBodyPrototype to CSL TransactionBody
pub fn proto_to_transaction_body(
    body: &TransactionBodyPrototype,
) -> Result<csl::TransactionBody, WError> {
    let inputs = proto_to_transaction_inputs(&body.inputs)?;
    let outputs = proto_to_transaction_outputs(&body.outputs)?;
    let fee = proto_to_bignum(&body.fee)?;

    let mut tx_body = csl::TransactionBody::new_tx_body(&inputs, &outputs, &fee);

    // TTL
    if let Some(ttl) = &body.ttl {
        tx_body.set_ttl(&proto_to_bignum(ttl)?);
    }

    // Certificates
    if let Some(certs) = &body.certs {
        tx_body.set_certs(&proto_to_certificates(certs)?);
    }

    // Withdrawals
    if let Some(withdrawals) = &body.withdrawals {
        let mut csl_withdrawals = csl::Withdrawals::new();
        for (addr_str, amount_str) in withdrawals {
            let reward_address =
                csl::RewardAddress::from_address(&csl::Address::from_bech32(addr_str).map_err(
                    WError::from_err("proto_to_transaction_body - invalid withdrawal address"),
                )?)
                .ok_or_else(|| {
                    WError::new(
                        "proto_to_transaction_body",
                        "invalid reward address for withdrawal",
                    )
                })?;
            let amount = proto_to_bignum(amount_str)?;
            csl_withdrawals.insert(&reward_address, &amount);
        }
        tx_body.set_withdrawals(&csl_withdrawals);
    }

    // Update (legacy)
    if let Some(update) = &body.update {
        let csl_update = proto_to_update(update)?;
        tx_body.set_update(&csl_update);
    }

    // Auxiliary data hash
    if let Some(aux_data_hash) = &body.auxiliary_data_hash {
        let hash = csl::AuxiliaryDataHash::from_hex(aux_data_hash).map_err(WError::from_err(
            "proto_to_transaction_body - invalid auxiliary_data_hash",
        ))?;
        tx_body.set_auxiliary_data_hash(&hash);
    }

    // Validity start interval
    if let Some(validity_start) = &body.validity_start_interval {
        tx_body.set_validity_start_interval_bignum(&proto_to_bignum(validity_start)?);
    }

    // Mint
    if let Some(mint) = &body.mint {
        tx_body.set_mint(&proto_to_mint(mint)?);
    }

    // Script data hash
    if let Some(script_data_hash) = &body.script_data_hash {
        let hash = csl::ScriptDataHash::from_hex(script_data_hash).map_err(WError::from_err(
            "proto_to_transaction_body - invalid script_data_hash",
        ))?;
        tx_body.set_script_data_hash(&hash);
    }

    // Collateral
    if let Some(collateral) = &body.collateral {
        let csl_collateral = proto_to_transaction_inputs(collateral)?;
        tx_body.set_collateral(&csl_collateral);
    }

    // Required signers
    if let Some(required_signers) = &body.required_signers {
        let mut signers = csl::Ed25519KeyHashes::new();
        for signer in required_signers {
            let hash = csl::Ed25519KeyHash::from_hex(signer).map_err(WError::from_err(
                "proto_to_transaction_body - invalid required_signer",
            ))?;
            signers.add(&hash);
        }
        tx_body.set_required_signers(&signers);
    }

    // Network ID
    if let Some(network_id) = &body.network_id {
        tx_body.set_network_id(&proto_to_network_id(network_id));
    }

    // Collateral return
    if let Some(collateral_return) = &body.collateral_return {
        let output = proto_to_transaction_output(collateral_return)?;
        tx_body.set_collateral_return(&output);
    }

    // Total collateral
    if let Some(total_collateral) = &body.total_collateral {
        tx_body.set_total_collateral(&proto_to_bignum(total_collateral)?);
    }

    // Reference inputs
    if let Some(reference_inputs) = &body.reference_inputs {
        let csl_ref_inputs = proto_to_transaction_inputs(reference_inputs)?;
        tx_body.set_reference_inputs(&csl_ref_inputs);
    }

    // Voting procedures
    if let Some(voting_procedures) = &body.voting_procedures {
        tx_body.set_voting_procedures(&proto_to_voting_procedures(voting_procedures)?);
    }

    // Voting proposals
    if let Some(voting_proposals) = &body.voting_proposals {
        tx_body.set_voting_proposals(&proto_to_voting_proposals(voting_proposals)?);
    }

    // Current treasury value
    if let Some(current_treasury_value) = &body.current_treasury_value {
        tx_body.set_current_treasury_value(&proto_to_bignum(current_treasury_value)?);
    }

    // Donation
    if let Some(donation) = &body.donation {
        tx_body.set_donation(&proto_to_bignum(donation)?);
    }

    Ok(tx_body)
}

/// Convert UpdatePrototype to CSL Update
fn proto_to_update(update: &UpdatePrototype) -> Result<csl::Update, WError> {
    let mut proposed_updates = csl::ProposedProtocolParameterUpdates::new();

    for (genesis_hash_hex, ppu) in &update.proposed_protocol_parameter_updates {
        let genesis_hash = csl::GenesisHash::from_hex(genesis_hash_hex)
            .map_err(WError::from_err("proto_to_update - invalid genesis_hash"))?;
        let protocol_param_update = proto_to_protocol_param_update_from_prototype(ppu)?;
        proposed_updates.insert(&genesis_hash, &protocol_param_update);
    }

    Ok(csl::Update::new(&proposed_updates, update.epoch))
}
