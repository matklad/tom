use std::{
    collections::HashMap,
};

use {TomlDoc, TomlNode, symbol::*};

#[derive(Debug)]
pub struct Edit<'f> {
    file: &'f TomlDoc,
    ops: HashMap<TomlNode<'f>, Op<'f>>,
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

#[derive(Debug, Clone)]
enum Op<'f> {
    Delete,
    Replace(TomlNode<'f>),
    Rewrite(String),
    ChangeContents(ChangeContents<'f>),
}

#[derive(Debug, Clone)]
struct ChangeContents<'f> {
    ops: Vec<ContentsChange<'f>>,
}

impl<'f> ChangeContents<'f> {
    fn merge(&self, other: &ChangeContents<'f>) -> ChangeContents<'f> {
        let mut ops = Vec::new();
        ops.extend(self.ops.clone());
        ops.extend(other.ops.clone());
        ChangeContents { ops }
    }
}

#[derive(Debug, Copy, Clone)]
struct ContentsChange<'f> {
    new_child: TomlNode<'f>,
    position: usize,
}

impl<'f> Edit<'f> {
    pub fn new(file: &'f TomlDoc) -> Edit {
        Edit {
            file,
            ops: HashMap::new(),
        }
    }

    pub fn replace(
        &mut self,
        node: impl Into<TomlNode<'f>>,
        replacement: impl Into<TomlNode<'f>>,
    ) {
        self.op(node.into(), Op::Replace(replacement.into()));
    }

    pub fn replace_with_text(
        &mut self,
        node: impl Into<TomlNode<'f>>,
        replacement: String,
    ) {
        self.op(node.into(), Op::Rewrite(replacement));
    }

    pub fn insert(
        &mut self,
        node: impl Into<TomlNode<'f>>,
        position: Position<'f>,
    ) {
        self.insert_many(vec![node], position)
    }

    pub fn insert_many(
        &mut self,
        children: Vec<impl Into<TomlNode<'f>>>,
        position: Position<'f>,
    ) {
        let (parent, position) = match position {
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
        let ops = children.into_iter().map(Into::into)
            .map(|new_child| ContentsChange { new_child, position })
            .collect();
        self.op(parent, Op::ChangeContents(ChangeContents { ops }));
    }

    pub fn append_children(
        &mut self,
        parent: impl Into<TomlNode<'f>>,
        children: Vec<impl Into<TomlNode<'f>>>,
    ) {
        let parent = parent.into();
        let position = parent.children().count();
        let change = ChangeContents {
            ops: children.into_iter().map(Into::into)
                .map(|node| ContentsChange {
                    new_child: node,
                    position,
                })
                .collect(),
        };
        self.op(parent, Op::ChangeContents(change));
    }

    pub fn delete(&mut self, node: impl Into<TomlNode<'f>>) {
        self.op(node.into(), Op::Delete);
    }

    pub fn finish(self) -> String {
        let root = self.file.parse_tree();
        let mut res = self.rendered(root);
        if !res.ends_with("\n") {
            res += "\n";
        }
        res
    }

    fn op(&mut self, target: TomlNode<'f>, op: Op<'f>) {
        let merged = if !self.ops.contains_key(&target) {
            op
        } else {
            let old = &self.ops[&target];
            match (old, &op) {
                (Op::ChangeContents(old), Op::ChangeContents(new)) => {
                    Op::ChangeContents(old.merge(new))
                }
                _ => op.clone(),
            }
        };
        self.ops.insert(target, merged);
    }
}

impl<'f> Edit<'f> {
    fn rendered(&self, node: TomlNode<'f>) -> String {
        match self.ops.get(&node) {
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
            Some(op) => {
                match op {
                    Op::Delete => String::new(),
                    Op::Rewrite(text) => text.to_owned(),
                    Op::Replace(replacement) => self.rendered(*replacement),
                    Op::ChangeContents(change) => {
                        let new = change.ops.iter()
                            .map(|ch| (ch.position, false, ch.new_child));
                        let old = node.children().enumerate()
                            .map(|(p, n)| (p, true, n));
                        let mut all: Vec<(usize, bool, TomlNode)> = new.chain(old).collect();
                        all.sort_by_key(|&(pos, old, _)| (pos, old));

                        let mut buff = String::new();
                        let mut prev_child = None;

                        for (_, is_old, child) in all {
                            match prev_child {
                                Some((prev_old, prev)) => {
                                    if !(prev_old && is_old) {
                                        buff += &compute_ws(prev, child)
                                    }
                                }
                                _ => (),
                            };
                            prev_child = Some((is_old, child));
                            buff += &self.rendered(child);
                        }
                        buff
                    }
                }
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
