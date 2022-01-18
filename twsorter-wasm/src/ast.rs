use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ASTRoot {
    #[serde(default)]
    pub children: Vec<Node>,
    #[serde(rename = "type")]
    pub ty: String,
    pub source_span: Span,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub value: Option<String>,
    pub source_span: Span,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(default)]
    pub attrs: Vec<Attr>,
    #[serde(default)]
    pub children: Vec<Node>,
    pub name: Option<String>,
    pub start_source_span: Option<Span>,
    pub end_source_span: Option<Span>,
    pub name_span: Option<Span>,
    pub has_explicit_namespace: Option<bool>,
    pub tag_definition: Option<TagDefinition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Span {
    pub start: Loc,
    pub end: Loc,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Loc {
    pub file: File,
    pub offset: i64,
    pub line: i64,
    pub col: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub content: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attr {
    pub name: String,
    pub value: String,
    pub source_span: Span,
    pub value_span: Span,
    pub name_span: Span,
    #[serde(rename = "type")]
    pub type_field: String,
    pub has_explicit_namespace: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagDefinition {
    pub closed_by_parent: bool,
    pub can_self_close: bool,
    pub is_void: bool,
    pub content_type: i64,
    pub ignore_first_lf: bool,
}
