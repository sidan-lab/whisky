use serde_json::{json, Value};

use crate::data::PlutusDataJson;

#[derive(Clone, Debug)]
pub struct Int {
    pub int: i128,
}

impl Int {
    pub fn new(int: i128) -> Self {
        Int { int }
    }
}

impl PlutusDataJson for Int {
    fn to_json(&self) -> Value {
        integer(self.int)
    }
}

pub fn integer(int: i128) -> Value {
    json!({ "int": int })
}

pub fn posix_time(posix_time: i128) -> Value {
    integer(posix_time)
}
