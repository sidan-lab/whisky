use std::collections::BTreeMap;
use std::str::FromStr;

use pallas::codec::utils::{NonEmptySet, NonZeroInt, Set};
use pallas::ledger::primitives::conway::{
    Certificate as PallasCertificate, GovActionId as PallasGovActionId,
    Multiasset as PallasMultiasset, NetworkId as PallasNetworkId, PositiveCoin,
    ProposalProcedure as PallasProposalProcedure, TransactionBody as PallasTransactionBody,
    TransactionInput as PallasTransactionInput, TransactionOutput as PallasTransactionOutput,
    Voter as PallasVoter, VotingProcedure as PallasVotingProcedure,
    VotingProcedures as PallasVotingProcedures,
};
use pallas::ledger::primitives::{Coin, Fragment, RewardAccount};

use crate::wrapper::transaction_body::{
    Certificate, GovActionId, MultiassetNonZeroInt, NetworkId, ProposalProcedure, RequiredSigners,
    TransactionInput, TransactionOutput, Voter, VotingProdecedure,
};
use pallas::crypto::hash::Hash;

#[derive(Debug, PartialEq, Clone)]
pub struct TransactionBody<'a> {
    pub inner: PallasTransactionBody<'a>,
}

impl<'a> TransactionBody<'a> {
    pub fn new(
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput<'a>>,
        fee: u64,
        ttl: Option<u64>,
        certificates: Option<Vec<Certificate>>,
        withdrawals: Option<Vec<(RewardAccount, u64)>>,
        auxiliary_data_hash: Option<String>,
        validity_interval_start: Option<u64>,
        mint: Option<MultiassetNonZeroInt>,
        script_data_hash: Option<String>,
        collateral: Option<Vec<TransactionInput>>,
        required_signers: Option<RequiredSigners>,
        network_id: Option<NetworkId>,
        collateral_return: Option<TransactionOutput<'a>>,
        total_collateral: Option<u64>,
        reference_inputs: Option<Vec<TransactionInput>>,
        voting_procedures: Option<Vec<(Voter, Vec<(GovActionId, VotingProdecedure)>)>>,
        proposal_procedures: Option<Vec<ProposalProcedure>>,
        treasury_value: Option<u64>,
        donation: Option<u64>,
    ) -> Result<Self, String> {
        Ok(Self {
            inner: PallasTransactionBody {
                inputs: Self::parse_inputs(inputs),
                outputs: Self::parse_outputs(outputs),
                fee: fee,
                ttl: ttl,
                certificates: Self::parse_certificates(certificates),
                withdrawals: Self::parse_withdrawals(withdrawals),
                auxiliary_data_hash: Self::parse_auxiliary_data_hash(auxiliary_data_hash),
                validity_interval_start: validity_interval_start,
                mint: Self::parse_mint(mint),
                script_data_hash: Self::parse_script_data_hash(script_data_hash),
                collateral: Self::parse_collateral(collateral),
                required_signers: Self::parse_required_signers(required_signers),
                network_id: Self::parse_network_id(network_id),
                collateral_return: Self::parse_collateral_return(collateral_return),
                total_collateral: total_collateral,
                reference_inputs: Self::parse_reference_inputs(reference_inputs),
                voting_procedures: Self::parse_voting_procedures(voting_procedures),
                proposal_procedures: Self::parse_proposal_procedures(proposal_procedures),
                treasury_value: treasury_value,
                donation: donation
                    .map(|d| PositiveCoin::try_from(d).expect("Invalid donation value")),
            },
        })
    }

    pub fn encode(&self) -> String {
        hex::encode(
            self.inner
                .encode_fragment()
                .expect("encoding failed at TransactionBody"),
        )
    }

    pub fn decode_bytes(bytes: &'a [u8]) -> Result<Self, String> {
        let inner = PallasTransactionBody::decode_fragment(&bytes)
            .map_err(|e| format!("Fragment decode error: {}", e.to_string()))?;
        Ok(Self { inner })
    }

    fn parse_inputs(inputs: Vec<TransactionInput>) -> Set<PallasTransactionInput> {
        let pallas_inputs: Vec<PallasTransactionInput> =
            inputs.into_iter().map(|input| input.inner).collect();
        Set::from(pallas_inputs)
    }

