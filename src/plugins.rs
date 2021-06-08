use anyhow::Result;
use regex::Regex;

use self::schema::{Node, Plugins};

pub mod schema;

lazy_static! {
    static ref CLASS_RE: Regex = Regex::new(r#"\.([^\s,]*)"#).unwrap();
}

fn walk_nodes<F>(nodes: &[Node], cb: &mut F)
where
    F: FnMut(&Node),
{
    for node in nodes {
        cb(node);
        walk_nodes(&node.nodes, cb);
    }
}

pub fn from_plugins(root: Plugins) -> Result<Vec<String>> {
    let mut classes: Vec<String> = Vec::with_capacity(root.components.len());

    for component in root.components {
        walk_nodes(&component.nodes, &mut |node| {
            if let Some(selector) = &node.selector {
                let mut cs: Vec<_> = CLASS_RE
                    .captures_iter(selector)
                    .filter_map(|cap| {
                        if cap.len() >= 2 {
                            Some(cap[1].replace("\\", ""))
                        } else {
                            None
                        }
                    })
                    .collect();

                classes.append(&mut cs);
            }
        });
    }

    for utility in root.utilities {
        walk_nodes(&utility.nodes, &mut |node| {
            if let Some(selector) = &node.selector {
                let mut cs: Vec<_> = CLASS_RE
                    .captures_iter(selector)
                    .filter_map(|cap| {
                        if cap.len() >= 2 {
                            Some(cap[1].replace("\\", ""))
                        } else {
                            None
                        }
                    })
                    .collect();

                classes.append(&mut cs);
            }
        });
    }

    classes.dedup();

    // let output_file = File::create("./output.json").unwrap();
    // serde_json::to_writer_pretty(output_file, &classes)?;

    Ok(classes)
}
