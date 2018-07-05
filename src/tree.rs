use std::num::NonZeroU32;

#[derive(Debug)]
pub(crate) struct Tree<ID, LD> {
    nodes: Vec<Node<ID, LD>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct NodeId(NonZeroU32);

impl NodeId {
    pub(crate) fn to_idx(self) -> usize {
        (self.0.get() - 1) as usize
    }
    pub(crate) fn from_idx(idx: usize) -> Self {
        NodeId(NonZeroU32::new((idx + 1) as u32).unwrap())
    }
}

#[derive(Debug)]
struct Node<ID, LD> {
    parent: Option<NodeId>,
    next_sibling: Option<NodeId>,
    // Kudos to http://www.aosabook.org/en/posa/parsing-xml-at-the-speed-of-light.html
    prev_sibling_cyclic: NodeId,
    kind: NodeKind<ID, LD>,
}

impl<ID, LD> Node<ID, LD> {
    fn new(kind: NodeKind<ID, LD>) -> Node<ID, LD> {
        Node {
            parent: None,
            next_sibling: None,
            prev_sibling_cyclic: NodeId::from_idx(0),
            kind,
        }
    }
}

pub(crate) enum TreeData<ID, LD> {
    Internal(ID),
    Leaf(LD),
}

#[derive(Debug)]
enum NodeKind<ID, LD> {
    Interior {
        first_child: Option<NodeId>,
        data: ID,
    },
    Leaf {
        data: LD,
    },
}

pub(crate) enum InsertPos {
    First,
    Last,
    After(NodeId),
    Before(NodeId),
}

impl<ID, LD> Tree<ID, LD> {
    pub fn root(&self) -> NodeId {
        NodeId::from_idx(0)
    }

    pub fn new(root_data: ID) -> Tree<ID, LD> {
        let root = Node::new(NodeKind::Interior {
            first_child: None,
            data: root_data,
        });
        let mut tree = Tree {
            nodes: Vec::with_capacity(1),
        };
        tree.new_node(root);
        tree
    }

    pub fn new_leaf(&mut self, data: LD) -> NodeId {
        self.new_node(Node::new(NodeKind::Leaf { data }))
    }

