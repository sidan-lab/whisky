use serde_json::{json, Value};

use crate::{PlutusDataToJson, ToJsonArray};

#[derive(Clone, Debug)]
pub struct Constr<T>
where
    T: Clone + PlutusDataToJson + ToJsonArray,
{
    pub tag: u64,
    pub fields: T,
}

impl<T> Constr<T>
where
    T: Clone + PlutusDataToJson + ToJsonArray,
{
    pub fn new(tag: u64, fields: T) -> Self {
        Constr {
            tag,
            fields: fields,
        }
    }
}

impl<T> PlutusDataToJson for Constr<T>
where
    T: Clone + PlutusDataToJson + ToJsonArray,
{
    fn to_json(&self) -> Value {
        let fields_json = self.fields.to_constr_fields_array();
        constr(self.tag, fields_json)
    }

    fn to_json_string(&self) -> String {
        self.to_json().to_string()
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
macro_rules! impl_plutus_data_tuple {
    ( $( $name:ident )+ ) => {
        #[allow(non_snake_case)]
        impl<$($name,)+> ToJsonArray for Box<($($name,)+)>
        where
            $($name: PlutusDataToJson + Clone,)+
        {
            fn to_constr_fields_array(&self) -> Vec<Value> {
                let tuple = &**self; // Properly dereference Box<Tuple>
                let ($($name,)+) = tuple.clone();
                vec![$($name.to_json(),)+]
            }
        }

        #[allow(non_snake_case)]
        impl<$($name,)+> PlutusDataToJson for Box<($($name,)+)>
        where
            $($name: PlutusDataToJson + Clone,)+
        {
            fn to_json(&self) -> Value {
                json!(self.to_constr_fields_array())
            }

            fn to_json_string(&self) -> String {
                self.to_json().to_string()
            }
        }
    }
}

impl_plutus_data_tuple!(T1);
impl_plutus_data_tuple!(T1 T2);
impl_plutus_data_tuple!(T1 T2 T3);
impl_plutus_data_tuple!(T1 T2 T3 T4);
impl_plutus_data_tuple!(T1 T2 T3 T4 T5);

// Macro to generate Constr0 through Constr10
macro_rules! impl_constr_n {
    ($($name:ident: $tag:expr),+) => {
        $(
            #[derive(Clone, Debug)]
            pub struct $name<T>
            where
                T: Clone + PlutusDataToJson + ToJsonArray,
            {
                pub fields: T,
            }

            impl<T> $name<T>
            where
                T: Clone + PlutusDataToJson + ToJsonArray,
            {
                pub fn new(fields: T) -> Self {
                    $name {
                        fields,
                    }
                }
            }

            impl<T> PlutusDataToJson for $name<T>
            where
                T: Clone + PlutusDataToJson + ToJsonArray,
            {
                fn to_json(&self) -> Value {
                    let fields_json = self.fields.to_constr_fields_array();
                    constr($tag, fields_json)
                }

                fn to_json_string(&self) -> String {
                    self.to_json().to_string()
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
