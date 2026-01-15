use pallas::ledger::primitives::conway::Tx;
use whisky_common::{PubKeyTxIn, TxIn, WError};

use crate::tx_parser::{context::ParserContext, inputs::utxo_to_tx_in};

pub fn extract_collaterals(
    pallas_tx: &Tx,
    parser_context: &ParserContext,
) -> Result<Vec<PubKeyTxIn>, WError> {
    let mut collaterals_vec: Vec<PubKeyTxIn> = Vec::new();
    let collateral_inputs = &pallas_tx.transaction_body.collateral;
    match collateral_inputs {
        Some(collateral) => {
            for input in collateral.iter() {
                let tx_in = utxo_to_tx_in(input, parser_context, 0)?;
                match tx_in {
                    TxIn::PubKeyTxIn(pub_key_tx_in) => collaterals_vec.push(pub_key_tx_in),
                    _ => {
                        return Err(WError::new(
                            "Whisky Pallas Parser - ",
                            "Only PubKeyTxIn are supported as collateral inputs",
                        ))
                    }
                }
            }
        }
        None => {}
    }
    Ok(collaterals_vec)
}
