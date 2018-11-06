extern crate m_lexer;
extern crate itertools;
extern crate text_unit;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate uncover;
extern crate drop_bomb;
extern crate rowan;

define_uncover_macros!(enable_if(cfg!(debug_assertions)));

mod chunked_text;
mod rtree;
mod parser;
// mod model;
// mod visitor;
mod validator;
// mod edit;

pub mod ast;
pub mod symbol;

use std::{
    num::NonZeroU8,
    marker::PhantomData,
};


// pub use edit::{IntoValue, Position};
pub use rowan::{SmolStr, TextRange, TextUnit, WalkEvent};
// pub use model::{Item, Map};
pub use rtree::{SyntaxNode, SyntaxNodeRef, RefRoot, OwnedRoot, SyntaxNodeChildren, TreeRoot, TomTypes};
pub(crate) use rtree::{GreenBuilder};
pub(crate) use chunked_text::ChunkedText;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(NonZeroU8);

impl Symbol {
    pub fn name(self) -> &'static str {
        self.info().0
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxError {
    range: TextRange,
    message: String,
}

impl SyntaxError {
    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

pub struct TomlDoc {
    root: rtree::SyntaxNode,
    validation_errors: Vec<SyntaxError>,
}

impl TomlDoc {
    pub fn new(text: &str) -> TomlDoc {
        let root = parser::parse(text);
        let mut doc = TomlDoc { root, validation_errors: Vec::new() };

        let validation_errors = validator::validate(&doc);
        doc.validation_errors = validation_errors;

        doc
    }

    pub fn cst(&self) -> SyntaxNodeRef {
        self.root.borrowed()
    }

    pub fn ast(&self) -> ast::Doc {
        ast::Doc::cast(self.cst()).unwrap()
    }
    // pub(crate) fn replace_with(&self, replacement: GreenNode) -> GreenNode {
    //     self.0.replace_with(replacement)
    // }

    // pub fn model(&self) -> Map {
    //     model::from_doc(self)
    // }

    pub fn errors(&self) -> Vec<SyntaxError> {
        self.root.root_data().iter()
            .chain(self.validation_errors.iter())
            .cloned()
            .collect()
    }

    pub fn debug(&self) -> String {
        let mut buff = String::new();
        let mut level = 0;
        for event in self.cst().preorder() {
            match event {
                WalkEvent::Enter(node) => {
                    buff.push_str(&String::from("  ").repeat(level));
                    let range = node.range();
                    let symbol = node.symbol();
                    buff.push_str(&format!("{}@{:?}", symbol.name(), range));
                    if let Some(text) = node.leaf_text() {
                        if !text.chars().all(char::is_whitespace) {
                            buff.push_str(&format!(" {:?}", text));
                        }
                    }
                    buff.push('\n');
                    level += 1;
                },
                WalkEvent::Leave(_) => {
                    level -= 1;
                },
            }
        }

        let errors = self.errors();
        if !errors.is_empty() {
            let text = self.cst().get_text();
            buff += "\n";
            for e in errors {
                let text = &text[e.range];
                buff += &format!("error@{:?} {:?}: {}\n", e.range(), text, e.message());
            }
        }
        return buff;

    }

}

pub trait AstNode<'a>: Clone + Copy + 'a {
    fn cast(syntax: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized;
    fn syntax(self) -> SyntaxNodeRef<'a>;
}

pub struct AstChildren<'a, A> {
    inner: SyntaxNodeChildren<RefRoot<'a>>,
    phantom: PhantomData<A>,
}

impl<'a, A: AstNode<'a>> AstChildren<'a, A> {
    fn new(node: SyntaxNodeRef<'a>) -> Self {
        AstChildren {
            inner: node.children(),
            phantom: PhantomData,
        }
    }
}

impl<'a, A: AstNode<'a>> Iterator for AstChildren<'a, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.inner.next() {
            if let Some(a) = A::cast(node) {
                return Some(a);
            }
        }
        return None;
    }
}
