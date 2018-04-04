#[macro_use]
extern crate lazy_static;

use parse_tree::ParseTree;
use ast::AstNode;
use edit::TreeEdit;

#[macro_use]
pub mod parse_tree;

mod parsing;
mod symbols;

pub mod ast;
pub mod edit;
pub mod ide;

pub struct TomlFile {
    parse_tree: ParseTree,
}

impl TomlFile {
    pub fn new(text: String) -> TomlFile {
        let tree = parsing::parse(text);
        TomlFile { parse_tree: tree }
    }

    pub fn parse_tree(&self) -> &ParseTree {
        &self.parse_tree
    }

    pub fn ast(&self) -> ast::File {
        ast::File::cast(self.parse_tree().root()).unwrap()
    }

    pub fn edit(&self) -> TreeEdit {
        TreeEdit::new(self.parse_tree())
    }

    pub fn debug_dump(&self) -> String {
        parse_tree::debug_dump(
            self.parse_tree().root(),
        )
    }
}
