use serde_json::{json, Value};

use crate::{data::PlutusDataJson, WError};

#[derive(Clone, Debug)]
pub struct Constr<T = ()>
where
    T: Clone + PlutusDataJson,
{
    pub tag: u64,
    pub fields: T,
}

impl<T> Constr<T>
where
    T: Clone + PlutusDataJson,
{
    pub fn new(tag: u64, fields: T) -> Self {
        Constr { tag, fields }
    }
}

impl<T> PlutusDataJson for Constr<T>
where
    T: Clone + PlutusDataJson,
{
    fn to_json(&self) -> Value {
        let fields_json = self.fields.to_constr_field();
        constr(self.tag, fields_json)
    }

    fn from_json(value: &Value) -> Result<Self, WError> {
        let tag = value
            .get("constructor")
            .ok_or_else(|| WError::new("Constr::from_json", "missing 'constructor' field"))?
            .as_u64()
            .ok_or_else(|| WError::new("Constr::from_json", "invalid 'constructor' value"))?;

        let fields_json = value
            .get("fields")
            .ok_or_else(|| WError::new("Constr::from_json", "missing 'fields' field"))?;

        let fields = T::from_constr_field(fields_json)
            .map_err(WError::add_err_trace("Constr::from_json"))?;

        Ok(Constr { tag, fields })
    }
}

impl PlutusDataJson for () {
    fn to_json(&self) -> Value {
        json!([])
    }
    fn to_constr_field(&self) -> Vec<serde_json::Value> {
        vec![]
    }
    fn from_json(value: &Value) -> Result<Self, WError> {
        // Accept both empty array and empty fields
        if value.is_array() {
            let arr = value.as_array().unwrap();
            if arr.is_empty() {
                return Ok(());
            }
        }
        Err(WError::new("()::from_json", "expected empty array"))
    }
}

// value constructor

pub fn constr<N: Into<Value>, T: Into<Value>>(constructor: N, fields: T) -> Value {
    json!({ "constructor": constructor.into(), "fields": fields.into() })
}

pub fn constr0<T: Into<Value>>(fields: T) -> Value {
    constr(0, fields)
}

pub fn constr1<T: Into<Value>>(fields: T) -> Value {
    constr(1, fields)
}

pub fn constr2<T: Into<Value>>(fields: T) -> Value {
    constr(2, fields)
}

/// Deprecated: Use `constr` instead.
pub fn con_str<N: Into<Value>, T: Into<Value>>(constructor: N, fields: T) -> Value {
    json!({ "constructor": constructor.into(), "fields": fields.into() })
}

/// Deprecated: Use `constr0` instead.
pub fn con_str0<T: Into<Value>>(fields: T) -> Value {
    con_str(0, fields)
}

/// Deprecated: Use `constr1` instead.
pub fn con_str1<T: Into<Value>>(fields: T) -> Value {
    con_str(1, fields)
}

/// Deprecated: Use `constr2` instead.
pub fn con_str2<T: Into<Value>>(fields: T) -> Value {
    con_str(2, fields)
}

/// Wrapper for tuples that provides Debug implementation for tuples larger than 12 elements
/// (Rust's std only implements Debug for tuples up to 12 elements)
/// Use this when you have constructor fields with 13+ elements that need Debug support.
#[repr(transparent)]
pub struct ConstrFields<T>(pub T);

impl<T: Clone> Clone for ConstrFields<T> {
    fn clone(&self) -> Self {
        ConstrFields(self.0.clone())
    }
}

