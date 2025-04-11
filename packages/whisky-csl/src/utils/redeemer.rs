use cardano_serialization_lib::{self as csl};
use whisky_common::*;

use super::to_bignum;

pub fn to_csl_redeemer(
    redeemer_tag: RedeemerTag,
    redeemer: Redeemer,
    index: u64,
) -> Result<csl::Redeemer, WError> {
    let csl_redeemer_tag = match redeemer_tag {
        RedeemerTag::Spend => csl::RedeemerTag::new_spend(),
        RedeemerTag::Mint => csl::RedeemerTag::new_mint(),
        RedeemerTag::Cert => csl::RedeemerTag::new_cert(),
        RedeemerTag::Reward => csl::RedeemerTag::new_reward(),
        RedeemerTag::Vote => csl::RedeemerTag::new_vote(),
        RedeemerTag::Propose => csl::RedeemerTag::new_voting_proposal(),
    };
    Ok(csl::Redeemer::new(
        &csl_redeemer_tag,
        &to_bignum(index).map_err(WError::add_err_trace(
            "to_csl_redeemer - invalid redeemer index",
        ))?,
        &csl::PlutusData::from_hex(&redeemer.data)
            .map_err(WError::from_err("to_csl_redeemer - invalid redeemer data"))?,
        &csl::ExUnits::new(
            &to_bignum(redeemer.ex_units.mem).map_err(WError::add_err_trace(
                "to_csl_redeemer - invalid redeemer memory",
            ))?,
            &to_bignum(redeemer.ex_units.steps).map_err(WError::add_err_trace(
                "to_csl_redeemer - invalid redeemer steps",
            ))?,
        ),
    ))
}
