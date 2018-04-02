#[macro_use]
extern crate parse_tree;

use parse_tree::ParseTree;
use ast::AstNode;

mod parsing;
mod symbols;

pub mod ast;
pub mod edit;
pub mod ide;

pub struct TomlFile {
    tree: ParseTree,
}

impl TomlFile {
    pub fn new(text: String) -> TomlFile {
        let tree = parsing::parse(text);
        TomlFile { tree }
    }

    pub fn debug_dump(&self) -> String {
        parse_tree::debug_dump(
            self.tree.root(),
        )
    }

    pub fn ast(&self) -> ast::File {
        ast::File::cast(self.tree.root()).unwrap()
    }
}
