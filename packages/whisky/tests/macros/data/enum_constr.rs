#[cfg(test)]
mod tests {
    use super::super::constr_wrapper::Account;
    use whisky::{
        data::{Constr, Constr2},
        Asset,
    };
    use whisky_common::data::{Bool, ByteString, Constr0, Int, PlutusDataJson, Tuple, Value};
    use whisky_macros::ConstrEnum;

    #[derive(Debug, Clone, ConstrEnum)]
    pub enum UserAccount {
        UserSpotAccount(Account),
        UserFundingAccount(Account),
        UserMobileAccount(Account),
    }

    #[derive(Debug, Clone, ConstrEnum)]
    pub enum HydraUserIntentRedeemer {
        MintPlaceOrderIntent(
            Constr0<
                Box<(
                    ByteString,
                    Tuple<(ByteString, ByteString)>,
                    Tuple<(ByteString, ByteString)>,
                    Bool,
                    Int,
                    Int,
                    Int,
                    Int,
                    UserAccount,
                )>,
            >,
        ),
        HydraUserPlaceOrder,
        MintCancelOrderIntent(UserAccount, ByteString),
        HydraUserCancelOrder,
        MintWithdrawalIntent(UserAccount, Value),
        HydraUserWithdrawal,
        MintTransferIntent(UserAccount, UserAccount, Value),
        HydraUserTransfer,
        BurnIntent,
    }

    #[test]
    fn test_constr_enum_unit_variant() {
        let variant = HydraUserIntentRedeemer::HydraUserPlaceOrder;
        let json = variant.to_json_string();
        let expected = Constr::new(1, ()).to_json_string();
        assert_eq!(json, expected);

        let variant = HydraUserIntentRedeemer::HydraUserCancelOrder;
        let json = variant.to_json_string();
        let expected = Constr::new(3, ()).to_json_string();
        assert_eq!(json, expected);

        let variant = HydraUserIntentRedeemer::HydraUserWithdrawal;
        let json = variant.to_json_string();
        let expected = Constr::new(5, ()).to_json_string();
        assert_eq!(json, expected);

        let variant = HydraUserIntentRedeemer::HydraUserTransfer;
        let json = variant.to_json_string();
        let expected = Constr::new(7, ()).to_json_string();
        assert_eq!(json, expected);

        let variant = HydraUserIntentRedeemer::BurnIntent;
        let json = variant.to_json_string();
        let expected = Constr::new(8, ()).to_json_string();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_constr_enum_unamed_variant_1() {
        let constr_fields = Box::new((
            ByteString::new("order_id"),
            Tuple::new((ByteString::new("asset_a"), ByteString::new("asset_b"))),
            Tuple::new((ByteString::new("price"), ByteString::new("quantity"))),
            Bool::new(true),
            Int::new(100),
            Int::new(200),
            Int::new(300),
            Int::new(400),
            UserAccount::UserSpotAccount(Account::from(
                "test_account_123",
                ("master_key_hash", false),
                ("operation_key_hash", true),
            )),
        ));
        let variant =
            HydraUserIntentRedeemer::MintPlaceOrderIntent(Constr0::new(constr_fields.clone()));
        let json = variant.to_json_string();
        let expected = Constr0::new(Constr::new(0, constr_fields)).to_json_string();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_constr_enum_unamed_variant_2() {
        let variant = HydraUserIntentRedeemer::MintCancelOrderIntent(
            UserAccount::UserSpotAccount(Account::from(
                "test_account_123",
                ("master_key_hash", false),
                ("operation_key_hash", true),
            )),
            ByteString::new("order_id"),
        );
        let json = variant.to_json_string();
        let expected = Constr2::new(Box::new((
            UserAccount::UserSpotAccount(Account::from(
                "test_account_123",
                ("master_key_hash", false),
                ("operation_key_hash", true),
            )),
            ByteString::new("order_id"),
        )))
        .to_json_string();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_constr_enum_unamed_variant_3() {
        let variant = HydraUserIntentRedeemer::MintWithdrawalIntent(
            UserAccount::UserSpotAccount(Account::from(
                "test_account_123",
                ("master_key_hash", false),
                ("operation_key_hash", true),
            )),
            Value::from_asset(&Asset::new_from_str("lovelace", "1000")),
        );
        let json = variant.to_json_string();
        let expected = Constr::new(
            4,
            Box::new((
                UserAccount::UserSpotAccount(Account::from(
                    "test_account_123",
                    ("master_key_hash", false),
                    ("operation_key_hash", true),
                )),
                Value::from_asset(&Asset::new_from_str("lovelace", "1000")),
            )),
        )
        .to_json_string();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_constr_enum_unamed_variant_4() {
        let variant = HydraUserIntentRedeemer::MintTransferIntent(
            UserAccount::UserSpotAccount(Account::from(
                "test_account_123",
                ("master_key_hash", false),
                ("operation_key_hash", true),
            )),
            UserAccount::UserSpotAccount(Account::from(
                "to_test_account_123",
                ("to_master_key_hash", false),
                ("to_operation_key_hash", true),
            )),
            Value::from_asset(&Asset::new_from_str("lovelace", "1000")),
        );
        let json = variant.to_json_string();
        let expected = Constr::new(
            6,
            Box::new((
                UserAccount::UserSpotAccount(Account::from(
                    "test_account_123",
                    ("master_key_hash", false),
                    ("operation_key_hash", true),
                )),
                UserAccount::UserSpotAccount(Account::from(
                    "to_test_account_123",
                    ("to_master_key_hash", false),
                    ("to_operation_key_hash", true),
                )),
                Value::from_asset(&Asset::new_from_str("lovelace", "1000")),
            )),
        )
        .to_json_string();
        assert_eq!(json, expected);
    }
}
