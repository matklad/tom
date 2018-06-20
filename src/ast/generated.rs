use *;
use ast::{AstNode, AstChildren};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Doc<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Table<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayTable<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableHeader<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key<'f>(TomlNode<'f>);

pub enum KeyKind<'f> {
    StringLit(StringLit<'f>),
    BareKey(BareKey<'f>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Val<'f>(TomlNode<'f>);

pub enum ValKind<'f> {
    Array(Array<'f>),
    Dict(Dict<'f>),
    Number(Number<'f>),
    Bool(Bool<'f>),
    DateTime(DateTime<'f>),
    StringLit(StringLit<'f>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLit<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BareKey<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Array<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dict<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bool<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime<'f>(TomlNode<'f>);


impl<'f> AstNode<'f> for Doc<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            DOC => Some(Doc(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<Doc<'f>> for TomlNode<'f> {
    fn from(ast: Doc<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> Doc<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }

    pub fn tables(self) -> AstChildren<'f, Table<'f>> {
        AstChildren::new(self.node().children())
    }
    pub fn array_tables(self) -> AstChildren<'f, ArrayTable<'f>> {
        AstChildren::new(self.node().children())
    }
}

impl<'f> AstNode<'f> for Table<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            TABLE => Some(Table(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<Table<'f>> for TomlNode<'f> {
    fn from(ast: Table<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> Table<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for ArrayTable<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            ARRAY_TABLE => Some(ArrayTable(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<ArrayTable<'f>> for TomlNode<'f> {
    fn from(ast: ArrayTable<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> ArrayTable<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for TableHeader<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            TABLE_HEADER => Some(TableHeader(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<TableHeader<'f>> for TomlNode<'f> {
    fn from(ast: TableHeader<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> TableHeader<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }

    pub fn keys(self) -> AstChildren<'f, Key<'f>> {
        AstChildren::new(self.node().children())
    }
}

impl<'f> AstNode<'f> for Entry<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            ENTRY => Some(Entry(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<Entry<'f>> for TomlNode<'f> {
    fn from(ast: Entry<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> Entry<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }

    pub fn keys(self) -> AstChildren<'f, Key<'f>> {
        AstChildren::new(self.node().children())
    }
    pub fn val(self) -> Val<'f> {
        AstChildren::new(self.node().children()).next().unwrap()
    }
}

impl<'f> AstNode<'f> for Key<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            KEY => Some(Key(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<Key<'f>> for TomlNode<'f> {
    fn from(ast: Key<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> Key<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }

    pub fn kind(self) -> KeyKind<'f> {
        let node = self.node().children().next().unwrap();
        if let Some(node) = StringLit::cast(node) {
            return KeyKind::StringLit(node);
        }
        if let Some(node) = BareKey::cast(node) {
            return KeyKind::BareKey(node);
        }
        unreachable!()
    }

}

impl<'f> AstNode<'f> for Val<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            VAL => Some(Val(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<Val<'f>> for TomlNode<'f> {
    fn from(ast: Val<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> Val<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }

    pub fn kind(self) -> ValKind<'f> {
        let node = self.node().children().next().unwrap();
        if let Some(node) = Array::cast(node) {
            return ValKind::Array(node);
        }
        if let Some(node) = Dict::cast(node) {
            return ValKind::Dict(node);
        }
        if let Some(node) = Number::cast(node) {
            return ValKind::Number(node);
        }
        if let Some(node) = Bool::cast(node) {
            return ValKind::Bool(node);
        }
        if let Some(node) = DateTime::cast(node) {
            return ValKind::DateTime(node);
        }
        if let Some(node) = StringLit::cast(node) {
            return ValKind::StringLit(node);
        }
        unreachable!()
    }

}

impl<'f> AstNode<'f> for StringLit<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            BASIC_STRING => Some(StringLit(node)),
            MULTILINE_BASIC_STRING => Some(StringLit(node)),
            LITERAL_STRING => Some(StringLit(node)),
            MULTILINE_LITERAL_STRING => Some(StringLit(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<StringLit<'f>> for TomlNode<'f> {
    fn from(ast: StringLit<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> StringLit<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for BareKey<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            BARE_KEY => Some(BareKey(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<BareKey<'f>> for TomlNode<'f> {
    fn from(ast: BareKey<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> BareKey<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for Array<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            ARRAY => Some(Array(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<Array<'f>> for TomlNode<'f> {
    fn from(ast: Array<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> Array<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for Dict<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            DICT => Some(Dict(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<Dict<'f>> for TomlNode<'f> {
    fn from(ast: Dict<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> Dict<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for Number<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            NUMBER => Some(Number(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<Number<'f>> for TomlNode<'f> {
    fn from(ast: Number<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> Number<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for Bool<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            BOOL => Some(Bool(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<Bool<'f>> for TomlNode<'f> {
    fn from(ast: Bool<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> Bool<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for DateTime<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        match node.symbol() {
            DATE_TIME => Some(DateTime(node)),
            _ => None,
        }
    }

    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> From<DateTime<'f>> for TomlNode<'f> {
    fn from(ast: DateTime<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> DateTime<'f> {
    pub fn node(self) -> TomlNode<'f> { self.0 }
}
