use serde_json::{json, Value};

use crate::data::PlutusDataToJson;

#[derive(Clone, Debug)]
pub struct List<T>
where
    T: Clone + PlutusDataToJson,
{
    pub items: Vec<T>,
}

impl<T> List<T>
where
    T: Clone + PlutusDataToJson,
{
    pub fn new(items: &[T]) -> Self {
        List {
            items: items.to_vec(),
        }
    }
}

impl<T> PlutusDataToJson for List<T>
where
    T: Clone + PlutusDataToJson,
{
    fn to_json(&self) -> Value {
        let items_json = self
            .items
            .iter()
            .map(|item| item.to_json())
            .collect::<Vec<Value>>();
        list(items_json)
    }

    fn to_json_string(&self) -> String {
        self.to_json().to_string()
    }
}

pub fn list<T: Into<Value>>(p_list: Vec<T>) -> Value {
    let list: Vec<Value> = p_list.into_iter().map(|item| item.into()).collect();
    json!({ "list": list })
}
