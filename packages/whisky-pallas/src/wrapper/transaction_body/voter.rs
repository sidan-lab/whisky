use std::str::FromStr;

use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::conway::Voter as PallasVoter;
use pallas::ledger::primitives::Fragment;
use whisky_common::WError;

pub enum VoterKind {
    ConstitutionalCommitteScript { script_hash: String },
    ConstitutionalCommitteKey { key_hash: String },
    DrepScript { script_hash: String },
    DrepKey { key_hash: String },
    StakePoolKey { pool_key_hash: String },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Voter {
    pub inner: PallasVoter,
}

impl Voter {
    pub fn new(voter: VoterKind) -> Result<Self, WError> {
        let pallas_voter = match voter {
            VoterKind::ConstitutionalCommitteScript { script_hash } => {
                PallasVoter::ConstitutionalCommitteeScript(
                    Hash::<28>::from_str(&script_hash).map_err(|e| {
                        WError::new("Voter::new", &format!("Invalid script hash length: {}", e))
                    })?,
                )
            }

            VoterKind::ConstitutionalCommitteKey { key_hash } => {
                PallasVoter::ConstitutionalCommitteeKey(Hash::<28>::from_str(&key_hash).map_err(
                    |e| WError::new("Voter::new", &format!("Invalid key hash length: {}", e)),
                )?)
            }

            VoterKind::DrepScript { script_hash } => {
                PallasVoter::DRepScript(Hash::<28>::from_str(&script_hash).map_err(|e| {
                    WError::new("Voter::new", &format!("Invalid script hash length: {}", e))
                })?)
            }

            VoterKind::DrepKey { key_hash } => {
                PallasVoter::DRepKey(Hash::<28>::from_str(&key_hash).map_err(|e| {
                    WError::new("Voter::new", &format!("Invalid key hash length: {}", e))
                })?)
            }

            VoterKind::StakePoolKey { pool_key_hash } => {
                PallasVoter::StakePoolKey(Hash::<28>::from_str(&pool_key_hash).map_err(|e| {
                    WError::new(
                        "Voter::new",
                        &format!("Invalid pool key hash length: {}", e),
                    )
                })?)
            }
        };

        Ok(Self {
            inner: pallas_voter,
        })
    }

    pub fn encode(&self) -> Result<String, WError> {
        let encoded = self
            .inner
            .encode_fragment()
            .map_err(|e| WError::new("Voter::encode", &format!("Fragment encode error: {}", e)))?;
        Ok(hex::encode(encoded))
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasVoter::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "Voter::decode_bytes",
                &format!("Fragment decode error: {}", e),
            )
        })?;
        Ok(Self { inner })
    }
}
