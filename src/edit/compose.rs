use std::collections::hash_map::{HashMap, Entry};
use {
    CstNode,
    edit::node_change::{Changes, MergedChild},
    edit::whitespace::{compute_ws, Location, Edge},
};

pub(crate) fn compose<'f>(
    root: CstNode<'f>,
    ops: &HashMap<CstNode<'f>, Changes>,
    smart_ws: bool,
) -> String {
    let mut state = State::new(ops, smart_ws);
    state.go(root, 0);
    state.buff
}

enum HasChanges {
    No,
    Yes,
    InProgress,
}

struct State<'a, 'f: 'a> {
    ops: &'a HashMap<CstNode<'f>, Changes<'f>>,
    smart_ws: bool,
    buff: String,
    has_changes: HashMap<CstNode<'f>, HasChanges>,
}

impl<'a, 'f> State<'a, 'f> {
    fn new(ops: &'a HashMap<CstNode<'f>, Changes<'f>>, smart_ws: bool) -> Self {
        State {
            ops,
            smart_ws,
            buff: String::new(),
            has_changes: HashMap::new(),
        }
    }

    fn go(&mut self, node: CstNode<'f>, level: u32) {
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
        let mut prev: Option<(bool, CstNode)> = None;
        for m in changes.merge(node.children()) {
            match m {
                MergedChild::Old(child) => {
                    match prev {
                        Some((prev_old, prev)) => {
                            if !prev_old {
                                self.adjust_ws(Location::Between(prev, child));
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
                    &match prev {
                        Some((_, prev)) => self.adjust_ws(
                            Location::Between(prev, new_child),
                        ),
                        None => self.adjust_ws(
                            Location::OnEdge { child: new_child, parent: node, edge: Edge::Left }
                        ),
                    };
                    self.go(new_child, level + 1);
                    prev = Some((false, new_child));
                }
            }
        }
        match prev {
            Some((false, new_child)) => {
                self.adjust_ws(
                    Location::OnEdge { child: new_child, parent: node, edge: Edge::Right }
                );
            }
            _ => (),
        }
    }

    fn has_changes(&mut self, node: CstNode<'f>) -> bool {
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
            if result { HasChanges::Yes } else { HasChanges::No },
        );
        result
    }

    fn adjust_ws(&mut self, loc: Location) {
        if self.smart_ws {
            let ws = compute_ws(loc);
            self.buff.push_str(&ws);
        }
    }
}
