#[cfg(test)]
mod tests {
    use whisky_common::models::*;

    #[test]
    fn test_asset() {
        let asset = Asset::new("lovelace".to_string(), "1000000".to_string());
        assert_eq!(asset.unit(), "lovelace".to_string());
        assert_eq!(asset.policy(), "lovelace".to_string());
        assert_eq!(asset.name(), "".to_string());
        assert_eq!(asset.quantity(), "1000000".to_string());
        assert_eq!(asset.quantity_i128(), 1000000);
    }

    #[test]
    fn test_asset2() {
        let asset = Asset::new(
            "fc0e0323b254c0eb7275349d1e32eb6cc7ecfd03f3b71408eb46d75168696e736f6e2e616461"
                .to_string(),
            "89346934".to_string(),
        );
        assert_eq!(
            asset.unit(),
            "fc0e0323b254c0eb7275349d1e32eb6cc7ecfd03f3b71408eb46d75168696e736f6e2e616461"
                .to_string()
        );
        assert_eq!(
            asset.policy(),
            "fc0e0323b254c0eb7275349d1e32eb6cc7ecfd03f3b71408eb46d751".to_string()
        );
        assert_eq!(asset.name(), "68696e736f6e2e616461".to_string());
        assert_eq!(asset.quantity(), "89346934".to_string());
        assert_eq!(asset.quantity_i128(), 89346934);
    }
}
