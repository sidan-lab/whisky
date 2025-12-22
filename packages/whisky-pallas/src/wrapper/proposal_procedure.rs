use pallas::ledger::primitives::{conway::ProposalProcedure as PallasProposalProcedure, Fragment};

use crate::wrapper::{reward_account::RewardAccount, Anchor, GovAction};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProposalProcedure {
    pub inner: PallasProposalProcedure,
}

impl ProposalProcedure {
    pub fn new(
        deposit: u64,
        reward_account: RewardAccount,
        gov_action: GovAction,
        anchor: Anchor,
    ) -> Result<Self, String> {
        Ok(Self {
            inner: PallasProposalProcedure {
                deposit,
                reward_account: reward_account.inner,
                gov_action: gov_action.inner,
                anchor: anchor.inner,
            },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at ProposalProcedure"),
        )
    }

    pub fn decode_bytes(bytes: &[u8]) -> Result<Self, String> {
        let inner = PallasProposalProcedure::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }
}
