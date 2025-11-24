use whisky_common::data::{ByteString, Constr0, Credential, Int, PlutusDataJson};
use whisky_macros::ImplConstr;

// Newtype wrappers with ImplConstr that provides both from() and PlutusDataJson
#[derive(Clone, Debug, ImplConstr)]
pub struct OutputReference(pub Constr0<Box<(ByteString, Int)>>);

// #[derive(Clone, Debug, ImplConstr)]
// pub struct UserTradeAccount(pub Constr0<Box<(ByteString, ByteString)>>);

#[derive(Clone, Debug, ImplConstr)]
pub struct Account(pub Constr0<Box<(ByteString, Credential, Credential)>>);
