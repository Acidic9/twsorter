use std::{env::current_dir, fs::File};

use anyhow::{Context, Result};
use serde::Deserialize;

use self::glob_pattern::GlobPattern;

pub mod glob_pattern;
pub mod pattern;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "config_default_patterns")]
    pub patterns: Vec<GlobPattern>,
    #[serde(default = "config_default_tw_config")]
    pub tw_config: String,
}

impl Config {
    pub fn from_file() -> Result<Self> {
        let config_path = current_dir()
            .context("current directory")?
            .join("twsorter.config.yaml");
        let config_file = File::open(config_path).context("read config file")?;

        serde_yaml::from_reader(config_file).map_err(anyhow::Error::from)
    }

    fn detect_tw_config_file() -> Option<&'static str> {
        let search_files = vec!["tailwind.config.js", "tailwind.config.cjs"];
        if let Ok(dir) = current_dir() {
            for search_file in search_files {
                if dir.join(search_file).exists() {
                    return Some(search_file);
                }
            }
        }

        None
    }
}

fn config_default_patterns() -> Vec<GlobPattern> {
    vec![GlobPattern {
        glob: "src/**/*".to_string(),
        pattern: r#"class(?:Name)?=["'](.*)["']"#.parse().unwrap(),
    }]
}

fn config_default_tw_config() -> String {
    Config::detect_tw_config_file()
        .unwrap_or("tailwind.config.js")
        .to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            patterns: config_default_patterns(),
            tw_config: config_default_tw_config(),
        }
    }
}
