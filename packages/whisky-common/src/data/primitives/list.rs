use serde_json::{json, Value};

use crate::{data::PlutusDataJson, WError};

#[derive(Clone, Debug, PartialEq)]
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

    fn from_json(value: &Value) -> Result<Self, WError> {
        let items_json = value
            .get("list")
            .ok_or_else(|| WError::new("List::from_json", "missing 'list' field"))?
            .as_array()
            .ok_or_else(|| WError::new("List::from_json", "invalid 'list' value"))?;

        let items = items_json
            .iter()
            .enumerate()
            .map(|(i, item)| {
                T::from_json(item).map_err(WError::add_err_trace(
                    Box::leak(format!("List::from_json[{}]", i).into_boxed_str())
                ))
            })
            .collect::<Result<Vec<T>, WError>>()?;

        Ok(List { items })
    }
}

pub fn list<T: Into<Value>>(p_list: Vec<T>) -> Value {
    let list: Vec<Value> = p_list.into_iter().map(|item| item.into()).collect();
    json!({ "list": list })
}
