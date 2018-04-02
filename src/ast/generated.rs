use parse_tree::Node;
use ast::{AstNode, AstChildren};
use symbols::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct File<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BareKey<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Array<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dict<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bool<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyVal<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Table<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayTable<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableHeader<'p>(Node<'p>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLit<'p>(Node<'p>);


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key<'p> {
    StringLit(StringLit<'p>),
    BareKey(BareKey<'p>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Val<'p> {
    Array(Array<'p>),
    Dict(Dict<'p>),
    Number(Number<'p>),
    Bool(Bool<'p>),
    DateTime(DateTime<'p>),
    StringLit(StringLit<'p>),
}

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

impl<'p> AstNode<'p> for Key<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if let Some(n) = StringLit::cast(node) { return Some(Key::StringLit(n)); }
        if let Some(n) = BareKey::cast(node) { return Some(Key::BareKey(n)); }
        None
    }
    fn node(self) -> Node<'p> {
        match self {
            Key::StringLit(n) => n.node(),
            Key::BareKey(n) => n.node(),
        }
    }
}

impl<'p> AstNode<'p> for Val<'p> {
    fn cast(node: Node<'p>) -> Option<Self> where Self: Sized {
        if let Some(n) = Array::cast(node) { return Some(Val::Array(n)); }
        if let Some(n) = Dict::cast(node) { return Some(Val::Dict(n)); }
        if let Some(n) = Number::cast(node) { return Some(Val::Number(n)); }
        if let Some(n) = Bool::cast(node) { return Some(Val::Bool(n)); }
        if let Some(n) = DateTime::cast(node) { return Some(Val::DateTime(n)); }
        if let Some(n) = StringLit::cast(node) { return Some(Val::StringLit(n)); }
        None
    }
    fn node(self) -> Node<'p> {
        match self {
            Val::Array(n) => n.node(),
            Val::Dict(n) => n.node(),
            Val::Number(n) => n.node(),
            Val::Bool(n) => n.node(),
            Val::DateTime(n) => n.node(),
            Val::StringLit(n) => n.node(),
        }
    }
}

impl<'p> File<'p> {
    pub fn tables(&self) -> AstChildren<'p, Table<'p>> {
        AstChildren::new(self.node().children())
    }
    pub fn array_tables(&self) -> AstChildren<'p, ArrayTable<'p>> {
        AstChildren::new(self.node().children())
    }
}
impl<'p> TableHeader<'p> {
    pub fn keys(&self) -> AstChildren<'p, Key<'p>> {
        AstChildren::new(self.node().children())
    }
}
impl<'p> KeyVal<'p> {
    pub fn key(&self) -> Key<'p> {
        AstChildren::new(self.node().children()).next().unwrap()
    }
    pub fn val(&self) -> Val<'p> {
        AstChildren::new(self.node().children()).next().unwrap()
    }
}
