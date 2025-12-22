use pallas::ledger::primitives::{conway::VotingProcedure as PallasVotingProcedure, Fragment};

use crate::wrapper::transaction_body::{Anchor, Vote};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VotingProdecedure {
    pub inner: PallasVotingProcedure,
}

impl VotingProdecedure {
    pub fn new(vote: Vote, anchor: Option<Anchor>) -> Self {
        let pallas_vote = vote.inner;
        let pallas_anchor = anchor.map(|a| a.inner);
        let pallas_vote_procedure = PallasVotingProcedure {
            vote: pallas_vote,
            anchor: pallas_anchor,
        };
        Self {
            inner: pallas_vote_procedure,
        }
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at VotingProcedure"),
        )
    }
    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasVotingProcedure::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
