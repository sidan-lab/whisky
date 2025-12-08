//! Converters from TransactionPrototype types to CSL (cardano-serialization-lib) types

mod auxiliary_data;
mod body;
mod certificates;
mod governance;
mod inputs_outputs;
mod native_script;
mod plutus_data;
mod primitives;
mod transaction;
mod value;
mod witness_set;

// Re-export main conversion functions
pub use transaction::{
    proto_to_csl_transaction, proto_to_transaction_bytes, proto_to_transaction_hex,
};

// Re-export component converters for fine-grained usage
pub use auxiliary_data::proto_to_auxiliary_data;
pub use body::proto_to_transaction_body;
pub use certificates::{proto_to_certificate, proto_to_certificates};
pub use governance::{
    proto_to_governance_action, proto_to_governance_action_id, proto_to_voter,
    proto_to_voter_votes, proto_to_voting_procedure, proto_to_voting_procedures,
    proto_to_voting_proposal, proto_to_voting_proposals,
};
pub use inputs_outputs::{
    proto_to_transaction_input, proto_to_transaction_inputs, proto_to_transaction_output,
    proto_to_transaction_outputs,
};
pub use native_script::{proto_to_native_script, proto_to_script_ref};
pub use plutus_data::{
    proto_to_data_option, proto_to_plutus_data, proto_to_plutus_data_from_variant,
};
pub use primitives::{
    proto_to_anchor, proto_to_bignum, proto_to_credential, proto_to_drep, proto_to_ex_units,
    proto_to_int, proto_to_ipv4, proto_to_ipv6, proto_to_language, proto_to_network_id,
    proto_to_protocol_version, proto_to_redeemer_tag, proto_to_unit_interval, proto_to_vote_kind,
};
pub use value::{proto_to_assets, proto_to_mint, proto_to_multiasset, proto_to_value};
pub use witness_set::proto_to_transaction_witness_set;