#[macro_export]
macro_rules! impl_constr_fields {
    ( $( $name:ident )+ ) => {
        // Implement PlutusDataJson for Box<(T1, T2, ...)>
        // This only works for tuples up to 12 elements due to Rust's Debug trait limit
        #[allow(non_snake_case)]
        impl<$($name,)+> PlutusDataJson for Box<($($name,)+)>
        where
            ($($name,)+): ::core::fmt::Debug,
            $($name: PlutusDataJson + Clone,)+
        {
            fn to_json(&self) -> Value {
                json!(self.to_constr_field())
            }

            fn to_constr_field(&self) -> Vec<Value> {
                let tuple = &**self;
                let ($($name,)+) = tuple.clone();
                vec![$($name.to_json(),)+]
            }

            fn from_json(value: &Value) -> Result<Self, WError> {
                let arr = value
                    .as_array()
                    .ok_or_else(|| WError::new("Box<tuple>::from_json", "expected array"))?;

                let mut iter = arr.iter();
                $(
                    let $name = {
                        let item = iter.next()
                            .ok_or_else(|| WError::new("Box<tuple>::from_json", "not enough elements"))?;
                        $name::from_json(item)
                            .map_err(WError::add_err_trace("Box<tuple>::from_json"))?
                    };
                )+

                Ok(Box::new(($($name,)+)))
            }
        }

        // Implement PlutusDataJson for Box<ConstrFields<(T1, T2, ...)>>
        // Use this for tuples with 13+ elements
        #[allow(non_snake_case)]
        impl<$($name,)+> PlutusDataJson for Box<ConstrFields<($($name,)+)>>
        where
            $($name: PlutusDataJson + Clone,)+
        {
            fn to_json(&self) -> Value {
                json!(self.to_constr_field())
            }

            fn to_constr_field(&self) -> Vec<Value> {
                let tuple = &self.0;
                let ($($name,)+) = tuple.clone();
                vec![$($name.to_json(),)+]
            }

            fn from_json(value: &Value) -> Result<Self, WError> {
                let arr = value
                    .as_array()
                    .ok_or_else(|| WError::new("Box<ConstrFields>::from_json", "expected array"))?;

                let mut iter = arr.iter();
                $(
                    let $name = {
                        let item = iter.next()
                            .ok_or_else(|| WError::new("Box<ConstrFields>::from_json", "not enough elements"))?;
                        $name::from_json(item)
                            .map_err(WError::add_err_trace("Box<ConstrFields>::from_json"))?
                    };
                )+

                Ok(Box::new(ConstrFields(($($name,)+))))
            }
        }

        // Implement Debug for ConstrFields
        #[allow(non_snake_case)]
        impl<$($name,)+> ::core::fmt::Debug for ConstrFields<($($name,)+)>
        where
            $($name: ::core::fmt::Debug,)+
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let tuple = &self.0;
                let ($($name,)+) = tuple;
                f.debug_tuple("")
                    $(.field($name))+
                    .finish()
            }
        }
    }
}

// Implement for tuples 2-12 (these work with both Box<(...)> and Box<ConstrFields<(...)>>)
impl_constr_fields!(T1 T2);
impl_constr_fields!(T1 T2 T3);
impl_constr_fields!(T1 T2 T3 T4);
impl_constr_fields!(T1 T2 T3 T4 T5);
impl_constr_fields!(T1 T2 T3 T4 T5 T6);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12);

// Implement for tuples 13-20 (these MUST use Box<ConstrFields<(...)>> instead of Box<(...)>)
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19);
impl_constr_fields!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20);

#[macro_export]
macro_rules! impl_constr_n {
    ($($name:ident: $tag:expr),+) => {
        $(
            #[derive(Clone, Debug, PartialEq)]
            pub struct $name<T = ()>
            where
                T: Clone + PlutusDataJson,
            {
                pub fields: T,
            }

            impl<T> $name<T>
            where
                T: Clone + PlutusDataJson,
            {
                pub fn new(fields: T) -> Self {
                    $name { fields }
                }
            }

            impl<T> PlutusDataJson for $name<T>
            where
                T: Clone + PlutusDataJson,
            {
                fn to_json(&self) -> Value {
                    let fields_json = self.fields.to_constr_field();
                    constr($tag, fields_json)
                }

                fn from_json(value: &Value) -> Result<Self, WError> {
                    let tag = value
                        .get("constructor")
                        .ok_or_else(|| WError::new(concat!(stringify!($name), "::from_json"), "missing 'constructor' field"))?
                        .as_u64()
                        .ok_or_else(|| WError::new(concat!(stringify!($name), "::from_json"), "invalid 'constructor' value"))?;

                    if tag != $tag {
                        return Err(WError::new(
                            concat!(stringify!($name), "::from_json"),
                            &format!("expected constructor tag {}, got {}", $tag, tag),
                        ));
                    }

                    let fields_json = value
                        .get("fields")
                        .ok_or_else(|| WError::new(concat!(stringify!($name), "::from_json"), "missing 'fields' field"))?;

                    let fields = T::from_constr_field(fields_json)
                        .map_err(WError::add_err_trace(concat!(stringify!($name), "::from_json")))?;

                    Ok($name { fields })
                }
            }
        )+
    }
}

impl_constr_n!(
    Constr0: 0,
    Constr1: 1,
    Constr2: 2,
    Constr3: 3,
    Constr4: 4,
    Constr5: 5,
    Constr6: 6,
    Constr7: 7,
    Constr8: 8,
    Constr9: 9,
    Constr10: 10
);

// Implement PlutusDataJson for Box<ConstrN<T>> to support boxed wrapper types
macro_rules! impl_box_constr {
    ($($constr:ident),+) => {
        $(
            impl<T> PlutusDataJson for Box<$constr<T>>
            where
                T: Clone + PlutusDataJson,
            {
                fn to_json(&self) -> Value {
                    self.as_ref().to_json()
                }

                fn to_json_string(&self) -> String {
                    self.to_json().to_string()
                }

                fn to_constr_field(&self) -> Vec<Value> {
                    vec![self.to_json()]
                }

                fn from_json(value: &Value) -> Result<Self, WError> {
                    $constr::<T>::from_json(value).map(Box::new)
                }
            }
        )+
    };
}

impl_box_constr!(
    Constr, Constr0, Constr1, Constr2, Constr3, Constr4, Constr5, Constr6, Constr7, Constr8,
    Constr9, Constr10
);
