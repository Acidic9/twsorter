use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugins {
    #[serde(default)]
    pub base: Vec<Plugin>,
    #[serde(default)]
    pub components: Vec<Plugin>,
    #[serde(default)]
    pub utilities: Vec<Plugin>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Raws {
    pub before: Option<String>,
    pub left: Option<String>,
    pub right: Option<String>,
    pub between: Option<String>,
    pub semicolon: Option<bool>,
    pub after: Option<String>,
    pub value: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub raws: Raws,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: Option<String>,
    pub text: Option<String>,
    pub source: Option<Source>,
    #[serde(default)]
    pub nodes: Vec<Node>,
    pub selector: Option<String>,
    pub prop: Option<String>,
    pub value: Option<String>,
    pub params: Option<String>,
    pub last_each: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub input_id: i64,
    pub start: Marker,
    pub end: Marker,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Marker {
    pub offset: i64,
    pub line: i64,
    pub column: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub value: String,
    pub raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    #[serde(rename = "hasBOM")]
    pub has_bom: bool,
    pub css: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub raws: Raws,
    pub name: String,
    pub params: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub nodes: Vec<Node>,
    pub inputs: Vec<::serde_json::Value>,
}
