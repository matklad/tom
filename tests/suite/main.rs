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

fn find<'f, A: AstNode<'f>>(toml: &'f TomlFile) -> A {
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

#[test]
fn test_parser_ok() {
    testutils::dir_tests(&["ok"], |text| toml(text).debug_dump())
}
