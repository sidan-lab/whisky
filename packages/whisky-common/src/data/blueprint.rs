use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlutusVersion {
    V1,
    V2,
    V3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Compiler {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Preamble {
    pub title: String,
    pub description: String,
    pub version: String,
    #[serde(rename = "plutusVersion")]
    pub plutus_version: PlutusVersion,
    pub compiler: Compiler,
    pub license: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub reference: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ByteDefinition {
    pub title: String,
    #[serde(rename = "dataType")]
    pub data_type: String, // "bytes"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataDefinition {
    pub title: String, // "Data"
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstructorField {
    pub title: String,
    #[serde(rename = "dataType")]
    pub data_type: String,
    pub index: u32,
    pub fields: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoolDefinition {
    pub title: String, // "Bool"
    #[serde(rename = "anyOf")]
    pub any_of: [ConstructorField; 2], // False and True
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntDefinition {
    #[serde(rename = "dataType")]
    pub data_type: String, // "integer"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapDefinition {
    pub title: String,
    #[serde(rename = "dataType")]
    pub data_type: String, // "map"
    pub keys: Reference,
    pub values: Reference,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListDefinition {
    pub title: String,
    #[serde(rename = "dataType")]
    pub data_type: String, // "list"
    pub items: Reference,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TupleDefinition {
    pub title: String,
    #[serde(rename = "dataType")]
    pub data_type: String, // "list"
    pub items: Vec<Reference>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SomeConstructor {
    pub title: String, // "Some"
    pub description: String,
    #[serde(rename = "dataType")]
    pub data_type: String, // "constructor"
    pub index: u32, // 0
    pub fields: Vec<Reference>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoneConstructor {
    pub title: String, // "None"
    pub description: String,
    #[serde(rename = "dataType")]
    pub data_type: String, // "constructor"
    pub index: u32,                     // 1
    pub fields: Vec<serde_json::Value>, // empty
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionDefinition {
    pub title: String, // "Option"
    #[serde(rename = "anyOf")]
    pub any_of: [serde_json::Value; 2], // Some and None
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstructorDefinition {
    pub title: String,
    #[serde(rename = "dataType")]
    pub data_type: String, // "constructor"
    pub index: u32,
    pub fields: Vec<Reference>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstructorsDefinition {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "anyOf")]
    pub any_of: Vec<ConstructorDefinition>,
}

pub type IgnoreDefinition = DataDefinition;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomDefinition {
    Map(MapDefinition),
    List(ListDefinition),
    Tuple(TupleDefinition),
    Option(OptionDefinition),
    Constructors(ConstructorsDefinition),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrimitiveDefinition {
    Int(IntDefinition),
    Byte(ByteDefinition),
    Bool(BoolDefinition),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Items {
    Single(Reference),
    Multiple(Vec<Reference>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnyOf {
    Constructors(Vec<ConstructorDefinition>),
    Option([serde_json::Value; 2]), // Some and None
    Bool([serde_json::Value; 2]),   // False and True
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Definition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "dataType", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    pub any_of: Option<AnyOf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Items>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Reference>>,
}

pub type Definitions = HashMap<String, Definition>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub title: String,
    pub schema: Reference,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Redeemer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub schema: Schema,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Datum {
    pub title: String,
    pub schema: Reference,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Validator {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemer: Option<Redeemer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datum: Option<Datum>,
    #[serde(rename = "compiledCode")]
    pub compiled_code: String,
    pub hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScriptPurpose {
    Spend,
    Mint,
    Withdraw,
    Publish,
}

/// Main Blueprint structure containing preamble, validators, and definitions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blueprint {
    pub preamble: Preamble,
    pub validators: Vec<Validator>,
    pub definitions: Definitions,
}
