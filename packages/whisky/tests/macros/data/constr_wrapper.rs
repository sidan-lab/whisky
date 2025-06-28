use serde_json::Value;
use whisky::impl_constr_wrapper_type;
use whisky_common::data::{ByteString, Constr0, Credential, PlutusDataToJson};
use whisky_macros::ConstrWrapper;

// Type being tested with both the derive macro and implementation macro
#[derive(Debug, Clone, ConstrWrapper)]
pub struct Account(Constr0<Box<(ByteString, Credential, Credential)>>);
impl_constr_wrapper_type!(Account, 0, [
    (account_id: ByteString, &str),
    (master_key: Credential, (&str, bool)),
    (operation_key: Credential, (&str, bool)),
]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constr_wrapper_derive() {
        // Create test data
        let account = Account::from(
            "test_account_123",
            ("master_key_hash", false),
            ("operation_key_hash", true),
        );

        let inner = account.clone().0;
        let wrapper_json = account.to_json_string();
        let inner_json = inner.to_json_string();
        assert_eq!(wrapper_json, inner_json);
    }
}
