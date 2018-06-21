use {
    CstNode,
    ast::{AstNode, AstChildren},
    symbols::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Doc<'f>(CstNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Table<'f>(CstNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayTable<'f>(CstNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableHeader<'f>(CstNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry<'f>(CstNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key<'f>(CstNode<'f>);

pub enum KeyKind<'f> {
    StringLit(StringLit<'f>),
    BareKey(BareKey<'f>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value<'f>(CstNode<'f>);

pub enum ValueKind<'f> {
    Array(Array<'f>),
    Dict(Dict<'f>),
    Number(Number<'f>),
    Bool(Bool<'f>),
    DateTime(DateTime<'f>),
    StringLit(StringLit<'f>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLit<'f>(CstNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BareKey<'f>(CstNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Array<'f>(CstNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dict<'f>(CstNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number<'f>(CstNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bool<'f>(CstNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime<'f>(CstNode<'f>);


impl<'f> AstNode<'f> for Doc<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<Doc<'f>> for CstNode<'f> {
    fn from(ast: Doc<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> Doc<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<Doc<'f>> {
        match node.symbol() {
            DOC => Some(Doc(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }

    pub fn tables(self) -> AstChildren<'f, Table<'f>> {
        AstChildren::new(self.cst().children())
    }
    pub fn array_tables(self) -> AstChildren<'f, ArrayTable<'f>> {
        AstChildren::new(self.cst().children())
    }
    pub fn entries(self) -> AstChildren<'f, Entry<'f>> {
        AstChildren::new(self.cst().children())
    }
}

impl<'f> AstNode<'f> for Table<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<Table<'f>> for CstNode<'f> {
    fn from(ast: Table<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> Table<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<Table<'f>> {
        match node.symbol() {
            TABLE => Some(Table(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }

    pub fn header(self) -> TableHeader<'f> {
        AstChildren::new(self.cst().children()).next().unwrap()
    }
    pub fn entries(self) -> AstChildren<'f, Entry<'f>> {
        AstChildren::new(self.cst().children())
    }
}

impl<'f> AstNode<'f> for ArrayTable<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<ArrayTable<'f>> for CstNode<'f> {
    fn from(ast: ArrayTable<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> ArrayTable<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<ArrayTable<'f>> {
        match node.symbol() {
            ARRAY_TABLE => Some(ArrayTable(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }

    pub fn header(self) -> TableHeader<'f> {
        AstChildren::new(self.cst().children()).next().unwrap()
    }
    pub fn entries(self) -> AstChildren<'f, Entry<'f>> {
        AstChildren::new(self.cst().children())
    }
}

impl<'f> AstNode<'f> for TableHeader<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<TableHeader<'f>> for CstNode<'f> {
    fn from(ast: TableHeader<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> TableHeader<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<TableHeader<'f>> {
        match node.symbol() {
            TABLE_HEADER => Some(TableHeader(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }

    pub fn keys(self) -> AstChildren<'f, Key<'f>> {
        AstChildren::new(self.cst().children())
    }
}

impl<'f> AstNode<'f> for Entry<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<Entry<'f>> for CstNode<'f> {
    fn from(ast: Entry<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> Entry<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<Entry<'f>> {
        match node.symbol() {
            ENTRY => Some(Entry(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }

    pub fn keys(self) -> AstChildren<'f, Key<'f>> {
        AstChildren::new(self.cst().children())
    }
    pub fn value(self) -> Value<'f> {
        AstChildren::new(self.cst().children()).next().unwrap()
    }
}

impl<'f> AstNode<'f> for Key<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<Key<'f>> for CstNode<'f> {
    fn from(ast: Key<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> Key<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<Key<'f>> {
        match node.symbol() {
            KEY => Some(Key(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }

    pub fn kind(self) -> KeyKind<'f> {
        let node = self.cst().children().next().unwrap();
        if let Some(node) = StringLit::cast(node) {
            return KeyKind::StringLit(node);
        }
        if let Some(node) = BareKey::cast(node) {
            return KeyKind::BareKey(node);
        }
        unreachable!()
    }

}

impl<'f> AstNode<'f> for Value<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<Value<'f>> for CstNode<'f> {
    fn from(ast: Value<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> Value<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<Value<'f>> {
        match node.symbol() {
            VALUE => Some(Value(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }

    pub fn kind(self) -> ValueKind<'f> {
        let node = self.cst().children().next().unwrap();
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

impl<'f> AstNode<'f> for StringLit<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<StringLit<'f>> for CstNode<'f> {
    fn from(ast: StringLit<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> StringLit<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<StringLit<'f>> {
        match node.symbol() {
            BASIC_STRING => Some(StringLit(node)),
            MULTILINE_BASIC_STRING => Some(StringLit(node)),
            LITERAL_STRING => Some(StringLit(node)),
            MULTILINE_LITERAL_STRING => Some(StringLit(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for BareKey<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<BareKey<'f>> for CstNode<'f> {
    fn from(ast: BareKey<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> BareKey<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<BareKey<'f>> {
        match node.symbol() {
            BARE_KEY => Some(BareKey(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for Array<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<Array<'f>> for CstNode<'f> {
    fn from(ast: Array<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> Array<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<Array<'f>> {
        match node.symbol() {
            ARRAY => Some(Array(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }

    pub fn values(self) -> AstChildren<'f, Value<'f>> {
        AstChildren::new(self.cst().children())
    }
}

impl<'f> AstNode<'f> for Dict<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<Dict<'f>> for CstNode<'f> {
    fn from(ast: Dict<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> Dict<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<Dict<'f>> {
        match node.symbol() {
            DICT => Some(Dict(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }

    pub fn entries(self) -> AstChildren<'f, Entry<'f>> {
        AstChildren::new(self.cst().children())
    }
}

impl<'f> AstNode<'f> for Number<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<Number<'f>> for CstNode<'f> {
    fn from(ast: Number<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> Number<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<Number<'f>> {
        match node.symbol() {
            NUMBER => Some(Number(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for Bool<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<Bool<'f>> for CstNode<'f> {
    fn from(ast: Bool<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> Bool<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<Bool<'f>> {
        match node.symbol() {
            BOOL => Some(Bool(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for DateTime<'f> {
    fn cast(node: CstNode<'f>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn cst(self) -> CstNode<'f> { self.cst() }
}

impl<'f> From<DateTime<'f>> for CstNode<'f> {
    fn from(ast: DateTime<'f>) -> CstNode<'f> { ast.cst() }
}

impl<'f> DateTime<'f> {
    pub fn cast(node: CstNode<'f>) -> Option<DateTime<'f>> {
        match node.symbol() {
            DATE_TIME => Some(DateTime(node)),
            _ => None,
        }
    }

    pub fn cst(self) -> CstNode<'f> { self.0 }
}
