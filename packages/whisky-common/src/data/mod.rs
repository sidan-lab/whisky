mod aliases;
mod credentials;
mod primitives;
mod value;
use std::fmt::Debug;

pub use aliases::*;
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

impl PlutusDataJson for Box<PlutusData> {
    fn to_json(&self) -> serde_json::Value {
        let inner = self.as_ref();
        inner.to_json()
    }

    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }

    fn to_constr_field(&self) -> Vec<serde_json::Value> {
        let inner = self.as_ref();
        match inner {
            PlutusData::Integer(int) => vec![int.to_json()],
            PlutusData::ByteString(bytes) => vec![bytes.to_json()],
            PlutusData::List(list) => vec![list.to_json()],
            PlutusData::Map(map) => vec![map.to_json()],
            PlutusData::Bool(bool) => vec![bool.to_json()],
            PlutusData::Constr(constr) => constr.fields.to_constr_field(),
        }
    }
}

#[macro_export]
macro_rules! impl_constr_type {
    // Single parameter case
    ($name:ident, 0, ($param_name:ident: $param_type:ty, $param_conv:ty)) => {
        impl $name {
            pub fn from($param_name: $param_conv) -> Self {
                Constr0::new(<$param_type>::new($param_name))
            }
        }
    };

    ($name:ident, 1,($param_name:ident: $param_type:ty, $param_conv:ty)) => {
        impl $name {
            pub fn from($param_name: $param_conv) -> Self {
                Constr1::new(<$param_type>::new($param_name))
            }
        }
    };

    ($name:ident, 2,($param_name:ident: $param_type:ty, $param_conv:ty)) => {
        impl $name {
            pub fn from($param_name: $param_conv) -> Self {
                Constr2::new(<$param_type>::new($param_name))
            }
        }
    };

    ($name:ident, $tag:expr, ($param_name:ident: $param_type:ty, $param_conv:ty)) => {
        impl $name {
            pub fn from($param_name: $param_conv) -> Self {
                Constr::new($tag, <$param_type>::new($param_name))
            }
        }
    };

    // Multiple parameters case
    ($name:ident, 0, [$(($param_name:ident: $param_type:ty, $param_conv:ty)),+ $(,)?]) => {
        impl $name {
            pub fn from($($param_name: $param_conv),+) -> Self {
                Constr0::new(Box::new((
                    $(<$param_type>::new($param_name),)+
                )))
            }
        }
    };

    ($name:ident, 1, [$(($param_name:ident: $param_type:ty, $param_conv:ty)),+ $(,)?]) => {
        impl $name {
            pub fn from($($param_name: $param_conv),+) -> Self {
                Constr1::new(Box::new((
                    $(<$param_type>::new($param_name),)+
                )))
            }
        }
    };

    ($name:ident, 2, [$(($param_name:ident: $param_type:ty, $param_conv:ty)),+ $(,)?]) => {
        impl $name {
            pub fn from($($param_name: $param_conv),+) -> Self {
                Constr1::new(Box::new((
                    $(<$param_type>::new($param_name),)+
                )))
            }
        }
    };

    ($name:ident, $tag:expr, [$(($param_name:ident: $param_type:ty, $param_conv:ty)),+ $(,)?]) => {
        impl $name {
            pub fn from($($param_name: $param_conv),+) -> Self {
                Constr::new($tag, Box::new((
                    $(<$param_type>::new($param_name),)+
                )))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_constr_wrapper_type {
    // Single parameter case
    ($name:ident, 0, ($param_name:ident: $param_type:ty, $param_conv:ty)) => {
        impl $name {
            pub fn from($param_name: $param_conv) -> Self {
                $name(Constr0::new(<$param_type>::new($param_name)))
            }
        }
    };

    ($name:ident, 1,($param_name:ident: $param_type:ty, $param_conv:ty)) => {
        impl $name {
            pub fn from($param_name: $param_conv) -> Self {
                $name(Constr1::new(<$param_type>::new($param_name)))
            }
        }
    };

    ($name:ident, 2,($param_name:ident: $param_type:ty, $param_conv:ty)) => {
        impl $name {
            pub fn from($param_name: $param_conv) -> Self {
                $name(Constr2::new(<$param_type>::new($param_name)))
            }
        }
    };

    ($name:ident, $tag:expr, ($param_name:ident: $param_type:ty, $param_conv:ty)) => {
        impl $name {
            pub fn from($param_name: $param_conv) -> Self {
                $name(Constr::new($tag, <$param_type>::new($param_name)))
            }
        }
    };

    // Multiple parameters case
    ($name:ident, 0, [$(($param_name:ident: $param_type:ty, $param_conv:ty)),+ $(,)?]) => {
        impl $name {
            pub fn from($($param_name: $param_conv),+) -> Self {
                $name(Constr0::new(Box::new((
                    $(<$param_type>::new($param_name),)+
                ))))
            }
        }
    };

    ($name:ident, 1, [$(($param_name:ident: $param_type:ty, $param_conv:ty)),+ $(,)?]) => {
        impl $name {
            pub fn from($($param_name: $param_conv),+) -> Self {
                $name(Constr1::new(Box::new((
                    $(<$param_type>::new($param_name),)+
                ))))
            }
        }
    };

    ($name:ident, 2, [$(($param_name:ident: $param_type:ty, $param_conv:ty)),+ $(,)?]) => {
        impl $name {
            pub fn from($($param_name: $param_conv),+) -> Self {
                $name(Constr1::new(Box::new((
                    $(<$param_type>::new($param_name),)+
                ))))
            }
        }
    };

    ($name:ident, $tag:expr, [$(($param_name:ident: $param_type:ty, $param_conv:ty)),+ $(,)?]) => {
        impl $name {
            pub fn from($($param_name: $param_conv),+) -> Self {
                $name(Constr::new($tag, Box::new((
                    $(<$param_type>::new($param_name),)+
                ))))
            }
        }
    };
}
