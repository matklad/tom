use std::{
    num::NonZeroU32,
};


#[derive(Debug)]
pub(crate) struct Tree<ID, LD> {
    nodes: Vec<Node<ID, LD>>
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct NodeId(NonZeroU32);


impl NodeId {
    pub(crate) fn to_idx(self) -> usize { (self.0.get() - 1) as usize }
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

impl <ID, LD> Node<ID, LD> {
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
    Interior(ID),
    Leaf(LD),
}

#[derive(Debug)]
enum NodeKind<ID, LD> {
    Interior {
        first_child: Option<NodeId>,
        data: ID,
    },
    Leaf {
        data: LD
    }
}

impl<ID, LD> Tree<ID, LD> {
    pub fn root(&self) -> NodeId {
        NodeId::from_idx(0)
    }

    pub fn new(root_data: ID) -> Tree<ID, LD> {
        let root = Node::new(NodeKind::Interior { first_child: None, data: root_data });
        let mut tree = Tree { nodes: Vec::with_capacity(1) };
        tree.new_node(root);
        tree
    }

    pub fn new_leaf(&mut self, data: LD) -> NodeId {
        self.new_node(Node::new(NodeKind::Leaf { data }))
    }

    pub fn new_internal(&mut self, data: ID) -> NodeId {
        self.new_node(Node::new(NodeKind::Interior { first_child: None, data }))
    }

    pub fn len(&self) -> usize { self.nodes.len() }

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

