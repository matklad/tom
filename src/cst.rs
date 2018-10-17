use std::cmp;

use {
    TomlDoc, Symbol, TextUnit, TextRange, ChunkedText,
    tree::{NodeId, TreeData},
    walk::{preorder, walk_filter, WalkEvent},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CstNode(pub(crate) NodeId);

pub enum CstNodeKind<'a> {
    Leaf(&'a str),
    Internal(CstChildren<'a>),
}

impl CstNode {
    pub fn symbol(self, doc: &TomlDoc) -> Symbol {
        *match self.0.data(&doc.tree) {
            TreeData::Internal(s) => s,
            TreeData::Leaf((s, _)) => s,
        }
    }

    pub fn range(self, doc: &TomlDoc) -> TextRange {
        assert!(
            !doc.edit_in_progress,
            "range info is unavailable during edit"
        );
        doc.data[self.0.to_idx()].range
    }

    pub fn kind(self, doc: &TomlDoc) -> CstNodeKind {
        match self.0.data(&doc.tree) {
            TreeData::Leaf((_, idx)) => CstNodeKind::Leaf(doc.intern.resolve(*idx)),
            TreeData::Internal(_) => CstNodeKind::Internal(self.children(doc)),
        }
    }

    pub fn is_leaf(self, doc: &TomlDoc) -> bool {
        match self.kind(doc) {
            CstNodeKind::Leaf(_) => true,
            CstNodeKind::Internal(_) => false,
        }
    }

    pub fn parent(self, doc: &TomlDoc) -> Option<CstNode> {
        self.0.parent(&doc.tree).map(CstNode)
    }

    pub fn children(self, doc: &TomlDoc) -> CstChildren {
        CstChildren { doc, node: self }
    }

    pub fn next_sibling(self, doc: &TomlDoc) -> Option<CstNode> {
        self.0.next_sibling(&doc.tree).map(CstNode)
    }

    pub fn prev_sibling(self, doc: &TomlDoc) -> Option<CstNode> {
        self.0.prev_sibling(&doc.tree).map(CstNode)
    }

    pub fn get_text(self, doc: &TomlDoc) -> String {
        self.chunked_text(doc).into_string()
    }

    pub(crate) fn chunked_text<'a>(self, doc: &'a TomlDoc) -> impl ChunkedText + 'a {
        preorder(doc, self)
            .filter_map(move |node| match node.kind(doc) {
                CstNodeKind::Leaf(text) => Some(text),
                CstNodeKind::Internal(_) => None,
            })
    }

    pub(crate) fn chunked_substring<'a>(
        self,
        doc: &'a TomlDoc,
        range: TextRange,
    ) -> impl ChunkedText + 'a {
        walk_filter(doc, self, move |node| intersects(range, node.range(doc)))
            .filter_map(move |event| match event {
                WalkEvent::Enter(node) => Some(node),
                WalkEvent::Exit(_) => None,
            })
            .filter_map(move |node| match node.kind(doc) {
                CstNodeKind::Leaf(text) => {
                    let node_range = node.range(doc);
                    let subrange = intersect(range, node_range).unwrap();
                    let subrange = relative_range(node_range.start(), subrange);
                    Some(&text[subrange])
                },
                CstNodeKind::Internal(_) => None,
            })
    }
    pub fn debug(self, doc: &TomlDoc) -> String {
        if doc.edit_in_progress {
            format!("{}@[??:??)", self.symbol(doc).name())
        } else {
            format!("{}@{:?}", self.symbol(doc).name(), self.range(doc))
        }
    }
}

#[derive(Clone, Copy)]
pub struct CstChildren<'a> {
    doc: &'a TomlDoc,
    node: CstNode,
}

impl<'a> CstChildren<'a> {
    pub fn first(self) -> Option<CstNode> {
        self.node.0.first_child(&self.doc.tree).map(CstNode)
    }
    pub fn last(self) -> Option<CstNode> {
        self.node.0.last_child(&self.doc.tree).map(CstNode)
    }
    pub fn iter(self) -> CstChildrenIter<'a> {
        CstChildrenIter {
            doc: self.doc,
            curr: self.first(),
        }
    }
    pub fn rev(self) -> RevCstChildrenIter<'a> {
        RevCstChildrenIter {
            doc: self.doc,
            curr: self.last(),
        }
    }
}

impl<'a> IntoIterator for CstChildren<'a> {
    type Item = CstNode;
    type IntoIter = CstChildrenIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone)]
pub struct CstChildrenIter<'a> {
    pub(crate) doc: &'a TomlDoc,
    curr: Option<CstNode>,
}

impl<'a> Iterator for CstChildrenIter<'a> {
    type Item = CstNode;
    fn next(&mut self) -> Option<CstNode> {
        self.curr.map(|node| {
            self.curr = node.next_sibling(self.doc);
            node
        })
    }
}

#[derive(Clone)]
pub struct RevCstChildrenIter<'a> {
    doc: &'a TomlDoc,
    curr: Option<CstNode>,
}

impl<'a> Iterator for RevCstChildrenIter<'a> {
    type Item = CstNode;
    fn next(&mut self) -> Option<CstNode> {
        self.curr.map(|node| {
            self.curr = node.prev_sibling(self.doc);
            node
        })
    }
}

fn intersects(r1: TextRange, r2: TextRange) -> bool {
    intersect(r1, r2).is_some()
}


