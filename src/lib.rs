extern crate m_lexer;
extern crate text_unit;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate uncover;

define_uncover_macros!(enable_if(cfg!(debug_assertions)));

mod chunked_text;
mod intern;
mod tree;
mod parser;
mod cst;
mod model;
mod visitor;
mod validator;
mod edit;

pub mod ast;
pub mod symbol;

use std::{
    marker::PhantomData,
    num::NonZeroU8
};

use intern::{Intern, InternId};

pub use edit::{IntoValue, Position};
pub use text_unit::{TextRange, TextUnit};
pub use cst::{CstNode, CstNodeKind, CstChildren, CstChildrenIter, RevCstChildrenIter};
pub use model::{Item};
pub(crate) use chunked_text::ChunkedText;

type Tree = tree::Tree<Symbol, (Symbol, InternId)>;

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
    tree: Tree,
    intern: Intern,
    errors: Vec<SyntaxError>,
    data: Vec<NodeData>,
    edit_in_progress: bool,
    smart_ws: bool,
}

impl TomlDoc {
    pub fn new(text: &str) -> TomlDoc {
        let mut doc = TomlDoc {
            tree: Tree::new(symbol::DOC),
            intern: Intern::new(),
            errors: Vec::new(),
            data: Vec::new(),
            edit_in_progress: false,
            smart_ws: true,
        };
        let root = doc.tree.root();
        parser::parse(text, &mut doc, root);
        let mut data = Vec::new();
        doc.recalculate_ranges(&mut data);
        doc.data = data;

        let validation_errors = validator::validate(&doc);
        doc.errors.extend(validation_errors);

        doc
    }

    pub fn cst(&self) -> CstNode {
        CstNode(self.tree.root())
    }

    pub fn ast(&self) -> ast::Doc {
        ast::Doc::cast(self.cst(), self).unwrap()
    }

    pub fn model(&self) -> Item {
        model::from_doc(self)
    }

    pub fn errors(&self) -> Vec<SyntaxError> {
        self.errors.clone()
    }

    pub fn debug(&self) -> String {
        let mut store;
        let data = if self.edit_in_progress {
            store = Vec::new();
            self.recalculate_ranges(&mut store);
            &store
        } else {
            &self.data
        };

        let mut buff = String::new();
        if self.edit_in_progress {
            buff += "*modified*\n";
        }
        go(self.cst(), 0, &self, &data, &mut buff);

        let text = self.cst().get_text(self);
        if !self.errors.is_empty() && !self.edit_in_progress {
            buff += "\n";
            for e in self.errors.iter() {
                let text = &text[e.range];
                buff += &format!("error@{:?} {:?}: {}\n", e.range(), text, e.message());
            }
        }
        return buff;

        fn go(node: CstNode, level: usize, doc: &TomlDoc, data: &[NodeData], buff: &mut String) {
            buff.push_str(&String::from("  ").repeat(level));
            //TODO: panics during edit :(
            let range = data[node.0.to_idx()].range;
            let symbol = node.symbol(doc);
            buff.push_str(&format!("{}@{:?}", symbol.name(), range));
            match node.kind(doc) {
                CstNodeKind::Leaf(text) => {
                    if !text.chars().all(char::is_whitespace) {
                        buff.push_str(&format!(" {:?}", text));
                    }
                }
                CstNodeKind::Internal(_) => (),
            }
            buff.push('\n');
            for child in node.children(&doc) {
                go(child, level + 1, doc, data, buff)
            }
        }
    }

    fn recalculate_ranges(&self, data: &mut Vec<NodeData>) {
        let node_data = NodeData {
            range: TextRange::offset_len(0.into(), 0.into()),
        };
        data.resize(self.tree.len(), node_data);
        go(self, self.cst(), 0.into(), data);

        fn go(
            doc: &TomlDoc,
            node: CstNode,
            start_offset: TextUnit,
            data: &mut Vec<NodeData>,
        ) -> TextUnit {
            let mut len: TextUnit = 0.into();
            match node.kind(doc) {
                CstNodeKind::Leaf(text) => {
                    len += TextUnit::of_str(text);
                }
                CstNodeKind::Internal(children) => {
                    for child in children {
                        len += go(doc, child, start_offset + len, data);
                    }
                }
            }
            data[node.0.to_idx()].range = TextRange::offset_len(start_offset, len);
            len
        }
    }
}

pub trait AstNode: Into<CstNode> + Clone + Copy {
    fn cst(self) -> CstNode {
        self.into()
    }
    fn cast(cst: CstNode, doc: &TomlDoc) -> Option<Self>;
}

pub struct AstChildren<'a, A: AstNode> {
    inner: CstChildrenIter<'a>,
    phantom: PhantomData<A>,
}

impl<'a, A: AstNode> AstChildren<'a, A> {
    fn new(node: CstNode, doc: &'a TomlDoc) -> Self {
        AstChildren {
            inner: node.children(doc).iter(),
            phantom: PhantomData,
        }
    }
}

impl<'a, A: AstNode> Iterator for AstChildren<'a, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.inner.next() {
            if let Some(a) = A::cast(node, self.inner.doc) {
                return Some(a);
            }
        }
        return None;
    }
}

#[derive(Clone, Copy)]
struct NodeData {
    range: TextRange,
}
