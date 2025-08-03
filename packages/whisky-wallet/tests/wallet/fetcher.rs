#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env::var;
    use whisky_provider::BlockfrostProvider;
    use whisky_wallet::Wallet;

    #[tokio::test]
    async fn test_get_address_with_params() {
        dotenv().ok();
        let provider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );
        let mnemonic_phrase = "summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer";
        let mut wallet = Wallet::new_mnemonic(mnemonic_phrase);
        wallet = wallet.with_fetcher(provider);
        let utxos = wallet.get_unspent_outputs(None, None).await.unwrap();

        println!("utxos: {:?}", utxos);
        assert_eq!(utxos.is_empty(), false);
    }
}
