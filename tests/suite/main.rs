// extern crate difference;
// #[macro_use]
// extern crate tom;
// #[macro_use]
// extern crate lazy_static;
// extern crate serde_json;

// mod ast;
// // mod dir;
// mod edit;
// mod factory;
// // mod model;
// mod util;

// use std::{
//     panic,
//     fs,
//     sync::Mutex,
//     time::Instant,
// };
// use util::{assert_eq_text, test_data_dir};
// use tom::{AstNode, CstNode, TomlDoc};

// #[test]
// fn simple_bench() {
//     let path = test_data_dir().join("ok/complex_config.toml");
//     let text = fs::read_to_string(path).unwrap();
//     let start = Instant::now();
//     let _doc = toml(&text);
//     let time = start.elapsed().subsec_micros();
//     println!("{} μs", time);
// }

// fn toml(text: &str) -> TomlDoc {
//     let doc = TomlDoc::new(text);
//     assert!(doc.errors().is_empty(),
//         "No errors expected, but found in:\n{}\nerrors:\n{:?}", text, doc.errors());
//     doc
// }

// fn find<A: AstNode>(doc: &TomlDoc) -> A {
//     subtree(doc.cst(), doc)
//         .into_iter()
//         .filter_map(|node| A::cast(node, doc))
//         .next()
//         .unwrap()
// }

// fn subtree(node: CstNode, doc: &TomlDoc) -> Vec<CstNode> {
//     let mut buff = Vec::new();
//     go(node, doc, &mut buff);
//     return buff;

//     fn go(node: CstNode, doc: &TomlDoc, buff: &mut Vec<CstNode>) {
//         buff.push(node);
//         for child in node.children(doc) {
//             go(child, doc, buff);
//         }
//     }
// }

// pub fn check_edit(before: &str, after: &str, edit: impl FnOnce(&mut TomlDoc)) {
//     let mut doc = TomlDoc::new(before);
//     doc.start_edit();
//     edit(&mut doc);
//     let actual = doc.cst().get_text(&doc);
//     assert_eq_text(after, &actual);
// }

// lazy_static! {
//     static ref LOCK: std::sync::Mutex<()> = Mutex::new(());
// }

// pub fn check_panics(f: impl FnOnce()) {
//     let _guard = LOCK.lock().unwrap();
//     let old_hook = panic::take_hook();
//     panic::set_hook(Box::new(|_| ()));
//     let result = panic::catch_unwind(panic::AssertUnwindSafe(f));
//     panic::set_hook(old_hook);
//     assert!(result.is_err());
// }
