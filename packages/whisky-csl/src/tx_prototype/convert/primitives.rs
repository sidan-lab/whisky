use cardano_serialization_lib as csl;
use whisky_common::WError;

use crate::tx_prototype::types::*;

/// Convert a string to BigNum (for Prototype types)
pub fn proto_to_bignum(val: &str) -> Result<csl::BigNum, WError> {
    csl::BigNum::from_str(val).map_err(WError::from_err("proto_to_bignum - invalid value"))
}

/// Convert a string to Int (can be negative)
pub fn proto_to_int(val: &str) -> Result<csl::Int, WError> {
    csl::Int::from_str(val).map_err(WError::from_err("proto_to_int - invalid value"))
}

/// Convert UnitIntervalPrototype to CSL UnitInterval
pub fn proto_to_unit_interval(
    interval: &UnitIntervalPrototype,
) -> Result<csl::UnitInterval, WError> {
    Ok(csl::UnitInterval::new(
        &proto_to_bignum(&interval.numerator)?,
        &proto_to_bignum(&interval.denominator)?,
    ))
}

/// Convert ProtocolVersionPrototype to CSL ProtocolVersion
pub fn proto_to_protocol_version(pv: &ProtocolVersionPrototype) -> csl::ProtocolVersion {
    csl::ProtocolVersion::new(pv.major, pv.minor)
}

/// Convert ExUnitsPrototype to CSL ExUnits
pub fn proto_to_ex_units(ex_units: &ExUnitsPrototype) -> Result<csl::ExUnits, WError> {
    Ok(csl::ExUnits::new(
        &proto_to_bignum(&ex_units.mem)?,
        &proto_to_bignum(&ex_units.steps)?,
    ))
}

/// Convert AnchorPrototype to CSL Anchor
pub fn proto_to_anchor(anchor: &AnchorPrototype) -> Result<csl::Anchor, WError> {
    Ok(csl::Anchor::new(
        &csl::URL::new(anchor.anchor_url.clone())
            .map_err(WError::from_err("proto_to_anchor - invalid url"))?,
        &csl::AnchorDataHash::from_hex(&anchor.anchor_data_hash).map_err(WError::from_err(
            "proto_to_anchor - invalid anchor_data_hash",
        ))?,
    ))
}

/// Convert CredTypePrototype to CSL Credential
pub fn proto_to_credential(cred: &CredTypePrototype) -> Result<csl::Credential, WError> {
    match cred {
        CredTypePrototype::Key { value: key_hash } => Ok(csl::Credential::from_keyhash(
            &csl::Ed25519KeyHash::from_hex(key_hash)
                .map_err(WError::from_err("proto_to_credential - invalid key hash"))?,
        )),
        CredTypePrototype::Script { value: script_hash } => Ok(csl::Credential::from_scripthash(
            &csl::ScriptHash::from_hex(script_hash).map_err(WError::from_err(
                "proto_to_credential - invalid script hash",
            ))?,
        )),
    }
}

/// Convert NetworkIdPrototype to CSL NetworkId
pub fn proto_to_network_id(network_id: &NetworkIdPrototype) -> csl::NetworkId {
    match network_id {
        NetworkIdPrototype::Mainnet => csl::NetworkId::mainnet(),
        NetworkIdPrototype::Testnet => csl::NetworkId::testnet(),
    }
}

/// Convert DRepPrototype to CSL DRep
pub fn proto_to_drep(drep: &DRepPrototype) -> Result<csl::DRep, WError> {
    match drep {
        DRepPrototype::AlwaysAbstain => Ok(csl::DRep::new_always_abstain()),
        DRepPrototype::AlwaysNoConfidence => Ok(csl::DRep::new_always_no_confidence()),
        DRepPrototype::KeyHash { value } => {
            let hash = csl::Ed25519KeyHash::from_hex(value)
                .map_err(WError::from_err("proto_to_drep - invalid key hash"))?;
            Ok(csl::DRep::new_key_hash(&hash))
        }
        DRepPrototype::ScriptHash { value } => {
            let hash = csl::ScriptHash::from_hex(value)
                .map_err(WError::from_err("proto_to_drep - invalid script hash"))?;
            Ok(csl::DRep::new_script_hash(&hash))
        }
    }
}

/// Convert VoteKindPrototype to CSL VoteKind
pub fn proto_to_vote_kind(vote_kind: &VoteKindPrototype) -> csl::VoteKind {
    match vote_kind {
        VoteKindPrototype::No => csl::VoteKind::No,
        VoteKindPrototype::Yes => csl::VoteKind::Yes,
        VoteKindPrototype::Abstain => csl::VoteKind::Abstain,
    }
}

/// Convert RedeemerTagPrototype to CSL RedeemerTag
pub fn proto_to_redeemer_tag(tag: &RedeemerTagPrototype) -> csl::RedeemerTag {
    match tag {
        RedeemerTagPrototype::Spend => csl::RedeemerTag::new_spend(),
        RedeemerTagPrototype::Mint => csl::RedeemerTag::new_mint(),
        RedeemerTagPrototype::Cert => csl::RedeemerTag::new_cert(),
        RedeemerTagPrototype::Reward => csl::RedeemerTag::new_reward(),
        RedeemerTagPrototype::Vote => csl::RedeemerTag::new_vote(),
        RedeemerTagPrototype::VotingProposal => csl::RedeemerTag::new_voting_proposal(),
    }
}

/// Convert LanguageKindPrototype to CSL Language
pub fn proto_to_language(lang: &LanguageKindPrototype) -> csl::Language {
    match lang {
        LanguageKindPrototype::PlutusV1 => csl::Language::new_plutus_v1(),
        LanguageKindPrototype::PlutusV2 => csl::Language::new_plutus_v2(),
        LanguageKindPrototype::PlutusV3 => csl::Language::new_plutus_v3(),
    }
}

/// Convert Ipv4Prototype to CSL Ipv4
pub fn proto_to_ipv4(ipv4: &Ipv4Prototype) -> Result<csl::Ipv4, WError> {
    csl::Ipv4::new(ipv4.to_vec()).map_err(WError::from_err("proto_to_ipv4 - invalid ipv4"))
}

/// Convert Ipv6Prototype to CSL Ipv6
pub fn proto_to_ipv6(ipv6: &Ipv6Prototype) -> Result<csl::Ipv6, WError> {
    csl::Ipv6::new(ipv6.to_vec()).map_err(WError::from_err("proto_to_ipv6 - invalid ipv6"))
}