    pub fn new_internal(&mut self, data: ID) -> NodeId {
        self.new_node(Node::new(NodeKind::Interior {
            first_child: None,
            data,
        }))
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn insert_child(&mut self, parent: NodeId, new_child: NodeId, insert_pos: InsertPos) {
        assert!(new_child.parent(self).is_none());
        //TODO: deal with potential cycles
        self.get_mut(new_child).parent = Some(parent);
        self.do_insert(parent, new_child, insert_pos);
        parent.check_invariants(self);
        new_child.check_invariants(self);
    }

    fn do_insert(&mut self, parent: NodeId, new_child: NodeId, insert_pos: InsertPos) {
        let (prev, next_cyclic) = match insert_pos {
            InsertPos::First => {
                covered_by!("insert_first");
                return self.do_insert_first(parent, new_child);
            }
            InsertPos::Last => match (parent.first_child(self), parent.last_child(self)) {
                (Some(first), Some(last)) => {
                    covered_by!("insert_last_with_children");
                    (last, first)
                }
                _ => {
                    covered_by!("insert_last_no_children");
                    return self.do_insert_first(parent, new_child);
                }
            },
            InsertPos::After(prev) => {
                covered_by!("insert_after");
                assert_eq!(prev.parent(self), Some(parent));
                (prev, prev.next_sibling_cyclic(self))
            }
            InsertPos::Before(next) => {
                assert_eq!(next.parent(self), Some(parent));
                match next.prev_sibling(self) {
                    Some(prev) => {
                        covered_by!("insert_before_between");
                        (prev, next)
                    }
                    None => {
                        covered_by!("insert_before_first");
                        return self.do_insert_first(parent, new_child);
                    }
                }
            }
        };

        self.get_mut(new_child).next_sibling = prev.next_sibling(self);
        self.get_mut(new_child).prev_sibling_cyclic = prev;
        self.get_mut(prev).next_sibling = Some(new_child);
        self.get_mut(next_cyclic).prev_sibling_cyclic = new_child;
    }

    fn do_insert_first(&mut self, parent: NodeId, new_child: NodeId) {
        let first_child = parent.first_child(self);
        parent.set_fist_child(self, Some(new_child));
        self.get_mut(new_child).next_sibling = first_child;
        if let Some(first_child) = first_child {
            covered_by!("insert_first_with_children");
            self.get_mut(new_child).prev_sibling_cyclic = first_child.prev_sibling_cyclic(self);
            self.get_mut(first_child).prev_sibling_cyclic = new_child;
        } else {
            covered_by!("insert_first_no_children");
        }
    }

    pub fn detach(&mut self, node: NodeId) {
        let parent = match node.parent(self) {
            Some(parent) => parent,
            None => panic!("can't detach node without parent"),
        };
        let next_sibling = node.next_sibling(self);
        let prev_sibling = node.prev_sibling(self);
        let next_sibling_cyclic = node.next_sibling_cyclic(self);
        let prev_sibling_cyclic = node.prev_sibling_cyclic(self);

        {
            if parent.first_child(self) == Some(node) {
                covered_by!("detach_first");
                parent.set_fist_child(self, next_sibling);
            }
            if let Some(s) = prev_sibling {
                covered_by!("detach_non_last");
                self.get_mut(s).next_sibling = next_sibling;
            }

            if node == next_sibling_cyclic {
                covered_by!("detach_single");
                assert_eq!(node, prev_sibling_cyclic);
            } else {
                covered_by!("detach_mid");
                assert_ne!(node, prev_sibling_cyclic);
                self.get_mut(next_sibling_cyclic).prev_sibling_cyclic = prev_sibling_cyclic;
            }

            self.get_mut(node).parent = None;
            self.get_mut(node).next_sibling = None;
            self.get_mut(node).prev_sibling_cyclic = node;
        }

        node.check_invariants(self);
        parent.check_invariants(self);
        next_sibling_cyclic.check_invariants(self);
        prev_sibling_cyclic.check_invariants(self);
    }

    pub fn replace(&mut self, node: NodeId, replacement: NodeId) {
        assert!(replacement.parent(self).is_none());
        //TODO: deal with potential cycles
        let parent = match node.parent(self) {
            Some(parent) => parent,
            None => panic!("can't replace node without parent"),
        };

        let next_sibling = node.next_sibling(self);
        let prev_sibling = node.prev_sibling(self);
        let next_sibling_cyclic = node.next_sibling_cyclic(self);
        let prev_sibling_cyclic = node.prev_sibling_cyclic(self);

        {
            if parent.first_child(self) == Some(node) {
                parent.set_fist_child(self, Some(replacement));
            }
            if let Some(s) = prev_sibling {
                self.get_mut(s).next_sibling = Some(replacement);
            }

            if node == next_sibling_cyclic {
                assert_eq!(node, prev_sibling_cyclic);
            } else {
                assert_ne!(node, prev_sibling_cyclic);
                self.get_mut(next_sibling_cyclic).prev_sibling_cyclic = replacement;
            }

            self.get_mut(replacement).parent = Some(parent);
            self.get_mut(replacement).next_sibling = next_sibling;
            self.get_mut(replacement).prev_sibling_cyclic = prev_sibling_cyclic;

            self.get_mut(node).parent = None;
            self.get_mut(node).next_sibling = None;
            self.get_mut(node).prev_sibling_cyclic = node;
        }

        replacement.check_invariants(self);
        node.check_invariants(self);
        parent.check_invariants(self);
        next_sibling_cyclic.check_invariants(self);
        prev_sibling_cyclic.check_invariants(self);
    }

    fn new_node(&mut self, mut node: Node<ID, LD>) -> NodeId {
        let id = NodeId::from_idx(self.nodes.len());
        node.prev_sibling_cyclic = id;
        self.nodes.push(node);
        id
    }

    fn get(&self, id: NodeId) -> &Node<ID, LD> {
        &self.nodes[id.to_idx()]
    }

    fn get_mut(&mut self, id: NodeId) -> &mut Node<ID, LD> {
        &mut self.nodes[id.to_idx()]
    }
}

impl NodeId {
    pub fn parent<ID, LD>(self, tree: &Tree<ID, LD>) -> Option<Self> {
        tree.get(self).parent
    }

    pub fn first_child<ID, LD>(self, tree: &Tree<ID, LD>) -> Option<NodeId> {
        match tree.get(self).kind {
            NodeKind::Interior { first_child, .. } => first_child,
            NodeKind::Leaf { .. } => None,
        }
    }

    pub fn last_child<ID, LD>(self, tree: &Tree<ID, LD>) -> Option<NodeId> {
        let first = self.first_child(tree)?;
        Some(tree.get(first).prev_sibling_cyclic)
    }

    fn set_fist_child<ID, LD>(self, tree: &mut Tree<ID, LD>, first_child: Option<Self>) {
        match &mut tree.get_mut(self).kind {
            NodeKind::Interior {
                first_child: slot, ..
            } => *slot = first_child,
            NodeKind::Leaf { .. } => panic!("can't set first_child on a leaf node"),
        }
    }

    pub fn next_sibling<ID, LD>(self, tree: &Tree<ID, LD>) -> Option<Self> {
        tree.get(self).next_sibling
    }

