extern crate testutils;
extern crate tom;

use tom::{
    TomlFile, TomlNode,
    ast::AstNode,
};

mod ast;
mod edit;
mod factory;


fn toml(text: &str) -> TomlFile {
    TomlFile::new(text.to_owned())
}

fn find<'p, A: AstNode<'p>>(toml: &'p TomlFile) -> A {
    subtree(toml.parse_tree()).into_iter()
        .filter_map(A::cast)
        .next()
        .unwrap()
}

fn subtree<'p>(node: TomlNode<'p>) -> Vec<TomlNode<'p>> {
    let mut buff = Vec::new();
    go(node, &mut buff);
    return buff;

    fn go<'p>(node: TomlNode<'p>, buff: &mut Vec<TomlNode<'p>>) {
        buff.push(node);
        for child in node.children() {
            go(child, buff);
        }
    }
}

#[test]
fn test_parser_ok() {
    testutils::dir_tests(&["ok"], |text| toml(text).debug_dump())
}
