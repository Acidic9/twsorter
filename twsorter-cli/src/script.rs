use std::{env::current_dir, process::Stdio};

use anyhow::{anyhow, Context, Result};
use home::home_dir;
use tokio::{
    fs::{create_dir_all, read, write},
    process::Command,
};
use twsorter::{plugins::schema::Plugins, twconfig::TwConfig};

pub async fn run(tw_config_file: &str) -> Result<(TwConfig, Plugins)> {
    let cache_path = home_dir()
        .ok_or_else(|| anyhow!("could not detect home directory"))?
        .join(".twsorter")
        .join(
            current_dir()
                .context("could not detect current directory")?
                .file_name()
                .context("could not detect current directory")?,
        );
    create_dir_all(cache_path.clone())
        .await
        .context("create cache dir")?;
    let script_path = cache_path.join("twsorter_script.js");

    // if !script_path.exists() {
    write(script_path.clone(), include_str!("./script.js"))
        .await
        .context("write script file")?;
    // }

    let tw_config_path = cache_path.join("config.json");
    let plugins_path = cache_path.join("plugins.json");

    let tw_config_modified_recently = tw_config_path
        .metadata()
        .and_then(|metadata| metadata.modified())
        .and_then(|modified| {
            current_dir().and_then(|dir| {
                dir.join(tw_config_file)
                    .metadata()
                    .and_then(|tw_config_metadata| {
                        tw_config_metadata
                            .modified()
                            .map(|tw_config_metadata| tw_config_metadata > modified)
                    })
            })
        })
        .unwrap_or(true);

    if tw_config_modified_recently || !tw_config_path.exists() || !plugins_path.exists() {
        let output_status = Command::new("node")
            .arg(script_path.as_os_str())
            .arg(tw_config_file)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await
            .context("execute script file")?;
        if !output_status.success() {
            anyhow!("failed to run script file");
        }
    }

    let tw_config_json_from_content = read(tw_config_path)
        .await
        .context("read tw config json file")?;

    let tw_config: TwConfig = serde_json::from_slice(&tw_config_json_from_content)
        .context("decode tw config json file")?;

    let plugins_json_file_content = read(plugins_path).await.context("read plugins json file")?;

    let plugins: Plugins =
        serde_json::from_slice(&plugins_json_file_content).context("decode plugins json file")?;

    Ok((tw_config, plugins))
}
