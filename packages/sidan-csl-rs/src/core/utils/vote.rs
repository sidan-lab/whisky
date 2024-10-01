use cardano_serialization_lib::{self as csl, JsError};

use crate::model::{VoteKind, Voter};

pub fn to_csl_voter(voter: Voter) -> Result<csl::Voter, JsError> {
    match voter {
        Voter::ConstitutionalCommitteeHotAddress(reward_address) => {
            Ok(csl::Voter::new_constitutional_committee_hot_credential(
                &csl::RewardAddress::from_address(&csl::Address::from_bech32(&reward_address)?)
                    .unwrap()
                    .payment_cred(),
            ))
        }
        Voter::DRepId(drep_id) => {
            let drep = csl::DRep::from_bech32(&drep_id).unwrap();
            let drep_credential = if drep.to_script_hash().is_some() {
                csl::Credential::from_scripthash(&drep.to_script_hash().unwrap())
            } else if drep.to_key_hash().is_some() {
                csl::Credential::from_keyhash(&drep.to_key_hash().unwrap())
            } else {
                return Err(JsError::from_str(
                    "Error occured when deserializing DrepId to either script hash or key hash",
                ));
            };
            Ok(csl::Voter::new_drep_credential(&drep_credential))
        }
        Voter::StakingPoolKeyHash(key_hash) => Ok(csl::Voter::new_stake_pool_key_hash(
            &csl::Ed25519KeyHash::from_hex(&key_hash)?,
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
