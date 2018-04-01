use parse_tree::Node;
use ast::{AstNode, AstChildren};
use symbols::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct File<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BareKey<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Array<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dict<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Number<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bool<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DateTime<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KeyVal<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Key<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Val<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Table<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ArrayTable<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TableHeader<'p>(Node<'p>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StringLit<'p>(Node<'p>);


impl<'p> AstNode<'p> for File<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == FILE { Some(File(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for BareKey<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == BARE_KEY { Some(BareKey(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Array<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == ARRAY { Some(Array(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Dict<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == DICT { Some(Dict(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Number<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == NUMBER { Some(Number(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Bool<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == BOOL { Some(Bool(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for DateTime<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == DATE_TIME { Some(DateTime(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for KeyVal<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == KEY_VAL { Some(KeyVal(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Key<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == KEY { Some(Key(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Val<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == VAL { Some(Val(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for Table<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == TABLE { Some(Table(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for ArrayTable<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == ARRAY_TABLE { Some(ArrayTable(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for TableHeader<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if node.symbol() == TABLE_HEADER { Some(TableHeader(node)) } else { None }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> AstNode<'p> for StringLit<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            BASIC_STRING => Some(StringLit(node)),
            MULTILINE_BASIC_STRING => Some(StringLit(node)),
            LITERAL_STRING => Some(StringLit(node)),
            MULTILINE_LITERAL_STRING => Some(StringLit(node)),
            _ => None,
        }
    }
    fn node(self) -> Node<'p> { self.0 }
}

impl<'p> File<'p> {
    fn tables(&self) -> AstChildren<'p, Table<'p>> {
        AstChildren::new(self.node().children())
    }
    fn array_tables(&self) -> AstChildren<'p, ArrayTable<'p>> {
        AstChildren::new(self.node().children())
    }
}
impl<'p> TableHeader<'p> {
    fn keys(&self) -> AstChildren<'p, Key<'p>> {
        AstChildren::new(self.node().children())
    }
}
impl<'p> KeyVal<'p> {
    fn key(&self) -> Key<'p> {
        AstChildren::new(self.node().children()).next().unwrap()
    }
    fn val(&self) -> Val<'p> {
        AstChildren::new(self.node().children()).next().unwrap()
    }
}
