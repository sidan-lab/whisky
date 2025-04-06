#[cfg(test)]
mod tests {
    use serde_json::json;
    use whisky::*;

    fn test_fn(tx: &mut WhiskyTx) -> Result<&mut WhiskyTx, WError> {
        let res = tx
            .withdraw_from_script(
                &LanguageVersion::V2,
                "stake_address",
                123,
                &WRedeemer {
                    ex_units: Budget::default(),
                    data: WData::JSON(con_str0(json!([])).to_string()),
                },
            )?
            .provide_script("123")?;
        Ok(res)
    }

    #[test]
    fn test_whisky_tx() {
        let mut whisky_tx = WhiskyTx::new();
        let res = test_fn(&mut whisky_tx);
        match res {
            Ok(tx) => println!("{:?}", tx.tx_builder.tx_hex()),
            Err(e) => panic!("{:?}", e),
        }
    }
}
