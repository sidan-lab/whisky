use serde_json::{json, Value};

use crate::{data::{PlutusData, PlutusDataJson}, WError};

#[derive(Clone, Debug, PartialEq)]
pub struct Tuple<T = PlutusData>
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

    fn from_json(value: &Value) -> Result<Self, WError> {
        // Tuple is stored as {"list": [...]}
        let list = value
            .get("list")
            .ok_or_else(|| WError::new("Tuple::from_json", "missing 'list' field"))?;

        let items = T::from_json(list)
            .map_err(WError::add_err_trace("Tuple::from_json"))?;

        Ok(Tuple { items })
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
            fn from_json(value: &Value) -> Result<Self, WError> {
                let arr = value
                    .as_array()
                    .ok_or_else(|| WError::new("tuple::from_json", "expected array"))?;

                let mut iter = arr.iter();
                $(
                    let $name = {
                        let item = iter.next()
                            .ok_or_else(|| WError::new("tuple::from_json", "not enough elements"))?;
                        $name::from_json(item)
                            .map_err(WError::add_err_trace("tuple::from_json"))?
                    };
                )+

                Ok(($($name,)+))
            }
        }
    }
}

impl_plutus_data_tuple!(T1);
impl_plutus_data_tuple!(T1 T2);
impl_plutus_data_tuple!(T1 T2 T3);
impl_plutus_data_tuple!(T1 T2 T3 T4);
impl_plutus_data_tuple!(T1 T2 T3 T4 T5);
