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

use crate::WError;

pub trait PlutusDataJson: Clone + Debug + Sized {
    fn to_json(&self) -> serde_json::Value;
    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }
    fn to_constr_field(&self) -> Vec<serde_json::Value> {
        vec![self.to_json()]
    }

    fn from_json(value: &serde_json::Value) -> Result<Self, WError>;
    fn from_json_string(json_str: &str) -> Result<Self, WError> {
        let value: serde_json::Value = serde_json::from_str(json_str)
            .map_err(WError::from_err("PlutusDataJson::from_json_string"))?;
        Self::from_json(&value)
    }

    /// Parse from constructor fields array (the inverse of to_constr_field).
    /// Default implementation handles single-element arrays by extracting the first element.
    fn from_constr_field(fields: &serde_json::Value) -> Result<Self, WError> {
        let arr = fields
            .as_array()
            .ok_or_else(|| WError::new("from_constr_field", "expected array"))?;
        if arr.len() == 1 {
            Self::from_json(&arr[0])
        } else if arr.is_empty() {
            Self::from_json(fields)
        } else {
            // For tuples and multi-field types, pass the array directly
            Self::from_json(fields)
        }
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

    fn from_json(value: &serde_json::Value) -> Result<Self, WError> {
        // Detect type based on JSON structure
        if value.get("int").is_some() {
            Int::from_json(value).map(PlutusData::Integer)
        } else if value.get("bytes").is_some() {
            ByteString::from_json(value).map(PlutusData::ByteString)
        } else if value.get("list").is_some() {
            List::<PlutusData>::from_json(value).map(PlutusData::List)
        } else if value.get("map").is_some() {
            Map::<PlutusData, PlutusData>::from_json(value).map(PlutusData::Map)
        } else if value.get("constructor").is_some() {
            // Check if it's a Bool (constructor 0 or 1 with empty fields)
            let constructor = value.get("constructor").and_then(|c| c.as_u64());
            let fields = value.get("fields").and_then(|f| f.as_array());
            if let (Some(tag), Some(f)) = (constructor, fields) {
                if (tag == 0 || tag == 1) && f.is_empty() {
                    return Bool::from_json(value).map(PlutusData::Bool);
                }
            }
            // Otherwise it's a Constr
            Constr::<Box<PlutusData>>::from_json(value).map(PlutusData::Constr)
        } else {
            Err(WError::new("PlutusData::from_json", "unrecognized JSON format"))
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

    fn from_json(value: &serde_json::Value) -> Result<Self, WError> {
        PlutusData::from_json(value).map(Box::new)
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

    fn from_json(value: &serde_json::Value) -> Result<Self, WError> {
        List::<T>::from_json(value).map(Box::new)
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

                fn from_json(value: &serde_json::Value) -> Result<Self, WError> {
                    <$ty>::from_json(value).map(Box::new)
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
