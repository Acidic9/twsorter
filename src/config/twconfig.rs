use hashlink::LinkedHashMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwConfig {
    pub theme: Theme,
    // pub variants: Variants,
    // pub core_plugins: Vec<String>,
    // pub plugins: Vec<::serde_json::Value>,
    // pub mode: String,
    // pub purge: Vec<String>,
    // pub presets: Vec<::serde_json::Value>,
    // pub dark_mode: bool,
    pub variant_order: Vec<String>,
    // pub prefix: String,
    // pub important: bool,
    pub separator: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub screens: LinkedHashMap<String, String>,
}
