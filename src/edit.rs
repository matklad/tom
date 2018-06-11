use std::{
    collections::HashMap,
};

use {TomlFile, TomlNode, symbol::*};

#[derive(Debug)]
pub struct Edit<'f> {
    file: &'f TomlFile,
    ops: HashMap<TomlNode<'f>, Op<'f>>,
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
    pub fn new(file: &'f TomlFile) -> Edit {
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

    pub fn append_child(
        &mut self,
        parent: impl Into<TomlNode<'f>>,
        child: impl Into<TomlNode<'f>>,
    ) {
        self.append_children(parent, vec![child]);
    }

    pub fn insert_sibling(
        &mut self,
        left_sibling: impl Into<TomlNode<'f>>,
        new_node: impl Into<TomlNode<'f>>,
    ) {
        let node = left_sibling.into();
        let parent = node.parent().unwrap();
        self.op(parent, Op::ChangeContents(ChangeContents {
            ops: vec![ContentsChange {
                new_child: new_node.into(),
                position: parent.children().position(|child| child == node).unwrap() + 1
            }]
        }))
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
