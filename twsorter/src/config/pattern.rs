use std::str::FromStr;

use regex::Regex;
use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug)]
pub struct Pattern(pub Regex);

impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Regex::new(&s).map(Pattern).map_err(D::Error::custom)
    }
}

impl FromStr for Pattern {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Regex::new(s).map(Pattern)
    }
}
