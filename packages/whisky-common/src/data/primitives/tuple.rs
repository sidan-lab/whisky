use serde_json::{json, Value};

use crate::data::PlutusDataJson;

#[derive(Clone, Debug)]
pub struct Tuple<T>
where
    T: Clone + PlutusDataJson,
{
    pub items: T,
}

impl<T> Tuple<T>
where
    T: Clone + PlutusDataJson,
{
    pub fn new(items: T) -> Self {
        Tuple { items }
    }
}

impl<T> PlutusDataJson for Tuple<T>
where
    T: Clone + PlutusDataJson,
{
    fn to_json(&self) -> Value {
        let items_json = self.items.to_constr_field();
        tuple(items_json)
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
        impl<$($name,)+> PlutusDataJson for ($($name,)+)
        where
            $($name: PlutusDataJson + Clone,)+
        {
            fn to_json(&self) -> Value {
                json!(self.to_constr_field())
            }
            fn to_constr_field(&self) -> Vec<Value> {
                let ($($name,)+) = self.clone();
                vec![$($name.to_json(),)+]
            }
        }
    }
}

impl_plutus_data_tuple!(T1 T2);
impl_plutus_data_tuple!(T1 T2 T3);
impl_plutus_data_tuple!(T1 T2 T3 T4);
impl_plutus_data_tuple!(T1 T2 T3 T4 T5);
