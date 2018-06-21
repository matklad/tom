use std::{
    collections::HashMap,
};

use {TomlDoc, CstNode};

mod factory;
mod node_change;
mod whitespace;
mod compose;

use self::node_change::{
    Changes, ChildChangeOp,
};
pub use self::factory::Factory;

#[derive(Debug)]
pub struct Edit<'f> {
    doc: &'f TomlDoc,
    ops: HashMap<CstNode<'f>, Changes<'f>>,
    smart_ws: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Position<'f> {
    After(CstNode<'f>),
    Before(CstNode<'f>),
    StartOf(CstNode<'f>),
    EndOf(CstNode<'f>),
}

impl<'f> Position<'f> {
    pub fn after(node: impl Into<CstNode<'f>>) -> Position<'f> {
        Position::After(node.into())
    }
    pub fn before(node: impl Into<CstNode<'f>>) -> Position<'f> {
        Position::Before(node.into())
    }
    pub fn start_of(node: impl Into<CstNode<'f>>) -> Position<'f> {
        Position::StartOf(node.into())
    }
    pub fn end_of(node: impl Into<CstNode<'f>>) -> Position<'f> {
        Position::EndOf(node.into())
    }
}

impl<'f> Edit<'f> {
    pub fn new(doc: &'f TomlDoc) -> Edit {
        Edit { doc, ops: HashMap::new(), smart_ws: true }
    }

    pub fn disable_smart_whitespace(&mut self) {
        self.smart_ws = false;
    }

    pub fn replace(
        &mut self,
        node: impl Into<CstNode<'f>>,
        replacement: impl Into<CstNode<'f>>,
    ) {
        let (parent, pos) = parent(node.into());
        self.changes_mut(parent).add_child_change(
            pos, ChildChangeOp::Replace(replacement.into()),
        );
    }

    pub fn insert(
        &mut self,
        node: impl Into<CstNode<'f>>,
        position: Position<'f>,
    ) {
        let (parent, pos) = match position {
            Position::After(a) => {
                let (parent, position) = parent(a);
                (parent, position + 1)
            }
            Position::Before(a) => {
                let (parent, position) = parent(a);
                (parent, position)
            }
            Position::StartOf(a) => (a, 0),
            Position::EndOf(a) => (a, a.children().count()),
        };
        self.changes_mut(parent)
            .add_child_change(pos, ChildChangeOp::Insert(node.into()));
    }

    pub fn delete(&mut self, node: impl Into<CstNode<'f>>) {
        let (parent, pos) = parent(node.into());
        self.changes_mut(parent)
            .add_child_change(pos, ChildChangeOp::Delete)
    }

    pub fn finish(self) -> String {
        let root = self.doc.cst();
        compose::compose(root, &self.ops, self.smart_ws)
    }

    fn changes_mut(&mut self, target: CstNode<'f>) -> &mut Changes<'f> {
        self.ops.entry(target).or_insert_with(Default::default)
    }
}

fn parent<'f>(child: CstNode<'f>) -> (CstNode<'f>, usize) {
    let parent = child.parent().unwrap();
    let position = parent.children().position(|it| it == child).unwrap();
    (parent, position)
}
