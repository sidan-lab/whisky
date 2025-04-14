#[cfg(test)]
mod tests {
    // use whisky::{calculate_tx_hash, MaestroProvider};
    // use whisky_common::Submitter;

    #[tokio::test]
    async fn test_submit_tx() {
        use dotenv::dotenv;
        // use std::env::var;
        dotenv().ok();
        // let provider = MaestroProvider::new(var("MAESTRO_API_KEY").unwrap().as_str(), "preprod");
        // let signed_tx = "84a300d9010281825820e332c41e8f2895e3eab8285fa11a5f55805ff02cc0cba3dd64bc7b404f49b0e80101828258390036314aebecfbc929ee447dcb50fd690604eceae9403a298d9b1f9a5475531fbe1e68b11e9a10dbbc5df889edea92325a85b758bbbf8735d91a000f424082583900d161d64eef0eeb59f9124f520f8c8f3b717ed04198d54c8b17e604aea63c153fb3ea8a4ea4f165574ea91173756de0bf30222ca0e95a649a1a068781e9021a0002917da10081825820aa8ce9e908f525c3b700a65669430ec68ca19615e7309e25bb6fa883964cfa9f5840065d990672c04350ed463c253e37b3644d52e5dc2b8e0e9b11cb89736a9a16a67f76680f8bc59e74c298e49e320b8c1f6b6e078f2816f54d92a7db5454a1d40ff5f6";
        // let result = provider.submit_tx(signed_tx).await.unwrap();
        // let tx_hash = calculate_tx_hash(signed_tx).unwrap();

        // println!("Submitted transaction: {:?}", result);
        // assert_eq!(result, tx_hash);
        // assert_eq!(result.len(), 64);
    }
}
