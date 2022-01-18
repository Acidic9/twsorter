use serde::Deserialize;
use twsorter::{
    plugins::{self, schema::Plugins},
    sort::sort_classes,
    twconfig::TwConfig,
};

#[derive(Deserialize)]
pub struct Node {
    pub attrs: Vec<Node>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub children: Vec<Node>,
}

fn main() {
    let mut ast: Node = serde_json::from_str(include_str!("./ast.json")).unwrap();
    let classes_order: Vec<String> =
        serde_json::from_str(include_str!("./classes_order.json")).unwrap();
    let states_order: Vec<String> =
        serde_json::from_str(include_str!("./states_order.json")).unwrap();

    let sort_classes =
        move |classes_str: &str| sort_classes(&classes_order, &states_order, classes_str);

    println!("starting...");
    let start = chrono::Utc::now();
    format_children(&mut ast.children, &sort_classes);
    let end = chrono::Utc::now() - start;
    println!("{}", end.num_milliseconds());
}

fn format_children<F>(children: &mut Vec<Node>, sorter: &F)
where
    F: Fn(&str) -> String,
{
    for node in children {
        format_node(node, sorter);
    }
}

fn format_node<F>(node: &mut Node, sorter: &F)
where
    F: Fn(&str) -> String,
{
    for attr in &mut node.attrs {
        if attr
            .name
            .as_ref()
            .map(|name| name == "class")
            .unwrap_or(false)
        {
            let classes = attr.value.as_deref().unwrap_or_default();
            attr.value = Some(sorter(classes));
        }
    }

    format_children(&mut node.children, sorter);
}
