use serde_json::{json, Value};

use crate::data::PlutusDataJson;

#[derive(Clone, Debug)]
pub struct List<T>
where
    T: Clone + PlutusDataJson,
{
    pub items: Vec<T>,
}

impl<T> List<T>
where
    T: Clone + PlutusDataJson,
{
    pub fn new(items: &[T]) -> Self {
        List {
            items: items.to_vec(),
        }
    }
}

impl<T> PlutusDataJson for List<T>
where
    T: Clone + PlutusDataJson,
{
    fn to_json(&self) -> Value {
        let items_json = self
            .items
            .iter()
            .map(|item| item.to_json())
            .collect::<Vec<Value>>();
        list(items_json)
    }
}

pub fn list<T: Into<Value>>(p_list: Vec<T>) -> Value {
    let list: Vec<Value> = p_list.into_iter().map(|item| item.into()).collect();
    json!({ "list": list })
}
