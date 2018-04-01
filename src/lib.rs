#[macro_use]
extern crate parse_tree;

use parse_tree::ParseTree;

mod parsing;
mod symbols;
pub mod ast;
use ast::AstNode;

pub struct TomlFile {
    tree: ParseTree,
    text: String,
}

impl TomlFile {
    pub fn new(text: String) -> TomlFile {
        let tree = parsing::parse(&text);
        TomlFile { tree, text }
    }

    pub fn debug_dump(&self) -> String {
        parse_tree::debug_dump(
            self.tree.root(),
            &self.text
        )
    }

    pub fn ast(&self) -> ast::File {
        ast::File::cast(self.tree.root()).unwrap()
    }
}
