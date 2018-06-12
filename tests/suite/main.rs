extern crate testutils;
#[macro_use]
extern crate tom;

use std::panic;

use tom::{
    TomlDoc, TomlNode,
    ast::AstNode,
};
use testutils::{
    assert_eq_text
};

mod ast;
mod factory;
mod edit;
mod cargo_toml;


fn toml(text: &str) -> TomlDoc {
    TomlDoc::new(text.to_owned())
}

fn find<'f, A: AstNode<'f>>(toml: &'f TomlDoc) -> A {
    subtree(toml.parse_tree()).into_iter()
        .filter_map(A::cast)
        .next()
        .unwrap()
}

fn subtree<'f>(node: TomlNode<'f>) -> Vec<TomlNode<'f>> {
    let mut buff = Vec::new();
    go(node, &mut buff);
    return buff;

    fn go<'f>(node: TomlNode<'f>, buff: &mut Vec<TomlNode<'f>>) {
        buff.push(node);
        for child in node.children() {
            go(child, buff);
        }
    }
}

pub fn check_edit(before: &str, after: &str, edit: impl FnOnce(&TomlDoc) -> String) {
    let doc = TomlDoc::new(before.to_string());
    let actual = edit(&doc);
    assert_eq_text(after, &actual);
}


pub fn check_panics(f: impl FnOnce()) {
    let old_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| ()));
    let result = panic::catch_unwind(panic::AssertUnwindSafe(f));
    panic::set_hook(old_hook);
    assert!(result.is_err());
}

#[test]
fn test_parser_ok() {
    testutils::dir_tests(&["ok"], |text| toml(text).debug_dump())
}
