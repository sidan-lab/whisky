use serde_json::{json, Value};

use crate::data::{PlutusDataToJson, ToJsonArray};

#[derive(Clone, Debug)]
pub struct Tuple<T>
where
    T: Clone + PlutusDataToJson + ToJsonArray,
{
    pub items: T,
}

impl<T> Tuple<T>
where
    T: Clone + PlutusDataToJson + ToJsonArray,
{
    pub fn new(items: T) -> Self {
        Tuple { items }
    }
}

impl<T> PlutusDataToJson for Tuple<T>
where
    T: Clone + PlutusDataToJson + ToJsonArray,
{
    fn to_json(&self) -> Value {
        let items_json = self.items.to_json_array();
        tuple(items_json)
    }

    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }
}

pub fn tuple<T: Into<Value>>(p_tuple: Vec<T>) -> Value {
    let list: Vec<Value> = p_tuple.into_iter().map(|item| item.into()).collect();
    json!({ "list": list })
}

#[macro_export]
macro_rules! impl_plutus_data_tuple {
    ( $( $name:ident )+ ) => {
        #[allow(non_snake_case)]
        impl<$($name,)+> ToJsonArray for ($($name,)+)
        where
            $($name: PlutusDataToJson + Clone,)+
        {
            fn to_json_array(&self) -> Vec<Value> {
                let ($($name,)+) = self.clone();
                vec![$($name.to_json(),)+]
            }
        }

        #[allow(non_snake_case)]
        impl<$($name,)+> PlutusDataToJson for ($($name,)+)
        where
            $($name: PlutusDataToJson + Clone,)+
        {
            fn to_json(&self) -> Value {
                json!(self.to_json_array())
            }

            fn to_json_string(&self) -> String {
                self.to_json().to_string()
            }
        }
    }
}

impl_plutus_data_tuple!(T1 T2);
impl_plutus_data_tuple!(T1 T2 T3);
impl_plutus_data_tuple!(T1 T2 T3 T4);
impl_plutus_data_tuple!(T1 T2 T3 T4 T5);
