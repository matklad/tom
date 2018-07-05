extern crate testutils;
#[macro_use]
extern crate tom;
#[macro_use]
extern crate lazy_static;

mod ast;
mod cargo_toml;
mod edit;
mod factory;

use std::{panic, sync::Mutex};

use testutils::assert_eq_text;
use tom::{AstNode, CstNode, TomlDoc};

fn toml(text: &str) -> TomlDoc {
    TomlDoc::new(text)
}

fn find<A: AstNode>(doc: &TomlDoc) -> A {
    subtree(doc.cst(), doc)
        .into_iter()
        .filter_map(|node| A::cast(node, doc))
        .next()
        .unwrap()
}

fn subtree(node: CstNode, doc: &TomlDoc) -> Vec<CstNode> {
    let mut buff = Vec::new();
    go(node, doc, &mut buff);
    return buff;

    fn go(node: CstNode, doc: &TomlDoc, buff: &mut Vec<CstNode>) {
        buff.push(node);
        for child in node.children(doc) {
            go(child, doc, buff);
        }
    }
}

pub fn check_edit(before: &str, after: &str, edit: impl FnOnce(&mut TomlDoc)) {
    let mut doc = TomlDoc::new(before);
    doc.start_edit();
    edit(&mut doc);
    let actual = doc.cst().get_text(&doc);
    assert_eq_text(after, &actual);
}

lazy_static! {
    static ref LOCK: std::sync::Mutex<()> = Mutex::new(());
}

pub fn check_panics(f: impl FnOnce()) {
    let _guard = LOCK.lock().unwrap();
    let old_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| ()));
    let result = panic::catch_unwind(panic::AssertUnwindSafe(f));
    panic::set_hook(old_hook);
    assert!(result.is_err());
}

#[test]
fn test_parser_inline() {
    testutils::dir_tests(&["inline"], |text| toml(text).debug_dump())
}

#[test]
fn test_parser_ok() {
    testutils::dir_tests(&["ok"], |text| toml(text).debug_dump())
}

#[test]
fn test_parser_validation() {
    testutils::dir_tests(&["validation"], |text| toml(text).debug_dump())
}
