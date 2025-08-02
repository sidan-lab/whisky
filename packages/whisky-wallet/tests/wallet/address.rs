#[cfg(test)]
mod test {
    use whisky_wallet::Wallet;

    #[test]
    fn test_get_address_with_params() {
        let mnemonic_phrase = "summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer";
        let wallet = Wallet::new_mnemonic(mnemonic_phrase);
        let address = wallet
            .get_change_address(whisky_wallet::AddressType::Payment)
            .unwrap();

        assert_eq!(
        address,
        "addr_test1qqzgg5pcaeyea69uptl9da5g7fajm4m0yvxndx9f4lxpkehqgezy0s04rtdwlc0tlvxafpdrfxnsg7ww68ge3j7l0lnszsw2wt"
    );
    }
}
