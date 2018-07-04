extern crate m_lexer;
extern crate string_interner;
extern crate text_unit;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate typed_index_derive;

pub mod ast;
mod intern;
mod parser;
pub mod symbol;
mod tree;
mod visitor;
mod validator;

use std::{marker::PhantomData, num::NonZeroU8, cmp};

use {
    intern::{Intern, InternId},
    tree::{NodeId, TreeData},
};

pub use text_unit::{TextRange, TextUnit};

type ID = Symbol;
type LD = (Symbol, InternId);
type Tree = tree::Tree<ID, LD>;

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
    data: Vec<NodeData>,
    intern: Intern,
    errors: Vec<SyntaxError>,
}

impl TomlDoc {
    pub fn new(text: &str) -> TomlDoc {
        let pt = parser::parse(text);
        assemble(pt)
    }

    pub fn cst(&self) -> CstNode {
        CstNode(self.tree.root())
    }

    pub fn debug_dump(&self) -> String {
        let mut result = String::new();
        go(self.cst(), 0, &self, &mut result);

        let text = self.cst().get_text(self);
        if !self.errors.is_empty() {
            result += "\n";
            for e in self.errors.iter() {
                let text = &text[e.range];
                result += &format!("error@{:?} {:?}: {}\n", e.range(), text, e.message());
            }
        }
        return result;

        fn go(node: CstNode, level: usize, doc: &TomlDoc, buff: &mut String) {
            buff.push_str(&String::from("  ").repeat(level));
            let range = node.range(doc);
            let symbol = node.symbol(doc);
            buff.push_str(&format!("{}@{:?}", symbol.name(), range));
            match node.kind(doc) {
                NodeKind::Leaf(text) => {
                    if !text.chars().all(char::is_whitespace) {
                       buff.push_str(&format!(" {:?}", text));
                    }
                }
                NodeKind::Internal(_) => (),
            }
            buff.push('\n');
            for child in node.children(&doc) {
                go(child, level + 1, doc, buff)
            }
        }
    }

    pub fn get_text(&self, range: TextRange) -> String {
        let mut buff = String::new();
        process_leaves(
            self.cst(),
            self,
            &mut |node| intersect(node.range(self), range).is_some(),
            &mut |leaf, text| {
                let node_range = leaf.range(self);
                let range = intersect(node_range, range).unwrap();
                let range = relative_range(node_range.start(), range);
                buff.push_str(&text[range]);
            }
        );
        return buff;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CstNode(NodeId);

pub enum NodeKind<'a> {
    Leaf(&'a str),
    Internal(Children<'a>),
}

impl CstNode {
    pub fn symbol(self, doc: &TomlDoc) -> Symbol {
        *match self.0.data(&doc.tree) {
            TreeData::Interior(s) => s,
            TreeData::Leaf((s, _)) => s,
        }
    }

    pub fn range(self, doc: &TomlDoc) -> TextRange {
        doc.data[self.0.to_idx()].range
    }

    pub fn kind(self, doc: &TomlDoc) -> NodeKind {
        match self.0.data(&doc.tree) {
            TreeData::Leaf((_, idx)) => NodeKind::Leaf(doc.intern.resolve(*idx)),
            TreeData::Interior(_) => NodeKind::Internal(self.children(doc)),
        }
    }

    pub fn is_leaf(self, doc: &TomlDoc) -> bool {
        match self.kind(doc) {
            NodeKind::Leaf(_) => true,
            NodeKind::Internal(_) => false,
        }
    }

    pub fn children(self, doc: &TomlDoc) -> Children {
        Children(self.0.children(&doc.tree))
    }

    pub fn get_text(self, doc: &TomlDoc) -> String {
        let mut buff = String::new();
        process_leaves(self, doc, &mut |_| true, &mut |_, text| buff.push_str(text));
        return buff;
    }

    pub fn debug(self, doc: &TomlDoc) -> String {
        format!("{}@{:?}", self.symbol(doc).name(), self.range(doc))
    }
}

pub struct Children<'a>(tree::Children<'a, ID, LD>);

impl<'a> Children<'a> {
    pub fn first(self) -> Option<CstNode> {
        self.0.first().map(CstNode)
    }
    pub fn last(self) -> Option<CstNode> {
        self.0.last().map(CstNode)
    }
    pub fn iter(self) -> ChildrenIter<'a> {
        ChildrenIter(self.0.iter())
    }
    pub fn rev(self) -> RevChildrenIter<'a> {
        RevChildrenIter(self.0.rev())
    }
}

