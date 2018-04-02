use parse_tree::{Node, Children};
use std::marker::PhantomData;

mod generated;
pub use self::generated::*;
mod ext;
pub use self::ext::*;


pub trait AstNode<'f>: Copy {
    fn cast(node: Node<'f>) -> Option<Self> where Self: Sized;

    fn node(self) -> Node<'f>;
}

pub struct AstChildren<'f, A: AstNode<'f>> {
    inner: Children<'f>,
    phantom: PhantomData<*const A>,
}

impl<'f, A: AstNode<'f>> AstChildren<'f, A> {
    pub fn new(children: Children<'f>) -> Self {
        AstChildren {
            inner: children,
            phantom: PhantomData,
        }
    }
}

impl<'f, A: AstNode<'f>> Iterator for AstChildren<'f, A> {
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.inner.next() {
            if let Some(a) = A::cast(node) {
                return Some(a);
            }
        }
        return None;
    }
}

pub mod search {
    use parse_tree::{Node, TextUnit};
    use parse_tree::search::{ancestors, find_leaf_at_offset, LeafAtOffset};
    use super::AstNode;

    pub fn ancestor<'f, T: AstNode<'f>>(node: Node<'f>) -> Option<T> {
        ancestors(node)
            .filter_map(T::cast)
            .next()
    }

    pub fn node_at_offset<'f, T: AstNode<'f>>(node: Node<'f>, offset: TextUnit) -> Option<T> {
        match find_leaf_at_offset(node, offset) {
            LeafAtOffset::None => None,
            LeafAtOffset::Single(node) => ancestor(node),
            LeafAtOffset::Between(left, right) => ancestor(left).or_else(|| ancestor(right)),
        }
    }
}
