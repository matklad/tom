//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{
    SyntaxNode, SyntaxNodeRef, AstNode, AstChildren, TreeRoot, RefRoot, OwnedRoot, TomTypes,
    symbol::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DocNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type Doc<'a> = DocNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type Table<'a> = TableNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayTableNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type ArrayTable<'a> = ArrayTableNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableHeaderNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type TableHeader<'a> = TableHeaderNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntryNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type Entry<'a> = EntryNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type Key<'a> = KeyNode<RefRoot<'a>>;

pub enum KeyKind<'a> {
    StringLit(StringLit<'a>),
    BareKey(BareKey<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type Value<'a> = ValueNode<RefRoot<'a>>;

pub enum ValueKind<'a> {
    Array(Array<'a>),
    Dict(Dict<'a>),
    Number(Number<'a>),
    Bool(Bool<'a>),
    DateTime(DateTime<'a>),
    StringLit(StringLit<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLitNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type StringLit<'a> = StringLitNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BareKeyNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type BareKey<'a> = BareKeyNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type Array<'a> = ArrayNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DictNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type Dict<'a> = DictNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type Number<'a> = NumberNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type Bool<'a> = BoolNode<RefRoot<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTimeNode<R: TreeRoot<TomTypes> = OwnedRoot>(SyntaxNode<R>);
pub type DateTime<'a> = DateTimeNode<RefRoot<'a>>;

impl<'a> AstNode<'a> for Doc<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<Doc<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Doc<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> Doc<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Doc> {
        match node.symbol() {
            DOC => Some(DocNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn tables(self) -> AstChildren<'a, Table<'a>> {
        AstChildren::new(self.syntax())
    }
    pub fn array_tables(self) -> AstChildren<'a, ArrayTable<'a>> {
        AstChildren::new(self.syntax())
    }
    pub fn entries(self) -> AstChildren<'a, Entry<'a>> {
        AstChildren::new(self.syntax())
    }
}

impl<'a> AstNode<'a> for Table<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<Table<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Table<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> Table<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Table> {
        match node.symbol() {
            TABLE => Some(TableNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn header(self) -> TableHeader<'a> {
        AstChildren::new(self.syntax()).next().unwrap()
    }
    pub fn entries(self) -> AstChildren<'a, Entry<'a>> {
        AstChildren::new(self.syntax())
    }
}

impl<'a> AstNode<'a> for ArrayTable<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<ArrayTable<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: ArrayTable<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> ArrayTable<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<ArrayTable> {
        match node.symbol() {
            ARRAY_TABLE => Some(ArrayTableNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn header(self) -> TableHeader<'a> {
        AstChildren::new(self.syntax()).next().unwrap()
    }
    pub fn entries(self) -> AstChildren<'a, Entry<'a>> {
        AstChildren::new(self.syntax())
    }
}

impl<'a> AstNode<'a> for TableHeader<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<TableHeader<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: TableHeader<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> TableHeader<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<TableHeader> {
        match node.symbol() {
            TABLE_HEADER => Some(TableHeaderNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn keys(self) -> AstChildren<'a, Key<'a>> {
        AstChildren::new(self.syntax())
    }
}

impl<'a> AstNode<'a> for Entry<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<Entry<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Entry<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> Entry<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Entry> {
        match node.symbol() {
            ENTRY => Some(EntryNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn keys(self) -> AstChildren<'a, Key<'a>> {
        AstChildren::new(self.syntax())
    }
    pub fn value(self) -> Value<'a> {
        AstChildren::new(self.syntax()).next().unwrap()
    }
}

impl<'a> AstNode<'a> for Key<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<Key<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Key<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> Key<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Key> {
        match node.symbol() {
            KEY => Some(KeyNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn kind(self) -> KeyKind<'a> {
        let node = self.syntax().children().next().unwrap();
        if let Some(node) = StringLit::cast(node) {
            return KeyKind::StringLit(node);
        }
        if let Some(node) = BareKey::cast(node) {
            return KeyKind::BareKey(node);
        }
        unreachable!()
    }
}

impl<'a> AstNode<'a> for Value<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<Value<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Value<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> Value<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Value> {
        match node.symbol() {
            VALUE => Some(ValueNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn kind(self) -> ValueKind<'a> {
        let node = self.syntax().children().next().unwrap();
        if let Some(node) = Array::cast(node) {
            return ValueKind::Array(node);
        }
        if let Some(node) = Dict::cast(node) {
            return ValueKind::Dict(node);
        }
        if let Some(node) = Number::cast(node) {
            return ValueKind::Number(node);
        }
        if let Some(node) = Bool::cast(node) {
            return ValueKind::Bool(node);
        }
        if let Some(node) = DateTime::cast(node) {
            return ValueKind::DateTime(node);
        }
        if let Some(node) = StringLit::cast(node) {
            return ValueKind::StringLit(node);
        }
        unreachable!()
    }
}

impl<'a> AstNode<'a> for StringLit<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<StringLit<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: StringLit<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> StringLit<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<StringLit> {
        match node.symbol() {
            BASIC_STRING => Some(StringLitNode(node)),
            MULTILINE_BASIC_STRING => Some(StringLitNode(node)),
            LITERAL_STRING => Some(StringLitNode(node)),
            MULTILINE_LITERAL_STRING => Some(StringLitNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn text(self) -> &'a str {
        self.syntax().leaf_text().unwrap()
    }
}

impl<'a> AstNode<'a> for BareKey<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<BareKey<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: BareKey<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> BareKey<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<BareKey> {
        match node.symbol() {
            BARE_KEY => Some(BareKeyNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn text(self) -> &'a str {
        self.syntax().leaf_text().unwrap()
    }
}

impl<'a> AstNode<'a> for Array<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<Array<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Array<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> Array<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Array> {
        match node.symbol() {
            ARRAY => Some(ArrayNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn values(self) -> AstChildren<'a, Value<'a>> {
        AstChildren::new(self.syntax())
    }
}

impl<'a> AstNode<'a> for Dict<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<Dict<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Dict<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> Dict<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Dict> {
        match node.symbol() {
            DICT => Some(DictNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn entries(self) -> AstChildren<'a, Entry<'a>> {
        AstChildren::new(self.syntax())
    }
}

impl<'a> AstNode<'a> for Number<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<Number<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Number<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> Number<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Number> {
        match node.symbol() {
            NUMBER => Some(NumberNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn text(self) -> &'a str {
        self.syntax().leaf_text().unwrap()
    }
}

impl<'a> AstNode<'a> for Bool<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<Bool<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Bool<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> Bool<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Bool> {
        match node.symbol() {
            BOOL => Some(BoolNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn text(self) -> &'a str {
        self.syntax().leaf_text().unwrap()
    }
}

impl<'a> AstNode<'a> for DateTime<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::cast(node)
    }
    fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }
}

impl<'a> From<DateTime<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: DateTime<'a>) -> SyntaxNodeRef<'a> {
        ast.syntax()
    }
}

impl<'a> DateTime<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<DateTime> {
        match node.symbol() {
            DATE_TIME => Some(DateTimeNode(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> {
        self.0
    }

    pub fn text(self) -> &'a str {
        self.syntax().leaf_text().unwrap()
    }
}
