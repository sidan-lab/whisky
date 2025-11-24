mod aliases;
pub mod blueprint;  // Public so blueprint types can be accessed via data::blueprint::
mod credentials;
mod primitives;
mod value;
use std::fmt::Debug;

pub use aliases::*;
// Note: Blueprint types are NOT wildcard exported to avoid conflicts
// Access them via whisky::data::blueprint::TypeName
pub use credentials::*;
pub use primitives::*;
pub use value::*;

pub trait PlutusDataJson: Clone + Debug {
    fn to_json(&self) -> serde_json::Value;
    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }
    fn to_constr_field(&self) -> Vec<serde_json::Value> {
        vec![self.to_json()]
    }
}

#[derive(Clone, Debug)]
pub enum PlutusData {
    Integer(Int),
    ByteString(ByteString),
    List(List<PlutusData>),
    Map(Map<PlutusData, PlutusData>),
    Bool(Bool),
    Constr(Constr<Box<PlutusData>>),
}

impl PlutusDataJson for PlutusData {
    fn to_json(&self) -> serde_json::Value {
        match self {
            PlutusData::Integer(int) => int.to_json(),
            PlutusData::ByteString(bytes) => bytes.to_json(),
            PlutusData::List(list) => list.to_json(),
            PlutusData::Map(map) => map.to_json(),
            PlutusData::Bool(bool) => bool.to_json(),
            PlutusData::Constr(constr) => constr.to_json(),
        }
    }

    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }

    fn to_constr_field(&self) -> Vec<serde_json::Value> {
        match self {
            PlutusData::Integer(int) => vec![int.to_json()],
            PlutusData::ByteString(bytes) => vec![bytes.to_json()],
            PlutusData::List(list) => vec![list.to_json()],
            PlutusData::Map(map) => vec![map.to_json()],
            PlutusData::Bool(bool) => vec![bool.to_json()],
            PlutusData::Constr(constr) => constr.fields.to_constr_field(),
        }
    }
}

// Implementation for Box<PlutusData>
impl PlutusDataJson for Box<PlutusData> {
    fn to_json(&self) -> serde_json::Value {
        self.as_ref().to_json()
    }

    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }

    fn to_constr_field(&self) -> Vec<serde_json::Value> {
        self.as_ref().to_constr_field()
    }
}

// Implementation for Box<List<T>>
impl<T: PlutusDataJson + Clone> PlutusDataJson for Box<List<T>> {
    fn to_json(&self) -> serde_json::Value {
        self.as_ref().to_json()
    }

    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }

    fn to_constr_field(&self) -> Vec<serde_json::Value> {
        vec![self.to_json()]
    }
}

// Macro to implement PlutusDataJson for Box<SingleType>
macro_rules! impl_box_plutus_data {
    ($($ty:ty),+) => {
        $(
            impl PlutusDataJson for Box<$ty> {
                fn to_json(&self) -> serde_json::Value {
                    self.as_ref().to_json()
                }

                fn to_json_string(&self) -> String {
                    self.to_json().to_string()
                }

                fn to_constr_field(&self) -> Vec<serde_json::Value> {
                    vec![self.to_json()]
                }
            }
        )+
    };
}

// Implement for common single types that are often boxed
impl_box_plutus_data!(
    ByteString,
    Int,
    Bool
);
