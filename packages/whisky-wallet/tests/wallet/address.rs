#[cfg(test)]
mod test {
    use whisky_wallet::Wallet;

    #[test]
    fn test_get_address_with_params() {
        let mnemonic_phrase = "summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer summer";
        let mut wallet = Wallet::new_mnemonic(mnemonic_phrase);
        let address = wallet
            .get_address_with_params(
                0,
                0,
                whisky_wallet::AddressType::Payment,
                Some("e0464447c1f51adaefe1ebfb0dd485a349a70479ced1d198cbdf7fe7"),
            )
            .unwrap();

        assert_eq!(
        address,
        "addr_test1qqzgg5pcaeyea69uptl9da5g7fajm4m0yvxndx9f4lxpkehqgezy0s04rtdwlc0tlvxafpdrfxnsg7ww68ge3j7l0lnszsw2wt"
    );
    }
}
