use serde_json::{json, Value};

use crate::data::PlutusDataJson;

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
        Constr {
            tag,
            fields: fields,
        }
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
}

impl PlutusDataJson for () {
    fn to_json(&self) -> Value {
        json!([])
    }
    fn to_constr_field(&self) -> Vec<serde_json::Value> {
        vec![]
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

#[macro_export]
macro_rules! impl_constr_fields {
    ( $( $name:ident )+ ) => {
        #[allow(non_snake_case)]
        impl<$($name,)+> PlutusDataJson for Box<($($name,)+)>
        where
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
        }
    }
}

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

#[macro_export]
macro_rules! impl_constr_n {
    ($($name:ident: $tag:expr),+) => {
        $(
            #[derive(Clone, Debug)]
            pub struct $name<T = ()>
            where
                T: Clone + PlutusDataJson + ,
            {
                pub fields: T,
            }

            impl<T> $name<T>
            where
                T: Clone + PlutusDataJson + ,
            {
                pub fn new(fields: T) -> Self {
                    $name {
                        fields,
                    }
                }
            }

            impl<T> PlutusDataJson for $name<T>
            where
                T: Clone + PlutusDataJson + ,
            {
                fn to_json(&self) -> Value {
                    let fields_json = self.fields.to_constr_field();
                    constr($tag, fields_json)
                }
            }
        )+
    }
}

impl_constr_n!(
    Constr0: 0,
    Constr1: 1,
    Constr2: 2,
    Constr3: 3
);
