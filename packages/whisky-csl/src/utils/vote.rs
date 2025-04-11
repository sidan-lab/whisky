use cardano_serialization_lib::{self as csl};

use whisky_common::*;

pub fn to_csl_voter(voter: Voter) -> Result<csl::Voter, WError> {
    match voter {
        Voter::ConstitutionalCommitteeHotCred(cred) => match cred {
            Credential::KeyHash(key_hash) => {
                Ok(csl::Voter::new_constitutional_committee_hot_credential(
                    &csl::Credential::from_keyhash(
                        &csl::Ed25519KeyHash::from_hex(&key_hash)
                            .map_err(WError::from_err("to_csl_voter - invalid key hash"))?,
                    ),
                ))
            }
            Credential::ScriptHash(script_hash) => {
                Ok(csl::Voter::new_constitutional_committee_hot_credential(
                    &csl::Credential::from_scripthash(
                        &csl::ScriptHash::from_hex(&script_hash)
                            .map_err(WError::from_err("to_csl_voter - invalid script hash"))?,
                    ),
                ))
            }
        },
        Voter::DRepId(drep_id) => {
            let drep = csl::DRep::from_bech32(&drep_id).unwrap();
            let drep_credential = if drep.to_script_hash().is_some() {
                csl::Credential::from_scripthash(&drep.to_script_hash().unwrap())
            } else if drep.to_key_hash().is_some() {
                csl::Credential::from_keyhash(&drep.to_key_hash().unwrap())
            } else {
                return Err(WError::new(
                    "to_csl_voter - invalid DRepId",
                    "Error occured when deserializing DrepId to either script hash or key hash",
                ));
            };
            Ok(csl::Voter::new_drep_credential(&drep_credential))
        }
        Voter::StakingPoolKeyHash(key_hash) => Ok(csl::Voter::new_stake_pool_key_hash(
            &csl::Ed25519KeyHash::from_hex(&key_hash)
                .map_err(WError::from_err("to_csl_voter - invalid key hash"))?,
        )),
    }
}

pub fn to_csl_vote_kind(vote_kind: VoteKind) -> csl::VoteKind {
    match vote_kind {
        VoteKind::No => csl::VoteKind::No,
        VoteKind::Yes => csl::VoteKind::Yes,
        VoteKind::Abstain => csl::VoteKind::Abstain,
    }
}
