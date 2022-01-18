const prettier = require("prettier");
const htmlParser = require("prettier/parser-html");
const twsorter = require("../../twsorter-wasm/pkg/twsorter_wasm");
// const TWClassesSorter = require("tailwind-classes-sorter").default;

// const twClassesSorter = new TWClassesSorter();

const { config, plugins } = require("./tailwind");
const { classes_order, states_order } = twsorter.classes_states(
  config,
  plugins
);

// let classes_order = plugins::from_plugins(tw_plugins).unwrap();
//     let mut screens: Vec<_> = tw_config
//         .theme
//         .screens
//         .keys()
//         .map(String::to_owned)
//         .collect();
//     let mut variants: Vec<_> = tw_config.variant_order.into_iter().collect();
//     let mut states_order: Vec<String> = Vec::with_capacity(0);

const htmlParserParse = htmlParser.parsers.html.parse;
htmlParser.parsers.html.parse = function () {
  let ast = htmlParserParse(...arguments);
  // process.exit(0);
  // JSON.stringify(ast);

  // const start = Date.now();

  // console.log("here");
  // format_children(ast.children);

  console.log("formatting...");
  twsorter.format_ast(classes_order, states_order, ast);
  // console.log("done in " + (Date.now() - start) + "ms");
  // process.exit(0);

  return ast;
};

// function format_children(children) {
//   if (children) {
//     for (const child of children) {
//       format_node(child);
//     }
//   }
// }

// function format_node(node) {
//   if (node.attrs) {
//     for (const attr of node.attrs) {
//       if (attr.name === "class" && attr.value) {
//         attr.value = sort_classes(classes_order, states_order, attr.value);
//       }
//     }
//   }

//   format_children(node.children);
// }

// function twclass(str) {
//   let parts = str.split(":");
//   let head = parts[0];
//   let tail = parts[1];
//   if (tail) {
//     return {
//       className: tail,
//       state: head,
//     };
//   } else {
//     return {
//       className: head,
//       state: null,
//     };
//   }
// }

// function sort_classes(classes_order, states_order, classes_str) {
//   // Preserve prefixing and suffixing whitespace (yes I made those words up)
//   let whitespace_before = classes_str.match(/^\s*/)[0].length;
//   let whitespace_after = classes_str.match(/\s*$/)[0].length;

//   let classes = classes_str
//     .split(" ")
//     .map((s) => s.trim())
//     .filter(Boolean);

//   classes.sort((a, b) => {
//     let a_class = twclass(a);
//     let b_class = twclass(b);

//     let a_class_index = classes_order.findIndex((c) => c === a_class.className);
//     let b_class_index = classes_order.findIndex((c) => c === b_class.className);

//     let a_state_index = null;
//     if (a_class.state) {
//       a_state_index = states_order.findIndex((s) => s === a_class.state);
//     }
//     let b_state_index = null;
//     if (b_class.state) {
//       b_state_index = states_order.findIndex((s) => s === b_class.state);
//     }

//     // Sort by state
//     if (!a_class.state && b_class.state) {
//       return -1;
//     }
//     if (a_class.state && !b_class.state) {
//       return 1;
//     }

//     // Both or none have a state at this point
//     if (a_state_index == null && b_state_index == null) {
//       if (a_state_index < b_state_index) {
//         return -1;
//       }
//       if (a_state_index > b_state_index) {
//         return 1;
//       }
//     }

//     // A or B have unknown selector
//     if (a_class_index != null && b_class_index == null) {
//       // B has unknown class
//       return 1;
//     }
//     if (a_class_index == null && b_class_index != null) {
//       // A has unknown class
//       return -1;
//     }

//     // Sort based on sorted selector
//     if (a_class_index != null && b_class_index != null) {
//       if (a_class_index < b_class_index) {
//         return -1;
//       }
//       if (a_class_index > b_class_index) {
//         return 1;
//       }
//     }

//     return 0;
//   });

//   return (
//     " ".repeat(whitespace_before) +
//     classes.join(" ") +
//     " ".repeat(whitespace_after)
//   );
// }
