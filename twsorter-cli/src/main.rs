use std::{env::args, path::PathBuf, sync::Arc};

use anyhow::Result;
use futures::{stream::FuturesUnordered, StreamExt};
use twsorter::{config::Config, plugins, sort::sort_file};

mod script;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(Config::from_file().unwrap_or_default());

    let (tw_config, plugins) = script::run(&config.tw_config).await?;

    let arg_files: Vec<_> = args().skip(1).map(PathBuf::from).collect();
    let files_patterns: Vec<_> = if !arg_files.is_empty() {
        // We have files passed in from the arguments, find which patterns match the files
        config
            .patterns
            .iter()
            .filter_map(|pattern| {
                glob::Pattern::new(&pattern.glob)
                    .map(|pattern| {
                        arg_files
                            .clone()
                            .into_iter()
                            .filter(|arg_file| pattern.matches_path(arg_file))
                            .collect::<Vec<_>>()
                    })
                    .map(|files| if files.is_empty() { None } else { Some(files) })
                    .transpose()
            })
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
    } else {
        // files_from_globs(&config.files)
        vec![vec![]]
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

    let start = chrono::Utc::now();
    sort_file("./index.html", &config, &classes_order, &states_order).await?;
    let end = chrono::Utc::now();
    let duration = end - start;
    println!("Time taken: {}ms", duration.num_milliseconds());

    let mut workers = FuturesUnordered::new();
    println!("HERE");
    for files in files_patterns {
        println!("{:?}", files);
        for file in files {
            let config = config.clone();
            let classes_order = classes_order.clone();
            let states_order = states_order.clone();
            workers.push(async move {
                let start = chrono::Utc::now();
                let result = sort_file(file, &config, &classes_order, &states_order).await;
                let end = chrono::Utc::now();
                let duration = end - start;
                println!("Time taken: {}ms", duration.num_milliseconds());
                result
            });
        }
    }

    while let Some(result) = workers.next().await {
        result?;
    }

    Ok(())
}
