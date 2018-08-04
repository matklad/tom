use {
    TomlDoc, CstNode, AstNode, AstChildren, CstNodeKind,
    symbol::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Doc(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Table(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayTable(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableHeader(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key(CstNode);

pub enum KeyKind {
    StringLit(StringLit),
    BareKey(BareKey),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value(CstNode);

pub enum ValueKind {
    Array(Array),
    Dict(Dict),
    Float(Float),
    Integer(Integer),
    Bool(Bool),
    DateTime(DateTime),
    StringLit(StringLit),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLit(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BareKey(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Array(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dict(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Float(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Integer(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bool(CstNode);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime(CstNode);


impl AstNode for Doc {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<Doc> for CstNode {
    fn from(ast: Doc) -> CstNode { ast.cst() }
}

impl Doc {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<Doc> {
        match node.symbol(doc) {
            DOC => Some(Doc(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn tables(self, doc: &TomlDoc) -> AstChildren<Table> {
        AstChildren::new(self.cst(), doc)
    }
    pub fn array_tables(self, doc: &TomlDoc) -> AstChildren<ArrayTable> {
        AstChildren::new(self.cst(), doc)
    }
    pub fn entries(self, doc: &TomlDoc) -> AstChildren<Entry> {
        AstChildren::new(self.cst(), doc)
    }
}

impl AstNode for Table {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<Table> for CstNode {
    fn from(ast: Table) -> CstNode { ast.cst() }
}

impl Table {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<Table> {
        match node.symbol(doc) {
            TABLE => Some(Table(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn header(self, doc: &TomlDoc) -> TableHeader {
        AstChildren::new(self.cst(), doc).next().unwrap()
    }
    pub fn entries(self, doc: &TomlDoc) -> AstChildren<Entry> {
        AstChildren::new(self.cst(), doc)
    }
}

impl AstNode for ArrayTable {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<ArrayTable> for CstNode {
    fn from(ast: ArrayTable) -> CstNode { ast.cst() }
}

impl ArrayTable {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<ArrayTable> {
        match node.symbol(doc) {
            ARRAY_TABLE => Some(ArrayTable(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn header(self, doc: &TomlDoc) -> TableHeader {
        AstChildren::new(self.cst(), doc).next().unwrap()
    }
    pub fn entries(self, doc: &TomlDoc) -> AstChildren<Entry> {
        AstChildren::new(self.cst(), doc)
    }
}

impl AstNode for TableHeader {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<TableHeader> for CstNode {
    fn from(ast: TableHeader) -> CstNode { ast.cst() }
}

impl TableHeader {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<TableHeader> {
        match node.symbol(doc) {
            TABLE_HEADER => Some(TableHeader(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn keys(self, doc: &TomlDoc) -> AstChildren<Key> {
        AstChildren::new(self.cst(), doc)
    }
}

impl AstNode for Entry {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<Entry> for CstNode {
    fn from(ast: Entry) -> CstNode { ast.cst() }
}

impl Entry {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<Entry> {
        match node.symbol(doc) {
            ENTRY => Some(Entry(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn keys(self, doc: &TomlDoc) -> AstChildren<Key> {
        AstChildren::new(self.cst(), doc)
    }
    pub fn value(self, doc: &TomlDoc) -> Value {
        AstChildren::new(self.cst(), doc).next().unwrap()
    }
}

impl AstNode for Key {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<Key> for CstNode {
    fn from(ast: Key) -> CstNode { ast.cst() }
}

impl Key {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<Key> {
        match node.symbol(doc) {
            KEY => Some(Key(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn kind(self, doc: &TomlDoc) -> KeyKind {
        let node = self.cst().children(doc).first().unwrap();
        if let Some(node) = StringLit::cast(node, doc) {
            return KeyKind::StringLit(node);
        }
        if let Some(node) = BareKey::cast(node, doc) {
            return KeyKind::BareKey(node);
        }
        unreachable!()
    }

}

impl AstNode for Value {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<Value> for CstNode {
    fn from(ast: Value) -> CstNode { ast.cst() }
}

impl Value {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<Value> {
        match node.symbol(doc) {
            VALUE => Some(Value(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn kind(self, doc: &TomlDoc) -> ValueKind {
        let node = self.cst().children(doc).first().unwrap();
        if let Some(node) = Array::cast(node, doc) {
            return ValueKind::Array(node);
        }
        if let Some(node) = Dict::cast(node, doc) {
            return ValueKind::Dict(node);
        }
        if let Some(node) = Float::cast(node, doc) {
            return ValueKind::Float(node);
        }
        if let Some(node) = Integer::cast(node, doc) {
            return ValueKind::Integer(node);
        }
        if let Some(node) = Bool::cast(node, doc) {
            return ValueKind::Bool(node);
        }
        if let Some(node) = DateTime::cast(node, doc) {
            return ValueKind::DateTime(node);
        }
        if let Some(node) = StringLit::cast(node, doc) {
            return ValueKind::StringLit(node);
        }
        unreachable!()
    }

}

impl AstNode for StringLit {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<StringLit> for CstNode {
    fn from(ast: StringLit) -> CstNode { ast.cst() }
}

impl StringLit {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<StringLit> {
        match node.symbol(doc) {
            BASIC_STRING => Some(StringLit(node)),
            MULTILINE_BASIC_STRING => Some(StringLit(node)),
            LITERAL_STRING => Some(StringLit(node)),
            MULTILINE_LITERAL_STRING => Some(StringLit(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn text(self, doc: &TomlDoc) -> &str {
        match self.cst().kind(doc) {
            CstNodeKind::Leaf(text) => text,
            CstNodeKind::Internal(_) => unreachable!(),
        }
    }
}

impl AstNode for BareKey {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<BareKey> for CstNode {
    fn from(ast: BareKey) -> CstNode { ast.cst() }
}

impl BareKey {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<BareKey> {
        match node.symbol(doc) {
            BARE_KEY => Some(BareKey(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn text(self, doc: &TomlDoc) -> &str {
        match self.cst().kind(doc) {
            CstNodeKind::Leaf(text) => text,
            CstNodeKind::Internal(_) => unreachable!(),
        }
    }
}

impl AstNode for Array {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<Array> for CstNode {
    fn from(ast: Array) -> CstNode { ast.cst() }
}

impl Array {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<Array> {
        match node.symbol(doc) {
            ARRAY => Some(Array(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn values(self, doc: &TomlDoc) -> AstChildren<Value> {
        AstChildren::new(self.cst(), doc)
    }
}

impl AstNode for Dict {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<Dict> for CstNode {
    fn from(ast: Dict) -> CstNode { ast.cst() }
}

impl Dict {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<Dict> {
        match node.symbol(doc) {
            DICT => Some(Dict(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn entries(self, doc: &TomlDoc) -> AstChildren<Entry> {
        AstChildren::new(self.cst(), doc)
    }
}

impl AstNode for Float {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<Float> for CstNode {
    fn from(ast: Float) -> CstNode { ast.cst() }
}

impl Float {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<Float> {
        match node.symbol(doc) {
            FLOAT => Some(Float(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn text(self, doc: &TomlDoc) -> &str {
        match self.cst().kind(doc) {
            CstNodeKind::Leaf(text) => text,
            CstNodeKind::Internal(_) => unreachable!(),
        }
    }
}

impl AstNode for Integer {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<Integer> for CstNode {
    fn from(ast: Integer) -> CstNode { ast.cst() }
}

impl Integer {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<Integer> {
        match node.symbol(doc) {
            INTEGER => Some(Integer(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn text(self, doc: &TomlDoc) -> &str {
        match self.cst().kind(doc) {
            CstNodeKind::Leaf(text) => text,
            CstNodeKind::Internal(_) => unreachable!(),
        }
    }
}

impl AstNode for Bool {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<Bool> for CstNode {
    fn from(ast: Bool) -> CstNode { ast.cst() }
}

impl Bool {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<Bool> {
        match node.symbol(doc) {
            BOOL => Some(Bool(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn text(self, doc: &TomlDoc) -> &str {
        match self.cst().kind(doc) {
            CstNodeKind::Leaf(text) => text,
            CstNodeKind::Internal(_) => unreachable!(),
        }
    }
}

impl AstNode for DateTime {
    fn cast(node: CstNode, doc: &TomlDoc) -> Option<Self> where Self: Sized { Self::cast(node, doc) }
}

impl From<DateTime> for CstNode {
    fn from(ast: DateTime) -> CstNode { ast.cst() }
}

impl DateTime {
    pub fn cast(node: CstNode, doc: &TomlDoc) -> Option<DateTime> {
        match node.symbol(doc) {
            DATE_TIME => Some(DateTime(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode { self.0 }

    pub fn text(self, doc: &TomlDoc) -> &str {
        match self.cst().kind(doc) {
            CstNodeKind::Leaf(text) => text,
            CstNodeKind::Internal(_) => unreachable!(),
        }
    }
}
