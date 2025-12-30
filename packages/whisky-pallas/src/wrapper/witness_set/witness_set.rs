use crate::wrapper::witness_set::{
    bootstrap_witness::BootstrapWitness, native_script::NativeScript, plutus_data::PlutusData,
    plutus_script::PlutusScript, redeemer::Redeemer, vkey_witness::VKeyWitness,
};
use pallas::{
    codec::utils::{KeepRaw, NonEmptySet},
    ledger::primitives::{
        conway::{
            BootstrapWitness as PallasBootstrapWitness, NativeScript as PallasNativeScript,
            PlutusData as PallasPlutusData, PlutusScript as PallasPlutusScript,
            Redeemers as PallasRedeemers, VKeyWitness as PallasVKeyWitness,
            WitnessSet as PallasWitnessSet,
        },
        Fragment,
    },
};
use whisky_common::WError;

pub struct WitnessSet<'a> {
    pub inner: PallasWitnessSet<'a>,
}

impl<'a> WitnessSet<'a> {
    pub fn new(
        vkeywitness: Option<Vec<VKeyWitness>>,
        native_script: Option<Vec<NativeScript>>,
        bootstrap_witness: Option<Vec<BootstrapWitness>>,
        plutus_v1_script: Option<Vec<PlutusScript<1>>>,
        plutus_data: Option<Vec<PlutusData>>,
        redeemer: Option<Vec<Redeemer>>,
        plutus_v2_script: Option<Vec<PlutusScript<2>>>,
        plutus_v3_script: Option<Vec<PlutusScript<3>>>,
    ) -> Result<Self, WError> {
        let pallas_vkeywitness: Option<NonEmptySet<PallasVKeyWitness>> = match vkeywitness {
            Some(wits) => Some(
                NonEmptySet::from_vec(wits.into_iter().map(|w| w.inner).collect()).ok_or_else(
                    || {
                        WError::new(
                            "WhiskyPallas - Creating witness set:",
                            "VKeyWitness NonEmptySet creation failed",
                        )
                    },
                )?,
            ),
            None => None,
        };

        let pallas_native_script: Option<NonEmptySet<KeepRaw<'a, PallasNativeScript>>> =
            match native_script {
                Some(scripts) => Some(
                    NonEmptySet::from_vec(
                        scripts
                            .into_iter()
                            .map(|s| KeepRaw::from(s.inner))
                            .collect(),
                    )
                    .ok_or_else(|| {
                        WError::new(
                            "WhiskyPallas - Creating witness set:",
                            "NativeScript NonEmptySet creation failed",
                        )
                    })?,
                ),
                None => None,
            };

        let pallas_bootstrap_witness: Option<NonEmptySet<PallasBootstrapWitness>> =
            match bootstrap_witness {
                Some(wits) => Some(
                    NonEmptySet::from_vec(wits.into_iter().map(|w| w.inner).collect()).ok_or_else(
                        || {
                            WError::new(
                                "WhiskyPallas - Creating witness set:",
                                "BootstrapWitness NonEmptySet creation failed",
                            )
                        },
                    )?,
                ),
                None => None,
            };

        let pallas_plutus_v1_script: Option<NonEmptySet<PallasPlutusScript<1>>> =
            match plutus_v1_script {
                Some(scripts) => Some(
                    NonEmptySet::from_vec(scripts.into_iter().map(|s| s.inner).collect())
                        .ok_or_else(|| {
                            WError::new(
                                "WhiskyPallas - Creating witness set:",
                                "PlutusV1Script NonEmptySet creation failed",
                            )
                        })?,
                ),
                None => None,
            };

        let pallas_plutus_data: Option<KeepRaw<'a, NonEmptySet<KeepRaw<'a, PallasPlutusData>>>> =
            match plutus_data {
                Some(data_vec) => Some(KeepRaw::from(
                    NonEmptySet::from_vec(
                        data_vec
                            .into_iter()
                            .map(|d| KeepRaw::from(d.inner))
                            .collect(),
                    )
                    .ok_or_else(|| {
                        WError::new(
                            "WhiskyPallas - Creating witness set:",
                            "PlutusData NonEmptySet creation failed",
                        )
                    })?,
                )),
                None => None,
            };

        let pallas_redeemer: Option<KeepRaw<'a, PallasRedeemers>> = match redeemer {
            Some(redeemers_vec) => Some(KeepRaw::from(PallasRedeemers::List(
                redeemers_vec.into_iter().map(|r| r.inner).collect(),
            ))),
            None => None,
        };

        let pallas_plutus_v2_script: Option<NonEmptySet<PallasPlutusScript<2>>> =
            match plutus_v2_script {
                Some(scripts) => Some(
                    NonEmptySet::from_vec(scripts.into_iter().map(|s| s.inner).collect())
                        .ok_or_else(|| {
                            WError::new(
                                "WhiskyPallas - Creating witness set:",
                                "PlutusV2Script NonEmptySet creation failed",
                            )
                        })?,
                ),
                None => None,
            };

        let pallas_plutus_v3_script: Option<NonEmptySet<PallasPlutusScript<3>>> =
            match plutus_v3_script {
                Some(scripts) => Some(
                    NonEmptySet::from_vec(scripts.into_iter().map(|s| s.inner).collect())
                        .ok_or_else(|| {
                            WError::new(
                                "WhiskyPallas - Creating witness set:",
                                "PlutusV3Script NonEmptySet creation failed",
                            )
                        })?,
                ),
                None => None,
            };

        let inner: PallasWitnessSet<'a> = PallasWitnessSet {
            vkeywitness: pallas_vkeywitness,
            native_script: pallas_native_script,
            bootstrap_witness: pallas_bootstrap_witness,
            plutus_v1_script: pallas_plutus_v1_script,
            plutus_data: pallas_plutus_data,
            redeemer: pallas_redeemer,
            plutus_v2_script: pallas_plutus_v2_script,
            plutus_v3_script: pallas_plutus_v3_script,
        };

        Ok(Self { inner })
    }

    pub fn encode(&self) -> Result<String, WError> {
        self.inner
            .encode_fragment()
            .map(|bytes| hex::encode(bytes))
            .map_err(|_| {
                WError::new(
                    "WhiskyPallas - Encoding witness set:",
                    "Failed to encode fragment",
                )
            })
    }

    pub fn decode_bytes(bytes: &'a [u8]) -> Result<Self, WError> {
        let inner = PallasWitnessSet::decode_fragment(&bytes).map_err(|e| {
            WError::new(
                "WhiskyPallas - Decoding witness set:",
                &format!("Fragment decode error: {}", e.to_string()),
            )
        })?;
        Ok(Self { inner })
    }
}