    pub fn next_sibling_cyclic<ID, LD>(self, tree: &Tree<ID, LD>) -> Self {
        if self.prev_sibling_cyclic(tree) == self {
            return self;
        }
        self.next_sibling(tree)
            .unwrap_or_else(|| {
                let parent = self.parent(tree).unwrap();
                parent.first_child(tree).unwrap()
            })
    }

    pub fn prev_sibling<ID, LD>(self, tree: &Tree<ID, LD>) -> Option<Self> {
        let prev_cyclic = tree.get(self).prev_sibling_cyclic;
        if tree.get(prev_cyclic).next_sibling == Some(self) {
            Some(prev_cyclic)
        } else {
            None
        }
    }

    pub fn prev_sibling_cyclic<ID, LD>(self, tree: &Tree<ID, LD>) -> Self {
        tree.get(self).prev_sibling_cyclic
    }

    pub fn data<ID, LD>(self, tree: &Tree<ID, LD>) -> TreeData<&ID, &LD> {
        match &tree.get(self).kind {
            NodeKind::Interior { data, .. } => TreeData::Internal(data),
            NodeKind::Leaf { data } => TreeData::Leaf(data),
        }
    }

    fn check_invariants<ID, LD>(self, tree: &Tree<ID, LD>) {
        #[cfg(debug_assertions)]
            match self.parent(tree) {
            None => {
                assert!(self.next_sibling(tree).is_none());
                assert!(self.prev_sibling(tree).is_none());
            }
            Some(parent) => {
                let mut is_child = false;
                let mut curr = parent.first_child(tree);
                while let Some(child) = curr {
                    if child == self {
                        is_child = true;
                        break;
                    }
                    curr = child.next_sibling(tree);
                }
                assert!(is_child);

                if let Some(next) = self.next_sibling(tree) {
                    assert_eq!(
                        next.prev_sibling(tree),
                        Some(self),
                        "me: {:?}, next: {:?}, next.prev: {:?}",
                        self,
                        next,
                        next.prev_sibling(tree)
                    );
                }
                if let Some(prev) = self.prev_sibling(tree) {
                    assert_eq!(prev.next_sibling(tree), Some(self));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InsertPos, NodeId, TreeData};

    type Tree = super::Tree<(), ()>;

    fn print(tree: &Tree) -> String {
        let mut buff = String::new();
        go(tree, &mut buff, tree.root());
        return buff;

        fn go(tree: &Tree, buff: &mut String, node: NodeId) {
            node.check_invariants(tree);
            buff.push_str(&node.0.to_string());
            match node.data(tree) {
                TreeData::Leaf(_) => return,
                TreeData::Internal(_) => (),
            }
            buff.push_str(" (");
            let mut first = true;
            let mut curr = node.first_child(tree);
            while let Some(child) = curr {
                if !first {
                    buff.push_str(" ");
                }
                first = false;
                go(tree, buff, child);
                curr = child.next_sibling(tree);
            }
            buff.push_str(")");
        }
    }

    #[test]
    fn tree() {
        let mut t = Tree::new(());
        macro_rules! check {
            ($expr:expr) => {
                assert_eq!(print(&t), $expr);
            };
        }
        check!("1 ()");
        let root = t.root();
        let a = t.new_leaf(());
        let b = t.new_leaf(());
        let c = t.new_leaf(());
        let d = t.new_leaf(());

        {
            covers!("insert_last_no_children");
            t.insert_child(root, a, InsertPos::Last);
            check!("1 (2)");
        }

        {
            covers!("insert_last_with_children");
            t.insert_child(root, c, InsertPos::Last);
            check!("1 (2 4)");
        }

        {
            covers!("insert_before_between");
            t.insert_child(root, b, InsertPos::Before(c));
            check!("1 (2 3 4)");
        }

        {
            covers!("detach_mid");
            covers!("detach_non_last");
            t.detach(b);
            check!("1 (2 4)");
        }

        {
            covers!("detach_first");
            t.detach(a);
            check!("1 (4)");
        }

        {
            covers!("detach_single");
            t.detach(c);
            check!("1 ()");
        }

        {
            covers!("insert_first");
            covers!("insert_first_no_children");
            t.insert_child(root, c, InsertPos::First);
            check!("1 (4)");
        }

        {
            covers!("insert_before_first");
            t.insert_child(root, a, InsertPos::Before(c));
            check!("1 (2 4)");
        }

        {
            covers!("insert_after");
            t.insert_child(root, b, InsertPos::After(a));
            check!("1 (2 3 4)");
        }

        {
            covers!("insert_first_with_children");
            t.insert_child(root, d, InsertPos::First);
            check!("1 (5 2 3 4)");
        }

        t.detach(d);
        check!("1 (2 3 4)");

        t.replace(a, d);
        check!("1 (5 3 4)");

        t.replace(b, a);
        check!("1 (5 2 4)");

        t.replace(c, b);
        check!("1 (5 2 3)");
    }
}
