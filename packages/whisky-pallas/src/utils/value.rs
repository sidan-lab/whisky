use whisky_common::{Asset, Output, WError};

use crate::converter::convert_output;

pub fn get_min_utxo_value(output: &Output, coins_per_utxo_size: &u64) -> Result<String, WError> {
    // loop over output's assets and check if it has lovelaces
    let mut has_lovelaces = false;
    for asset in &output.amount {
        if asset.policy() == "lovelace" {
            has_lovelaces = true;
            break;
        }
    }
    match has_lovelaces {
        true => {
            let wrapped_output = convert_output(output)?;
            let cbor_length = wrapped_output.encode()?.len();
            let min_utxo_value = coins_per_utxo_size * (cbor_length as u64 + 160);
            Ok(min_utxo_value.to_string())
        }
        false => {
            // if it doesn't have lovelaces, we need to add a dummy lovelace to calculate the min utxo value
            let mut dummy_value = output.amount.clone();
            dummy_value.push(Asset::new_from_str("lovelace", &u64::MAX.to_string()));
            let dummy_output = Output {
                address: output.address.clone(),
                datum: output.datum.clone(),
                reference_script: output.reference_script.clone(),
                amount: dummy_value,
            };
            let wrapped_output = convert_output(&dummy_output)?;
            let cbor_length = wrapped_output.encode()?.len();
            let min_utxo_value = coins_per_utxo_size * (cbor_length as u64 + 160);
            Ok(min_utxo_value.to_string())
        }
    }
}
