mod ast;
// mod ast2;
mod utils;

use ast::ASTRoot;
use serde::Serialize;
use twsorter::{
    plugins::{self, schema::Plugins},
    sort::sort_classes,
    twconfig::TwConfig,
};
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
extern "C" {
    pub type vs;

    #[wasm_bindgen(method, getter = type)]
    pub fn ty(this: &vs) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &vs) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &vs) -> Option<String>;

    #[wasm_bindgen(method, setter = value)]
    pub fn set_value(this: &vs, value: &str) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &vs) -> Option<Vec<vs>>;

    #[wasm_bindgen(method, getter)]
    pub fn attrs(this: &vs) -> Option<Vec<vs>>;
}

#[derive(Serialize)]
pub struct ClassesStates {
    pub classes_order: Vec<String>,
    pub states_order: Vec<String>,
}

#[wasm_bindgen]
pub fn classes_states(tw_config: &JsValue, tw_plugins: &JsValue) -> JsValue {
    set_panic_hook();

    let tw_config: TwConfig = tw_config.into_serde().unwrap();
    let tw_plugins: Plugins = tw_plugins.into_serde().unwrap();

    let start = instant::now();
    let classes_order = plugins::from_plugins(tw_plugins).unwrap();
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
    let end = instant::now() - start;
    log(&end.to_string());

    // let sort_classes =
    //     move |classes_str: &str| sort_classes(&classes_order, &states_order, classes_str);

    // format_children(&mut ast.children(), &sort_classes);

    JsValue::from_serde(&ClassesStates {
        classes_order,
        states_order,
    })
    .unwrap()
}

#[derive(Serialize)]
pub struct Node {
    pub attrs: Vec<Node>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub children: Vec<Node>,
}

impl Node {
    fn from_vs(v: &vs) -> Self {
        Node {
            attrs: v
                .attrs()
                .unwrap_or_default()
                .into_iter()
                .map(|node| Node::from_vs(&node))
                .collect(),
            name: v.name(),
            value: v.value(),
            children: v
                .children()
                .unwrap_or_default()
                .into_iter()
                .map(|node| Node::from_vs(&node))
                .collect(),
        }
    }
}

// impl From<vs> for Node {
//     fn from(v: vs) -> Self {
//         Node {
//             name: v.name(),
//             value: v.value(),
//             children: v
//                 .children()
//                 .unwrap_or_default()
//                 .into_iter()
//                 .map(|node| node.into())
//                 .collect(),
//         }
//     }
// }

// impl std::iter::Iterator for vs {
//     type Item = Vec<Node>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.children()
//             .map(|nodes| nodes.into_iter().map(|node| node.into()).collect())
//     }
// }

// fn collect_node_children(node: &mut Node, v: &vs) {
//     v.children().iter().map(|child| )
// }

#[wasm_bindgen]
pub fn format_ast(classes_order: &JsValue, states_order: &JsValue, ast: vs) {
    set_panic_hook();

    // let node = Node::from_vs(&ast);
    // let json = serde_json::to_string(&node).unwrap();
    // log(&json);

    // let items: Node = ast
    //     .children()
    //     .unwrap_or_default()
    //     .into_iter()
    //     .fold(ast.into(), |mut acc, nodes| {});

    let classes_order: Vec<String> = classes_order.into_serde().unwrap();
    let states_order: Vec<String> = states_order.into_serde().unwrap();
    // log("into serde");
    // let ast: ASTRoot = ast.into_serde().unwrap();
    // log("converting to json");
    // let ast_json = serde_json::to_string(&ast).unwrap();
    // log("converted to json");
    // for chunk in ast_json
    //     .chars()
    //     .collect::<Vec<char>>()
    //     .chunks(500)
    //     .map(|c| c.iter().collect::<String>())
    // {
    //     log(&chunk);
    // }

    let sort_classes =
        move |classes_str: &str| sort_classes(&classes_order, &states_order, classes_str);

    let start = instant::now();
    format_children(&mut ast.children(), &sort_classes);
    let end = instant::now() - start;
    log(&end.to_string());
}

fn format_children<F>(children: &mut Option<Vec<vs>>, sorter: &F)
where
    F: Fn(&str) -> String,
{
    if let Some(children) = children {
        for node in children {
            format_node(node, sorter);
        }
    }
}

fn format_node<F>(node: &mut vs, sorter: &F)
where
    F: Fn(&str) -> String,
{
    // do stuff
    if let Some(attrs) = node.attrs() {
        for attr in attrs {
            if attr.name().map(|name| name == "class").unwrap_or(false) {
                let classes = attr.value();
                // log(classes.as_deref().unwrap_or_default());
                attr.set_value(&sorter(classes.as_deref().unwrap_or_default()));
            }
        }
    }

    format_children(&mut node.children(), sorter);
}
