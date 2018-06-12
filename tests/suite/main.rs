extern crate testutils;
extern crate tom;

use tom::{
    TomlDoc, TomlNode,
    ast::AstNode,
};
use testutils::{
    assert_eq_text
};

mod ast;
mod edit;
mod factory;


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


#[test]
fn test_parser_ok() {
    testutils::dir_tests(&["ok"], |text| toml(text).debug_dump())
}