    fn set_fist_child<ID, LD>(self, tree: &mut Tree<ID, LD>, first_child: Option<Self>) {
        match &mut tree.get_mut(self).kind {
            NodeKind::Interior { first_child: slot, .. } => *slot = first_child,
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
        self.next_sibling(tree).unwrap_or_else(|| {
            self.parent(tree).unwrap().children(tree).first().unwrap()
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

    pub fn children<ID, LD>(self, tree: &Tree<ID, LD>) -> Children<ID, LD> {
        Children { tree, node: self }
    }

    pub fn data<ID, LD>(self, tree: &Tree<ID, LD>) -> TreeData<&ID, &LD> {
        match &tree.get(self).kind {
            NodeKind::Interior { data, .. } => TreeData::Interior(data),
            NodeKind::Leaf { data } => TreeData::Leaf(data),
        }
    }

    pub fn is_leaf<ID, LD>(self, tree: &Tree<ID, LD>) -> bool {
        match &tree.get(self).kind {
           NodeKind::Interior { .. } => false,
           NodeKind::Leaf { .. } => true,
        }
    }

    pub fn detach<ID, LD>(self, tree: &mut Tree<ID, LD>) {
        let parent = match self.parent(tree) {
            Some(parent) => parent,
            None => panic!("can't detach node without parent"),
        };
        let next_sibling = self.next_sibling(tree);
        let prev_sibling = self.prev_sibling(tree);
        let next_sibling_cyclic = self.next_sibling_cyclic(tree);
        let prev_sibling_cyclic = self.prev_sibling_cyclic(tree);

        {
            if parent.children(tree).first() == Some(self) {
                parent.set_fist_child(tree, next_sibling);
            }
            if let Some(s) = prev_sibling {
                tree.get_mut(s).next_sibling = next_sibling;
            }

            if self == next_sibling_cyclic {
                assert!(self == prev_sibling_cyclic);
            } else {
                assert!(self != prev_sibling_cyclic);
                tree.get_mut(next_sibling_cyclic).prev_sibling_cyclic = prev_sibling_cyclic;
            }

            tree.get_mut(self).parent = None;
            tree.get_mut(self).next_sibling = None;
            tree.get_mut(self).prev_sibling_cyclic = self;
        }

        self.check_invariants(tree);
        parent.check_invariants(tree);
        next_sibling_cyclic.check_invariants(tree);
        prev_sibling_cyclic.check_invariants(tree);
    }

    pub fn replace<ID, LD>(self, tree: &mut Tree<ID, LD>, replacement: Self) {
        assert!(replacement.parent(tree).is_none());
        let parent = match self.parent(tree) {
            Some(parent) => parent,
            None => panic!("can't replace node without parent"),
        };

        let next_sibling = self.next_sibling(tree);
        let prev_sibling = self.prev_sibling(tree);
        let next_sibling_cyclic = self.next_sibling_cyclic(tree);
        let prev_sibling_cyclic = self.prev_sibling_cyclic(tree);

        {
            if parent.children(tree).first() == Some(self) {
                parent.set_fist_child(tree, Some(replacement));
            }
            if let Some(s) = prev_sibling {
                tree.get_mut(s).next_sibling = Some(replacement);
            }

            if self == next_sibling_cyclic {
                assert!(self == prev_sibling_cyclic);
            } else {
                assert!(self != prev_sibling_cyclic);
                tree.get_mut(next_sibling_cyclic).prev_sibling_cyclic = replacement;
            }

            tree.get_mut(replacement).next_sibling = next_sibling;
            tree.get_mut(replacement).parent = Some(parent);
        }

        replacement.check_invariants(tree);
        self.check_invariants(tree);
        parent.check_invariants(tree);
        next_sibling_cyclic.check_invariants(tree);
        prev_sibling_cyclic.check_invariants(tree);
    }

    pub fn append_child<ID, LD>(self, tree: &mut Tree<ID, LD>, child: Self) {
        assert!(child.parent(tree).is_none());
        match self.children(tree).last() {
            None => {
                tree.get_mut(child).parent = Some(self);
                self.set_fist_child(tree, Some(child));
            }
            Some(last) => {
                last.append_sibling(tree, child);
            }
        }
        self.check_invariants(tree);
        child.check_invariants(tree);
    }

    pub fn append_sibling<ID, LD>(self, tree: &mut Tree<ID, LD>, sibling: Self) {
        assert!(sibling.parent(tree).is_none());
        let parent = match self.parent(tree) {
            Some(parent) => parent,
            None => panic!("can't append sibling to node without a parent"),
        };
        let next_sibling = self.next_sibling(tree);
        let next_sibling_cyclic = self.next_sibling_cyclic(tree);

        {
            tree.get_mut(sibling).parent = Some(parent);
            tree.get_mut(sibling).next_sibling = next_sibling;
            tree.get_mut(sibling).prev_sibling_cyclic = self;

            tree.get_mut(self).next_sibling = Some(sibling);
            tree.get_mut(next_sibling_cyclic).prev_sibling_cyclic = sibling;
        }
        self.check_invariants(tree);
        sibling.check_invariants(tree);
    }

    fn check_invariants<ID, LD>(self, tree: &Tree<ID, LD>) {
        match self.parent(tree) {
            None => {
                assert!(self.next_sibling(tree).is_none());
                assert!(self.prev_sibling(tree).is_none());
            }
            Some(parent) => {
                assert!(parent.children(tree).iter().any(|c| c == self));
                if let Some(next) = self.next_sibling(tree) {
                    assert!(
                        next.prev_sibling(tree) == Some(self),
                        "me: {:?}, next: {:?}, next.prev: {:?}", self, next, next.prev_sibling(tree)
                    );
                }
                if let Some(prev) = self.prev_sibling(tree) {
                    assert!(prev.next_sibling(tree) == Some(self));
                }
            }
        }
    }
}

pub(crate) struct Children<'a, ID: 'a, LD: 'a> {
    tree: &'a Tree<ID, LD>,
    node: NodeId,
}

impl<'a, ID: 'a, LD: 'a> Copy for Children<'a, ID, LD> {
}

impl<'a, ID: 'a, LD: 'a> Clone for Children<'a, ID, LD> {
    fn clone(&self) -> Self { *self }
}

impl<'a, ID: 'a, LD: 'a> Children<'a, ID, LD> {
    pub fn first(self) -> Option<NodeId> {
        match self.node().kind {
            NodeKind::Interior { first_child, .. } => first_child,
            NodeKind::Leaf { .. } => None,
        }
    }

    pub fn last(self) -> Option<NodeId> {
        let first = self.first()?;
        Some(self.tree.get(first).prev_sibling_cyclic)
    }

    pub fn iter(self) -> ChildrenIter<'a, ID, LD> {
        ChildrenIter { curr: self.first(), tree: self.tree }
    }

    pub fn rev(self) -> RevChildrenIter<'a, ID, LD> {
        RevChildrenIter { curr: self.last(), tree: self.tree }
    }

    fn node(self) -> &'a Node<ID, LD> {
        self.tree.get(self.node)
    }
}

impl<'a, ID: 'a, LD: 'a> IntoIterator for Children<'a, ID, LD> {
    type Item = NodeId;
    type IntoIter = ChildrenIter<'a, ID, LD>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

pub(crate) struct ChildrenIter<'a, ID: 'a, LD: 'a> {
    curr: Option<NodeId>,
    tree: &'a Tree<ID, LD>
}

impl<'a, ID: 'a, LD: 'a> Iterator for ChildrenIter<'a, ID, LD> {
    type Item = NodeId;
    fn next(&mut self) -> Option<NodeId> {
        self.curr.map(|id| {
            self.curr = id.next_sibling(self.tree);
            id
        })
    }
}

pub(crate) struct RevChildrenIter<'a, ID: 'a, LD: 'a> {
    curr: Option<NodeId>,
    tree: &'a Tree<ID, LD>,
}

impl<'a, ID: 'a, LD: 'a> Iterator for RevChildrenIter<'a, ID, LD> {
    type Item = NodeId;
    fn next(&mut self) -> Option<NodeId> {
        self.curr.map(|id| {
            self.curr = id.prev_sibling(self.tree);
            id
        })
    }
}

#[cfg(test)]
mod tests {
    use super::NodeId;
    type Tree = super::Tree<(), ()>;

    fn print(tree: &Tree) -> String {
        let mut buff = String::new();
        go(tree, &mut buff, tree.root());
        return buff;

        fn go(tree: &Tree, buff: &mut String, node: NodeId) {
            node.check_invariants(tree);
            buff.push_str(&node.0.to_string());
            if node.is_leaf(tree) {
                return;
            }
            buff.push_str(" (");
            let mut first = true;
            for child in node.children(tree).iter() {
                if !first { buff.push_str(" "); }
                first = false;
                go(tree, buff, child);
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

        root.append_child(&mut t, a);
        check!("1 (2)");

        root.append_child(&mut t, b);
        check!("1 (2 3)");

        root.append_child(&mut t, c);
        check!("1 (2 3 4)");

        b.detach(&mut t);
        check!("1 (2 4)");

        a.detach(&mut t);
        check!("1 (4)");

        c.detach(&mut t);
        check!("1 ()");

        root.append_child(&mut t, a);
        check!("1 (2)");

        a.append_sibling(&mut t, c);
        check!("1 (2 4)");

        a.append_sibling(&mut t, b);
        check!("1 (2 3 4)");
    }
}
