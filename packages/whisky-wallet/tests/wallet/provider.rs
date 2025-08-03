#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env::var;
    use whisky_provider::BlockfrostProvider;
    use whisky_wallet::Wallet;

    #[tokio::test]
    async fn test_get_utxos() {
        dotenv().ok();
        let provider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );
        let mnemonic_phrase = "summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer";
        let mut wallet = Wallet::new_mnemonic(mnemonic_phrase);
        wallet = wallet.with_fetcher(provider);
        let utxos = wallet.get_utxos(None, None).await.unwrap();

        println!("utxos: {:?}", utxos);
        assert_eq!(utxos.is_empty(), false);
    }

    #[tokio::test]
    async fn test_get_collateral() {
        dotenv().ok();
        let provider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );
        let mnemonic_phrase = "summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer";
        let mut wallet = Wallet::new_mnemonic(mnemonic_phrase);
        wallet = wallet.with_fetcher(provider);
        let collateral = wallet.get_collateral(None).await.unwrap();

        println!("collateral: {:?}", collateral);
        assert_eq!(collateral.is_empty(), false);
    }

    #[tokio::test]
    async fn test_submit_tx() {
        dotenv().ok();
        let provider = BlockfrostProvider::new(
            var("BLOCKFROST_PREPROD_PROJECT_ID").unwrap().as_str(),
            "preprod",
        );
        let mnemonic_phrase = "summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer";
        let mut wallet = Wallet::new_mnemonic(mnemonic_phrase);
        wallet = wallet.with_submitter(provider);
        let tx_hex = "84a300d901028182582037939fafee72fbb1f61fc10d73c51b9825d6d998307af14bcbe14d2bbfc164d4010182825839001195997a35c4f3f0b0d1edb2c3123a25897d9810e0545f950c61ae1f187a7d176af61c8d705e1d1c81ea3d229570d61f4690e638a5dafca61a004c4b4082583900b5ea75ba2eac9a884ba7c47110ab7f94f9c0306636e4df01f338920f84cb00eb67b5995c61a92b1e016f78da68c1487c41055b96bec1b3981a3a59361e021a0002917da100d9010281825820312803d57366263f9e5310262fb9984f58dbc5602ad5ebdd5725719f4f913ced5840c36612ef96989d030b8414a619abf64b2ba68838d84fa87535bf1140ad8d27b74c7fdbdddc14e74defb1e9282e2f1bfa20875c5368928cf4f98d143674ed260cf5f6";
        let result = wallet.submit_tx(tx_hex).await;
        println!("result: {:?}", result);
        assert_eq!(result.is_ok(), false);
    }
}
