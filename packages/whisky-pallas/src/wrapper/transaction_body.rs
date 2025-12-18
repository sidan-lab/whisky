use std::collections::BTreeMap;
use std::str::FromStr;

use hex::FromHex;
use pallas::codec::utils::{NonEmptySet, NonZeroInt, Set};
use pallas::ledger::primitives::conway::{
    Certificate as PallasCertificate, Multiasset as PallasMultiasset,
    TransactionBody as PallasTransactionBody, TransactionInput as PallasTransactionInput,
    TransactionOutput as PallasTransactionOutput,
};
use pallas::ledger::primitives::{Coin, Fragment, RewardAccount};

use crate::wrapper::transaction_input::TransactionInput;
use crate::wrapper::transaction_output::TransactionOutput;
use crate::wrapper::{Certificate, MultiassetNonZeroInt, RequiredSigners};
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
                network_id: None,          // Placeholder implementation
                collateral_return: None,   // Placeholder implementation
                total_collateral: None,    // Placeholder implementation
                reference_inputs: None,    // Placeholder implementation
                voting_procedures: None,   // Placeholder implementation
                proposal_procedures: None, // Placeholder implementation
                treasury_value: None,      // Placeholder implementation
                donation: None,            // Placeholder implementation
            },
        }) // Placeholder implementation
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
}
