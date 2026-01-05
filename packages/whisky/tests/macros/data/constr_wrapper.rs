use whisky::data::{ByteString, Constr0, Credential, PlutusDataJson};
use whisky_macros::ImplConstr;

// Type being tested with ImplConstr that now includes ConstrWrapper functionality
#[derive(Debug, Clone, ImplConstr)]
pub struct Account(pub Constr0<Box<(ByteString, Credential, Credential)>>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constr_wrapper_impl() {
        let inner = Constr0::new(Box::new((
            ByteString::new("test_account_123"),
            Credential::new(("master_key_hash", false)),
            Credential::new(("operation_key_hash", true)),
        )));
        let account = Account(inner);
        let account_from = Account::from(
            "test_account_123",
            ("master_key_hash", false),
            ("operation_key_hash", true),
        );
        assert_eq!(account.to_json_string(), account_from.to_json_string());
    }

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