impl<'a> IntoIterator for Children<'a> {
    type Item = CstNode;
    type IntoIter = ChildrenIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct ChildrenIter<'a>(tree::ChildrenIter<'a, ID, LD>);
impl<'a> Iterator for ChildrenIter<'a> {
    type Item = CstNode;
    fn next(&mut self) -> Option<CstNode> {
        self.0.next().map(CstNode)
    }
}

pub struct RevChildrenIter<'a>(tree::RevChildrenIter<'a, ID, LD>);
impl<'a> Iterator for RevChildrenIter<'a> {
    type Item = CstNode;
    fn next(&mut self) -> Option<CstNode> {
        self.0.next().map(CstNode)
    }
}

pub trait AstNode: Into<CstNode> + Clone + Copy {
    fn cst(self) -> CstNode {
        self.into()
    }
    fn cast(cst: CstNode, doc: &TomlDoc) -> Option<Self>;
}

pub struct AstChildren<'a, A: AstNode> {
    inner: ChildrenIter<'a>,
    doc: &'a TomlDoc, // TODO: get rid of,
    phantom: PhantomData<A>,
}

impl<'a, A: AstNode> AstChildren<'a, A> {
    fn new(node: CstNode, doc: &'a TomlDoc) -> Self {
        AstChildren {
            inner: node.children(doc).iter(),
            doc,
            phantom: PhantomData,
        }
    }
}

impl<'a, A: AstNode> Iterator for AstChildren<'a, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.inner.next() {
            if let Some(a) = A::cast(node, self.doc) {
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

fn assemble(pt: parser::ParseTree) -> TomlDoc {
    let mut data = vec![
        NodeData {
            range: TextRange::offset_len(0.into(), 0.into())
        };
        pt.tree.len()
    ];
    go(pt.tree.root(), 0.into(), &mut data, &pt.tree, &pt.intern);
    let mut doc = TomlDoc {
        tree: pt.tree,
        data,
        intern: pt.intern,
        errors: pt.errors,
    };
    let validation_errors = validator::validate(&doc);
    eprintln!("{:?}", validation_errors);
    doc.errors.extend(validation_errors);
    return doc;

    fn go(
        node: NodeId,
        start_offset: TextUnit,
        data: &mut Vec<NodeData>,
        tree: &Tree,
        intern: &Intern,
    ) -> TextUnit {
        let mut len: TextUnit = 0.into();
        match node.data(tree) {
            TreeData::Leaf(&(_, idx)) => {
                len += (intern.resolve(idx).len() as u32).into();
            }
            TreeData::Interior(_) => {
                for child in node.children(tree) {
                    len += go(child, start_offset + len, data, tree, intern);
                }
            }
        }
        data[node.to_idx()].range = TextRange::offset_len(start_offset, len);
        len
    }
}

fn intersect(r1: TextRange, r2: TextRange) -> Option<TextRange> {
    let start = cmp::max(r1.start(), r2.start());
    let end = cmp::min(r1.end(), r2.end());
    if end > start {
        Some(TextRange::from_to(start, end))
    } else {
        None
    }
}

fn relative_range(offset: TextUnit, range: TextRange) -> TextRange {
    TextRange::from_to(range.start() - offset, range.end() - offset)
}

fn process_leaves(
    node: CstNode,
    doc: &TomlDoc,
    node_filter: &mut impl Fn(CstNode) -> bool,
    cb: &mut impl FnMut(CstNode, &str),
) {
    if !node_filter(node) {
        return
    }
    match node.kind(doc) {
        NodeKind::Leaf(text) => cb(node, text),
        NodeKind::Internal(children) => {
            for child in children {
                process_leaves(child, doc, node_filter, cb);
            }
        }
    }
}