    fn parse_outputs(outputs: Vec<TransactionOutput<'a>>) -> Vec<PallasTransactionOutput<'a>> {
        outputs.into_iter().map(|output| output.inner).collect()
    }

    fn parse_certificates(
        certificates: Option<Vec<Certificate>>,
    ) -> Option<NonEmptySet<PallasCertificate>> {
        match certificates {
            Some(certs) => {
                let pallas_certs: Vec<PallasCertificate> =
                    certs.into_iter().map(|cert| cert.inner).collect();
                NonEmptySet::from_vec(pallas_certs)
            }
            None => None,
        }
    }

    fn parse_withdrawals(
        withdrawals: Option<Vec<(RewardAccount, u64)>>,
    ) -> Option<BTreeMap<RewardAccount, Coin>> {
        withdrawals.map(|wds| BTreeMap::from_iter(wds.into_iter().map(|(ra, coin)| (ra, coin))))
    }

    fn parse_auxiliary_data_hash(auxiliary_data_hash: Option<String>) -> Option<Hash<32>> {
        auxiliary_data_hash
            .map(|hash_str| Hash::from_str(&hash_str).expect("Invalid auxiliary hash"))
    }

    fn parse_mint(mint: Option<MultiassetNonZeroInt>) -> Option<PallasMultiasset<NonZeroInt>> {
        mint.map(|ma| ma.inner)
    }

    fn parse_script_data_hash(script_data_hash: Option<String>) -> Option<Hash<32>> {
        script_data_hash
            .map(|hash_str| Hash::from_str(&hash_str).expect("Invalid script data hash"))
    }

    fn parse_collateral(
        collateral: Option<Vec<TransactionInput>>,
    ) -> Option<NonEmptySet<PallasTransactionInput>> {
        let collatera_vec = collateral.map(|inputs| {
            let pallas_inputs: Vec<PallasTransactionInput> =
                inputs.into_iter().map(|input| input.inner).collect();
            pallas_inputs
        });
        match collatera_vec {
            Some(vec) => NonEmptySet::from_vec(vec),
            None => None,
        }
    }

    fn parse_required_signers(
        required_signers: Option<RequiredSigners>,
    ) -> Option<NonEmptySet<Hash<28>>> {
        required_signers.map(|rs| rs.inner)
    }

    fn parse_network_id(network_id: Option<NetworkId>) -> Option<PallasNetworkId> {
        network_id.map(|nid| nid.inner)
    }

    fn parse_collateral_return(
        collateral_return: Option<TransactionOutput<'a>>,
    ) -> Option<PallasTransactionOutput<'a>> {
        collateral_return.map(|cr| cr.inner)
    }

    fn parse_reference_inputs(
        reference_inputs: Option<Vec<TransactionInput>>,
    ) -> Option<NonEmptySet<PallasTransactionInput>> {
        let ref_inputs_vec = reference_inputs.map(|inputs| {
            let pallas_inputs: Vec<PallasTransactionInput> =
                inputs.into_iter().map(|input| input.inner).collect();
            pallas_inputs
        });
        match ref_inputs_vec {
            Some(vec) => NonEmptySet::from_vec(vec),
            None => None,
        }
    }

    fn parse_voting_procedures(
        voting_procedures: Option<Vec<(Voter, Vec<(GovActionId, VotingProdecedure)>)>>,
    ) -> Option<PallasVotingProcedures> {
        let mut voting_procedures_map: BTreeMap<
            PallasVoter,
            BTreeMap<PallasGovActionId, PallasVotingProcedure>,
        > = BTreeMap::new();
        match voting_procedures {
            None => return None,
            Some(vp) => {
                for (voter, actions) in vp {
                    let pallas_voter = voter.inner;
                    let mut pallas_action_map: BTreeMap<PallasGovActionId, PallasVotingProcedure> =
                        BTreeMap::new();

                    for (gov_action_id, voting_procedure) in actions {
                        pallas_action_map.insert(gov_action_id.inner, voting_procedure.inner);
                    }
                    voting_procedures_map.insert(pallas_voter, pallas_action_map);
                }
                Some(voting_procedures_map)
            }
        }
    }

    fn parse_proposal_procedures(
        proposal_procedures: Option<Vec<ProposalProcedure>>,
    ) -> Option<NonEmptySet<PallasProposalProcedure>> {
        match proposal_procedures {
            Some(pp) => {
                let pallas_pp: Vec<PallasProposalProcedure> =
                    pp.into_iter().map(|proc| proc.inner).collect();
                NonEmptySet::from_vec(pallas_pp)
            }
            None => None,
        }
    }
}
