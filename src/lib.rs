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

// mod chunked_text;
// mod intern;
// mod tree;
mod rtree;
mod parser;
// mod cst;
// mod model;
// mod visitor;
// mod walk;
// mod validator;
// mod edit;

pub mod ast;
pub mod symbol;

use std::{
    num::NonZeroU8
};

// use intern::{Intern, InternId};
// use walk::{walk, WalkEvent};

// pub use edit::{IntoValue, Position};
pub use rowan::{SmolStr, TextRange, TextUnit};
// pub use cst::{CstNode, CstNodeKind, CstChildren, CstChildrenIter, RevCstChildrenIter};
// pub use model::{Item, Map};
pub use rtree::{SyntaxNode, SyntaxNodeRef};
pub(crate) use rtree::{GreenBuilder};
// pub(crate) use chunked_text::ChunkedText;


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
}

impl TomlDoc {
    pub fn new(text: &str) -> TomlDoc {
        let root = parser::parse(text);
        let doc = TomlDoc { root };

        // let validation_errors = validator::validate(&doc);
        // doc.errors.extend(validation_errors);

        doc
    }

    pub fn cst(&self) -> SyntaxNodeRef {
        self.root.borrowed()
    }

    pub fn ast(&self) -> ast::Doc {
        ast::Doc::cast(self.cst()).unwrap()
    }

    // pub fn model(&self) -> Map {
    //     model::from_doc(self)
    // }

    // pub fn errors(&self) -> Vec<SyntaxError> {
    //     self.errors.clone()
    // }

    // pub fn debug(&self) -> String {
    //     let mut store;
    //     let data = if self.edit_in_progress {
    //         store = Vec::new();
    //         self.recalculate_ranges(&mut store);
    //         &store
    //     } else {
    //         &self.data
    //     };

    //     let mut buff = String::new();
    //     if self.edit_in_progress {
    //         buff += "*modified*\n";
    //     }
    //     let mut level = 0;
    //     for event in walk(self, self.cst()) {
    //         match event {
    //             WalkEvent::Enter(node) => {
    //                 buff.push_str(&String::from("  ").repeat(level));
    //                 let range = data[node.0.to_idx()].range;
    //                 let symbol = node.symbol(self);
    //                 buff.push_str(&format!("{}@{:?}", symbol.name(), range));
    //                 match node.kind(self) {
    //                     CstNodeKind::Leaf(text) => {
    //                         if !text.chars().all(char::is_whitespace) {
    //                             buff.push_str(&format!(" {:?}", text));
    //                         }
    //                     }
    //                     CstNodeKind::Internal(_) => (),
    //                 }
    //                 buff.push('\n');
    //                 level += 1;
    //             },
    //             WalkEvent::Exit(_) => {
    //                 level -= 1;
    //             },
    //         }
    //     }

    //     if !self.errors.is_empty() && !self.edit_in_progress {
    //         let text = self.cst().get_text(self);
    //         buff += "\n";
    //         for e in self.errors.iter() {
    //             let text = &text[e.range];
    //             buff += &format!("error@{:?} {:?}: {}\n", e.range(), text, e.message());
    //         }
    //     }
    //     return buff;

    // }

    // fn recalculate_ranges(&self, data: &mut Vec<NodeData>) {
    //     let node_data = NodeData {
    //         range: TextRange::offset_len(0.into(), 0.into()),
    //     };
    //     data.resize(self.tree.len(), node_data);
    //     let mut edge: TextUnit = 0.into();
    //     for event in walk(self, self.cst()) {
    //         match event {
    //             WalkEvent::Enter(node) => {
    //                 data[node].range = TextRange::offset_len(
    //                     edge,
    //                     0.into(),
    //                 );
    //                 match node.kind(self) {
    //                     CstNodeKind::Leaf(text) => edge += TextUnit::of_str(text),
    //                     CstNodeKind::Internal(_) => (),
    //                 }
    //             },
    //             WalkEvent::Exit(node) => {
    //                 let start = data[node].range.start();
    //                 data[node].range = TextRange::from_to(
    //                     start,
    //                     edge,
    //                 )
    //             },
    //         }
    //     }
    // }
}

pub trait AstNode<'a>: Clone + Copy + 'a {
    fn cast(syntax: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized;
    fn syntax(self) -> SyntaxNodeRef<'a>;
}

// pub struct AstChildren<'a, A: AstNode> {
//     inner: CstChildrenIter<'a>,
//     phantom: PhantomData<A>,
// }

// impl<'a, A: AstNode> AstChildren<'a, A> {
//     fn new(node: CstNode, doc: &'a TomlDoc) -> Self {
//         AstChildren {
//             inner: node.children(doc).iter(),
//             phantom: PhantomData,
//         }
//     }
// }

// impl<'a, A: AstNode> Iterator for AstChildren<'a, A> {
//     type Item = A;

//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some(node) = self.inner.next() {
//             if let Some(a) = A::cast(node, self.inner.doc) {
//                 return Some(a);
//             }
//         }
//         return None;
//     }
// }
