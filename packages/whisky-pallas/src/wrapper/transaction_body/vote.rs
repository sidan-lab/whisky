use pallas::ledger::primitives::{conway::Vote as PallasVote, Fragment};
use whisky_common::WError;

pub enum VoteKind {
    Yes,
    No,
    Abstain,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Vote {
    pub inner: PallasVote,
}
impl Vote {
    pub fn new(vote: VoteKind) -> Result<Self, WError> {
        let pallas_vote = match vote {
            VoteKind::Yes => PallasVote::Yes,
            VoteKind::No => PallasVote::No,
            VoteKind::Abstain => PallasVote::Abstain,
        };

        Ok(Self { inner: pallas_vote })
    }

    pub fn encode(&self) -> Result<String, WError> {
        let encoded = self
            .inner
            .encode_fragment()
            .map_err(|e| WError::new("Vote::encode", &format!("Fragment encode error: {}", e)))?;
        Ok(hex::encode(encoded))
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, WError> {
        let inner = PallasVote::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "Vote::decode_bytes",
                &format!("Fragment decode error: {}", e),
            )
        })?;
        Ok(Self { inner })
    }
}
