use itertools::{Itertools, EitherOrBoth, merge_join_by};
use {TomlNode, Children};

#[derive(Debug, Clone, Default)]
pub(crate) struct Changes<'f> {
    child_changes: Vec<ChildChange<'f>>,
}

impl<'f> Changes<'f> {
    pub fn add_child_change(&mut self, pos: usize, op: ChildChangeOp<'f>) {
        self.child_changes.push(ChildChange { pos, op })
    }

    pub fn merge(&self, old_children: Children<'f>) -> Vec<MergedChild<'f>> {
        let mut changes = self.child_changes.clone();
        changes.sort_by_key(|change| change.pos);
        // I am eternally grateful to itertools for being able to write this
        let by_pos = changes.into_iter()
            .group_by(|ch| (ch.pos, match ch.op {
                ChildChangeOp::Insert(..) => 0,
                _ => 1
            }));
        let merged = merge_join_by(
            &by_pos,
            old_children.enumerate(),
            |((group_pos, _), _), (child_pos, _)| group_pos.cmp(child_pos),
        );

        let mut res = Vec::new();
        'pos: for op_child in merged {
            match op_child {
                EitherOrBoth::Both((_, group), (_, old_child)) => {
                    for change in group {
                        match change.op {
                            ChildChangeOp::Insert(new_child) =>
                                res.push(MergedChild::Inserted(new_child)),
                            // Insert are sorted first above
                            ChildChangeOp::Delete => {
                                res.push(MergedChild::Deleted(old_child));
                                continue 'pos;
                            }
                            ChildChangeOp::Replace(new_child) => {
                                res.push(MergedChild::Replaced(new_child));
                                continue 'pos;
                            }
                        }
                    }
                }
                EitherOrBoth::Left((_, group)) => {
                    for change in group {
                        match change.op {
                            ChildChangeOp::Insert(new_child) =>
                                res.push(MergedChild::Inserted(new_child)),
                            _ => unreachable!("Modification operation after last child")
                        }
                    }
                }
                EitherOrBoth::Right((_, old_child)) =>
                    res.push(MergedChild::Old(old_child)),
            }
        }
        res
    }
}

pub(crate) enum MergedChild<'f> {
    Old(TomlNode<'f>),
    Deleted(TomlNode<'f>),
    Replaced(TomlNode<'f>),
    Inserted(TomlNode<'f>),
}

#[derive(Clone, Copy, Debug)]
struct ChildChange<'f> {
    pos: usize,
    op: ChildChangeOp<'f>,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum ChildChangeOp<'f> {
    Delete,
    Replace(TomlNode<'f>),
    Insert(TomlNode<'f>),
}
