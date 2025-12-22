use pallas::ledger::primitives::{conway::Vote as PallasVote, Fragment};

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
    pub fn new(vote: VoteKind) -> Self {
        let pallas_vote = match vote {
            VoteKind::Yes => PallasVote::Yes,
            VoteKind::No => PallasVote::No,
            VoteKind::Abstain => PallasVote::Abstain,
        };

        Self { inner: pallas_vote }
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at Vote"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasVote::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
