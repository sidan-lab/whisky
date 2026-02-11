use std::str::FromStr;

use pallas::ledger::addresses::{Network, ShelleyAddress, ShelleyDelegationPart};
use pallas_crypto::hash::Hash;

pub fn script_to_address(
    network_id: u8,
    script_hash: &str,
    stake_cred: Option<(&str, bool)>,
) -> String {
    let stake_cred = match stake_cred {
        Some((stake, is_script)) => {
            let stake_cred = if is_script {
                ShelleyDelegationPart::Script(Hash::from_str(stake).unwrap())
            } else {
                ShelleyDelegationPart::Key(Hash::from_str(stake).unwrap())
            };
            stake_cred
        }
        None => ShelleyDelegationPart::Null,
    };
    let payment_cred =
        pallas::ledger::addresses::ShelleyPaymentPart::Script(Hash::from_str(script_hash).unwrap());

    let address = ShelleyAddress::new(
        Network::try_from(network_id).unwrap(),
        payment_cred,
        stake_cred,
    );
    address.to_bech32().unwrap()
}
