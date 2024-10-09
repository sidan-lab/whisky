use crate::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct JsVecString(Vec<String>);

#[wasm_bindgen]
impl JsVecString {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> String {
        self.0[index].clone()
    }

    pub fn add(&mut self, elem: String) {
        self.0.push(elem.clone());
    }

    pub(crate) fn into_vec(self) -> Vec<String> {
        self.0
    }
}

impl IntoIterator for JsVecString {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl JsVecString {
    pub fn iter(&self) -> std::slice::Iter<String> {
        self.0.iter()
    }
}
