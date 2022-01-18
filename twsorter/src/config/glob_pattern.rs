use serde::Deserialize;

use super::pattern::Pattern;

#[derive(Debug, Deserialize)]
pub struct GlobPattern {
    pub glob: String,
    pub pattern: Pattern,
}
