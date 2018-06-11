use *;
use ast::{AstNode, AstChildren};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct File<'f>(TomlNode<'f>);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyVal<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Table<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayTable<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableHeader<'f>(TomlNode<'f>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLit<'f>(TomlNode<'f>);


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key<'f> {
    StringLit(StringLit<'f>),
    BareKey(BareKey<'f>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Val<'f> {
    Array(Array<'f>),
    Dict(Dict<'f>),
    Number(Number<'f>),
    Bool(Bool<'f>),
    DateTime(DateTime<'f>),
    StringLit(StringLit<'f>),
}

impl<'f> AstNode<'f> for File<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if node.symbol() == FILE { Some(File(node)) } else { None }
    }
    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for BareKey<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if node.symbol() == BARE_KEY { Some(BareKey(node)) } else { None }
    }
    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for Array<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if node.symbol() == ARRAY { Some(Array(node)) } else { None }
    }
    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for Dict<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if node.symbol() == DICT { Some(Dict(node)) } else { None }
    }
    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for Number<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if node.symbol() == NUMBER { Some(Number(node)) } else { None }
    }
    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for Bool<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if node.symbol() == BOOL { Some(Bool(node)) } else { None }
    }
    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for DateTime<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if node.symbol() == DATE_TIME { Some(DateTime(node)) } else { None }
    }
    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for KeyVal<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if node.symbol() == KEY_VAL { Some(KeyVal(node)) } else { None }
    }
    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for Table<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if node.symbol() == TABLE { Some(Table(node)) } else { None }
    }
    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for ArrayTable<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if node.symbol() == ARRAY_TABLE { Some(ArrayTable(node)) } else { None }
    }
    fn node(self) -> TomlNode<'f> { self.0 }
}

impl<'f> AstNode<'f> for TableHeader<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if node.symbol() == TABLE_HEADER { Some(TableHeader(node)) } else { None }
    }
    fn node(self) -> TomlNode<'f> { self.0 }
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

impl<'f> AstNode<'f> for Key<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if let Some(n) = StringLit::cast(node) { return Some(Key::StringLit(n)); }
        if let Some(n) = BareKey::cast(node) { return Some(Key::BareKey(n)); }
        None
    }
    fn node(self) -> TomlNode<'f> {
        match self {
            Key::StringLit(n) => n.node(),
            Key::BareKey(n) => n.node(),
        }
    }
}

impl<'f> AstNode<'f> for Val<'f> {
    fn cast(node: TomlNode<'f>) -> Option<Self> where Self: Sized {
        if let Some(n) = Array::cast(node) { return Some(Val::Array(n)); }
        if let Some(n) = Dict::cast(node) { return Some(Val::Dict(n)); }
        if let Some(n) = Number::cast(node) { return Some(Val::Number(n)); }
        if let Some(n) = Bool::cast(node) { return Some(Val::Bool(n)); }
        if let Some(n) = DateTime::cast(node) { return Some(Val::DateTime(n)); }
        if let Some(n) = StringLit::cast(node) { return Some(Val::StringLit(n)); }
        None
    }
    fn node(self) -> TomlNode<'f> {
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

impl<'f> From<File<'f>> for TomlNode<'f> {
    fn from(ast: File<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<BareKey<'f>> for TomlNode<'f> {
    fn from(ast: BareKey<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<Array<'f>> for TomlNode<'f> {
    fn from(ast: Array<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<Dict<'f>> for TomlNode<'f> {
    fn from(ast: Dict<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<Number<'f>> for TomlNode<'f> {
    fn from(ast: Number<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<Bool<'f>> for TomlNode<'f> {
    fn from(ast: Bool<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<DateTime<'f>> for TomlNode<'f> {
    fn from(ast: DateTime<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<KeyVal<'f>> for TomlNode<'f> {
    fn from(ast: KeyVal<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<Table<'f>> for TomlNode<'f> {
    fn from(ast: Table<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<ArrayTable<'f>> for TomlNode<'f> {
    fn from(ast: ArrayTable<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<TableHeader<'f>> for TomlNode<'f> {
    fn from(ast: TableHeader<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<StringLit<'f>> for TomlNode<'f> {
    fn from(ast: StringLit<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<Key<'f>> for TomlNode<'f> {
    fn from(ast: Key<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> From<Val<'f>> for TomlNode<'f> {
    fn from(ast: Val<'f>) -> TomlNode<'f> { ast.node() }
}

impl<'f> File<'f> {
    pub fn tables(self) -> AstChildren<'f, Table<'f>> {
        AstChildren::new(self.node().children())
    }
    pub fn array_tables(self) -> AstChildren<'f, ArrayTable<'f>> {
        AstChildren::new(self.node().children())
    }
}
impl<'f> TableHeader<'f> {
    pub fn keys(self) -> AstChildren<'f, Key<'f>> {
        AstChildren::new(self.node().children())
    }
}
impl<'f> KeyVal<'f> {
    pub fn key(self) -> Key<'f> {
        AstChildren::new(self.node().children()).next().unwrap()
    }
    pub fn val(self) -> Val<'f> {
        AstChildren::new(self.node().children()).next().unwrap()
    }
}
