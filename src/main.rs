#[macro_use]
extern crate lazy_static;

use std::{env::args, path::PathBuf, sync::Arc};

use anyhow::Result;
use config::Config;
use futures::{stream::FuturesUnordered, StreamExt};

use crate::files::files_from_globs;

mod config;
mod files;
mod plugins;
mod script;
mod sort;
use sort::sort_file;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(Config::from_file().unwrap_or_default());

    let (tw_config, plugins) = script::run(&config.tw_config).await?;

    let arg_files: Vec<_> = args().skip(1).map(PathBuf::from).collect();
    let files: Vec<_> = if !arg_files.is_empty() {
        arg_files
    } else {
        files_from_globs(&config.files)
    };

    let classes_order = Arc::new(plugins::from_plugins(plugins)?);
    let mut screens: Vec<_> = tw_config
        .theme
        .screens
        .keys()
        .map(String::to_owned)
        .collect();
    let mut variants: Vec<_> = tw_config.variant_order.into_iter().collect();
    let mut states_order: Vec<String> = Vec::with_capacity(0);
    states_order.append(&mut screens);
    states_order.append(&mut variants);

    let states_order = Arc::new(states_order);
    let mut workers = FuturesUnordered::new();
    for file in files {
        let config = config.clone();
        let classes_order = classes_order.clone();
        let states_order = states_order.clone();
        workers.push(tokio::spawn(async move {
            sort_file(file, &config, &classes_order, &states_order).await
        }));
    }

    while let Some(result) = workers.next().await {
        result??;
    }

    Ok(())
}
