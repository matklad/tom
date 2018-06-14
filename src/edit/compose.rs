use std::collections::hash_map::{HashMap, Entry};
use {
    TomlNode,
    edit::node_change::{Changes, MergedChild},
    edit::whitespace::{compute_ws, Location, Edge},
};

pub(crate) fn compose<'f>(root: TomlNode<'f>, ops: &HashMap<TomlNode<'f>, Changes>) -> String {
    let mut state = State::new(ops);
    state.go(root, 0);
    state.buff
}

enum HasChanges {
    No,
    Yes,
    InProgress,
}

struct State<'a, 'f: 'a> {
    ops: &'a HashMap<TomlNode<'f>, Changes<'f>>,
    buff: String,
    has_changes: HashMap<TomlNode<'f>, HasChanges>,
}

impl<'a, 'f> State<'a, 'f> {
    fn new(ops: &'a HashMap<TomlNode<'f>, Changes<'f>>) -> Self {
        State {
            ops,
            buff: String::new(),
            has_changes: HashMap::new(),
        }
    }

    fn go(&mut self, node: TomlNode<'f>, level: u32) {
        if !self.has_changes(node) {
            self.buff.push_str(node.text());
            return;
        }
        if level > 999 {
            covered_by!("infinite_doc");
            panic!("Infinite edit");
        }

        let no_changes = Default::default();
        let changes = self.ops.get(&node)
            .unwrap_or_else(|| &no_changes);
        let mut prev: Option<(bool, TomlNode)> = None;
        for m in changes.merge(node.children()) {
            match m {
                MergedChild::Old(child) => {
                    match prev {
                        Some((prev_old, prev)) => {
                            if !prev_old {
                                self.buff.push_str(
                                    &compute_ws(Location::Between(prev, child))
                                )
                            }
                        }
                        _ => (),
                    };
                    self.go(child, level + 1);
                    prev = Some((true, child));
                }
                MergedChild::Deleted(_) => (),
                MergedChild::Replaced(new_child) => {
                    self.go(new_child, level + 1);
                    prev = Some((false, new_child));
                }
                MergedChild::Inserted(new_child) => {
                    let ws = &match prev {
                        Some((_, prev)) => compute_ws(
                            Location::Between(prev, new_child),
                        ),
                        None => compute_ws(
                            Location::OnEdge { child: new_child, parent: node, edge: Edge::Left }
                        ),
                    };
                    self.buff.push_str(&ws);
                    self.go(new_child, level + 1);
                    prev = Some((false, new_child));
                }
            }
        }
        match prev {
            Some((false, new_child)) => {
                let ws = &compute_ws(
                    Location::OnEdge { child: new_child, parent: node, edge: Edge::Right }
                );
                self.buff.push_str(&ws);
            }
            _ => (),
        }
    }

    fn has_changes(&mut self, node: TomlNode<'f>) -> bool {
        match self.has_changes.entry(node) {
            Entry::Vacant(entry) => {
                entry.insert(HasChanges::InProgress);
            }
            Entry::Occupied(entry) => return match entry.get() {
                HasChanges::No => false,
                HasChanges::Yes => true,
                HasChanges::InProgress => unreachable!("Cycle in the tree"),
            }
        }
        let result = self.ops.contains_key(&node) ||
            node.children().any(|child| self.has_changes(child));
        self.has_changes.insert(
            node,
            if result { HasChanges::Yes } else { HasChanges::No }
        );
        result
    }
}
