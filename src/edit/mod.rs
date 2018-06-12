use std::{
    collections::HashMap,
};

use {TomlDoc, TomlNode, symbol::*};

mod node_change;
use self::node_change::{
    Changes, MergedChild, ChildChangeOp
};

#[derive(Debug)]
pub struct Edit<'f> {
    doc: &'f TomlDoc,
    ops: HashMap<TomlNode<'f>, Changes<'f>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Position<'f> {
    After(TomlNode<'f>),
    Before(TomlNode<'f>),
    StartOf(TomlNode<'f>),
    EndOf(TomlNode<'f>),
}

impl<'f> Position<'f> {
    pub fn after(node: impl Into<TomlNode<'f>>) -> Position<'f> {
        Position::After(node.into())
    }
    pub fn before(node: impl Into<TomlNode<'f>>) -> Position<'f> {
        Position::Before(node.into())
    }
    pub fn start_of(node: impl Into<TomlNode<'f>>) -> Position<'f> {
        Position::StartOf(node.into())
    }
    pub fn end_of(node: impl Into<TomlNode<'f>>) -> Position<'f> {
        Position::EndOf(node.into())
    }
}

impl<'f> Edit<'f> {
    pub fn new(doc: &'f TomlDoc) -> Edit {
        Edit { doc, ops: HashMap::new() }
    }

    pub fn replace(
        &mut self,
        node: impl Into<TomlNode<'f>>,
        replacement: impl Into<TomlNode<'f>>,
    ) {
        let (parent, pos) = parent(node.into());
        self.changes_mut(parent).add_child_change(
            pos, ChildChangeOp::Replace(replacement.into())
        );
    }

    pub fn insert(
        &mut self,
        node: impl Into<TomlNode<'f>>,
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

    pub fn insert_many(
        &mut self,
        children: Vec<impl Into<TomlNode<'f>>>,
        position: Position<'f>,
    ) {
        for child in children {
            self.insert(child, position);
        }
    }

    pub fn delete(&mut self, node: impl Into<TomlNode<'f>>) {
        let (parent, pos) = parent(node.into());
        self.changes_mut(parent)
            .add_child_change(pos, ChildChangeOp::Delete)
    }

    pub fn finish(self) -> String {
        let root = self.doc.parse_tree();
        let mut res = self.rendered(root);
        if !res.ends_with("\n") {
            res += "\n";
        }
        res
    }

    fn changes(&self, target: TomlNode<'f>) -> Option<&Changes<'f>> {
        self.ops.get(&target)
    }

    fn changes_mut(&mut self, target: TomlNode<'f>) -> &mut Changes<'f> {
        self.ops.entry(target).or_insert_with(Default::default)
    }
}

impl<'f> Edit<'f> {
    fn rendered(&self, node: TomlNode<'f>) -> String {
        match self.changes(node) {
            None => {
                if node.is_leaf() {
                    node.text().to_owned()
                } else {
                    let mut buff = String::new();
                    for child in node.children() {
                        buff += &self.rendered(child);
                    }
                    buff
                }
            }
            Some(changes) => {
                let mut buff = String::new();
                let mut prev: Option<(bool, TomlNode)> = None;
                for m in changes.merge(node.children()) {
                    match m {
                        MergedChild::Old(child) => {
                            match prev {
                                Some((prev_old, prev)) => {
                                    if !prev_old {
                                        buff += &compute_ws(prev, child)
                                    }
                                }
                                _ => (),
                            };
                            buff += &self.rendered(child);
                            prev = Some((true, child));
                        },
                        MergedChild::Deleted(_) => (),
                        MergedChild::Replaced(new_child) => {
                            buff += &self.rendered(new_child);
                            prev = Some((false, new_child));
                        },
                        MergedChild::Inserted(new_child) => {
                            if let Some((_, prev)) = prev {
                                buff += &compute_ws(prev, new_child);
                            }
                            buff += &self.rendered(new_child);
                            prev = Some((false, new_child));
                        },
                    }
                }
                buff
            }
        }
    }
}

fn compute_ws(left: TomlNode, right: TomlNode) -> String {
    match (left.symbol(), right.symbol()) {
        (KEY_VAL, KEY_VAL) | (TABLE_HEADER, KEY_VAL) => String::from("\n"),
        (TABLE, TABLE) | (KEY_VAL, TABLE) => String::from("\n\n"),
        _ => String::new(),
    }
}

fn parent<'f>(child: TomlNode<'f>) -> (TomlNode<'f>, usize) {
    let parent = child.parent().unwrap();
    let position = parent.children().position(|it| it == child).unwrap();
    (parent, position)
}
