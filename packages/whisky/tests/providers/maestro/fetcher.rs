#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env::var;
    use whisky::{Fetcher, MaestroProvider};

    #[tokio::test]
    async fn test_fetch_account_info() {
        dotenv().ok();
        let provider = MaestroProvider::new(var("MAESTRO_API_KEY").unwrap().as_str(), "preprod");
        let address: &str = "addr_test1qzhm3fg7v9t9e4nrlw0z49cysmvzfy3xpmvxuht80aa3rvnm5tz7rfnph9ntszp2fclw5m334udzq49777gkhwkztsks4c69rg";
        let result = provider.fetch_account_info(address).await;
        match result {
            Ok(account_info) => {
                println!("account_info: {:?}", account_info);
                assert!(!account_info.active);
            }
            _ => panic!("Error fetching account info"),
        }
    }

    #[tokio::test]
    async fn test_fetch_address_utxos() {
        dotenv().ok();
        let provider = MaestroProvider::new(var("MAESTRO_API_KEY").unwrap().as_str(), "preprod");
        let address: &str = "addr_test1wrhn0024gx9ndkmg5sfu4r6f79ewf0w42qdrd2clyuuvgjgylk345";
        let result = provider.fetch_address_utxos(address, None).await;
        println!("result: {:?}", result);
        match result {
            Ok(address_utxos) => {
                println!("address_utxos: {:?}", address_utxos);
            }
            _ => panic!("Error fetching address utxos"),
        }
    }

    #[tokio::test]
    async fn test_fetch_asset_addresses() {
        dotenv().ok();
        let provider = MaestroProvider::new(var("MAESTRO_API_KEY").unwrap().as_str(), "preprod");
        let asset = format!(
            "{}{}",
            "1c24687602c866101d41aa64e39685ee7092f26af15c5329104141fd", "6d657368"
        );

        let result = provider.fetch_asset_addresses(&asset).await;
        println!("result: {:?}", result);
        match result {
            Ok(asset_addresses) => {
                println!("asset_addresses: {:?}", asset_addresses);
                assert!(asset_addresses[0] == ("addr_test1qzhm3fg7v9t9e4nrlw0z49cysmvzfy3xpmvxuht80aa3rvnm5tz7rfnph9ntszp2fclw5m334udzq49777gkhwkztsks4c69rg".to_string(),"1".to_string()));
            }
            _ => panic!("Error fetching asset addresses"),
        }
    }

    #[tokio::test]
    async fn test_fetch_asset_metadata() {
        dotenv().ok();
        let provider = MaestroProvider::new(var("MAESTRO_API_KEY").unwrap().as_str(), "preprod");
        let asset = format!(
            "{}{}",
            "1c24687602c866101d41aa64e39685ee7092f26af15c5329104141fd", "6d657368"
        );

        let result = provider.fetch_asset_metadata(&asset).await;
        println!("result: {:?}", result);
        match result {
            Ok(asset_metadata) => {
                println!("asset_metadata: {:?}", asset_metadata);
            }
            _ => panic!("Error fetching asset metadata"),
        }
    }

    #[tokio::test]
    async fn test_fetch_block_info() {
        dotenv().ok();
        let provider = MaestroProvider::new(var("MAESTRO_API_KEY").unwrap().as_str(), "preprod");
        let block: &str = "3132189";

        let result = provider.fetch_block_info(block).await;
        println!("result: {:?}", result);
        match result {
            Ok(block_info) => {
                println!("block_info: {:?}", block_info);
                assert!(
                    block_info.hash
                        == "d527a0d00d917cb997c680a2dadd2b3642f26e7572e6074db98c45b2d270b1f1"
                );
            }
            _ => panic!("Error fetching block info"),
        }
    }

    #[tokio::test]
    async fn test_fetch_collection_assets() {
        dotenv().ok();
        let provider = MaestroProvider::new(var("MAESTRO_API_KEY").unwrap().as_str(), "preprod");
        let policy_id: &str = "1c24687602c866101d41aa64e39685ee7092f26af15c5329104141fd";

        let result = provider.fetch_collection_assets(policy_id, None).await;
        println!("result: {:?}", result);
        match result {
            Ok(collection_assets) => {
                println!("collection_assets: {:?}", collection_assets);
            }
            _ => panic!("Error fetching collection assets"),
        }
    }

    #[tokio::test]
    async fn test_fetch_protocol_parameters() {
        dotenv().ok();
        let provider = MaestroProvider::new(var("MAESTRO_API_KEY").unwrap().as_str(), "preprod");

        let result = provider.fetch_protocol_parameters(None).await;
        println!("result: {:?}", result);
        match result {
            Ok(protocol_para) => {
                println!("protocol_para: {:?}", protocol_para);
            }
            _ => panic!("Error fetching protocol para"),
        }
    }

    #[tokio::test]
    async fn test_fetch_tx_info() {
        dotenv().ok();
        let provider = MaestroProvider::new(var("MAESTRO_API_KEY").unwrap().as_str(), "preprod");
        let hash: &str = "ccdf490c8b7fd1e67f81b59eb98791d910cc785c23498a82ec845540467dc3ba";

        let result = provider.fetch_tx_info(hash).await;
        println!("result: {:?}", result);
        match result {
            Ok(tx_info) => {
                println!("tx_info: {:?}", tx_info);
                assert!(
                    tx_info.block
                        == "d527a0d00d917cb997c680a2dadd2b3642f26e7572e6074db98c45b2d270b1f1"
                );
            }
            _ => panic!("Error fetching tx info"),
        }
    }

    #[tokio::test]
    async fn test_fetch_utxo() {
        dotenv().ok();
        let provider = MaestroProvider::new(var("MAESTRO_API_KEY").unwrap().as_str(), "preprod");
        let hash: &str = "bda0866e2edc3778191960d4200a982af5530fee8e5c2efc75f6b35e5e546800";

        let result = provider.fetch_utxos(hash, Some(1)).await;
        println!("result: {:?}", result);
        match result {
            Ok(utxos) => {
                println!("utxos: {:?}", utxos);
                assert!(utxos[0].input.output_index == 1);
            }
            _ => panic!("Error fetching utxos"),
        }
    }
}
