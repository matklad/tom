use {
    SyntaxNodeRef, AstNode, AstChildren,
    symbol::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Doc<'a>(SyntaxNodeRef<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Table<'a>(SyntaxNodeRef<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayTable<'a>(SyntaxNodeRef<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableHeader<'a>(SyntaxNodeRef<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry<'a>(SyntaxNodeRef<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key<'a>(SyntaxNodeRef<'a>);

pub enum KeyKind<'a> {
    StringLit(StringLit<'a>),
    BareKey(BareKey<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value<'a>(SyntaxNodeRef<'a>);

pub enum ValueKind<'a> {
    Array(Array<'a>),
    Dict(Dict<'a>),
    Number(Number<'a>),
    Bool(Bool<'a>),
    DateTime(DateTime<'a>),
    StringLit(StringLit<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringLit<'a>(SyntaxNodeRef<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BareKey<'a>(SyntaxNodeRef<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Array<'a>(SyntaxNodeRef<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dict<'a>(SyntaxNodeRef<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number<'a>(SyntaxNodeRef<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bool<'a>(SyntaxNodeRef<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime<'a>(SyntaxNodeRef<'a>);


impl<'a> AstNode<'a> for Doc<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<Doc<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Doc<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> Doc<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Doc> {
        match node.symbol() {
            DOC => Some(Doc(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

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
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<Table<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Table<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> Table<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Table> {
        match node.symbol() {
            TABLE => Some(Table(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

    pub fn header(self) -> TableHeader<'a> {
        AstChildren::new(self.syntax()).next().unwrap()
    }
    pub fn entries(self) -> AstChildren<'a, Entry<'a>> {
        AstChildren::new(self.syntax())
    }
}

impl<'a> AstNode<'a> for ArrayTable<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<ArrayTable<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: ArrayTable<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> ArrayTable<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<ArrayTable> {
        match node.symbol() {
            ARRAY_TABLE => Some(ArrayTable(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

    pub fn header(self) -> TableHeader<'a> {
        AstChildren::new(self.syntax()).next().unwrap()
    }
    pub fn entries(self) -> AstChildren<'a, Entry<'a>> {
        AstChildren::new(self.syntax())
    }
}

impl<'a> AstNode<'a> for TableHeader<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<TableHeader<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: TableHeader<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> TableHeader<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<TableHeader> {
        match node.symbol() {
            TABLE_HEADER => Some(TableHeader(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

    pub fn keys(self) -> AstChildren<'a, Key<'a>> {
        AstChildren::new(self.syntax())
    }
}

impl<'a> AstNode<'a> for Entry<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<Entry<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Entry<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> Entry<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Entry> {
        match node.symbol() {
            ENTRY => Some(Entry(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

    pub fn keys(self) -> AstChildren<'a, Key<'a>> {
        AstChildren::new(self.syntax())
    }
    pub fn value(self) -> Value<'a> {
        AstChildren::new(self.syntax()).next().unwrap()
    }
}

impl<'a> AstNode<'a> for Key<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<Key<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Key<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> Key<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Key> {
        match node.symbol() {
            KEY => Some(Key(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

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
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<Value<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Value<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> Value<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Value> {
        match node.symbol() {
            VALUE => Some(Value(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

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
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<StringLit<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: StringLit<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> StringLit<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<StringLit> {
        match node.symbol() {
            BASIC_STRING => Some(StringLit(node)),
            MULTILINE_BASIC_STRING => Some(StringLit(node)),
            LITERAL_STRING => Some(StringLit(node)),
            MULTILINE_LITERAL_STRING => Some(StringLit(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

    pub fn text(self) -> &'a str {
        self.syntax().leaf_text().unwrap()
    }
}

impl<'a> AstNode<'a> for BareKey<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<BareKey<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: BareKey<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> BareKey<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<BareKey> {
        match node.symbol() {
            BARE_KEY => Some(BareKey(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

    pub fn text(self) -> &'a str {
        self.syntax().leaf_text().unwrap()
    }
}

impl<'a> AstNode<'a> for Array<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<Array<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Array<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> Array<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Array> {
        match node.symbol() {
            ARRAY => Some(Array(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

    pub fn values(self) -> AstChildren<'a, Value<'a>> {
        AstChildren::new(self.syntax())
    }
}

impl<'a> AstNode<'a> for Dict<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<Dict<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Dict<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> Dict<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Dict> {
        match node.symbol() {
            DICT => Some(Dict(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

    pub fn entries(self) -> AstChildren<'a, Entry<'a>> {
        AstChildren::new(self.syntax())
    }
}

impl<'a> AstNode<'a> for Number<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<Number<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Number<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> Number<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Number> {
        match node.symbol() {
            NUMBER => Some(Number(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

    pub fn text(self) -> &'a str {
        self.syntax().leaf_text().unwrap()
    }
}

impl<'a> AstNode<'a> for Bool<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<Bool<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: Bool<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> Bool<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<Bool> {
        match node.symbol() {
            BOOL => Some(Bool(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

    pub fn text(self) -> &'a str {
        self.syntax().leaf_text().unwrap()
    }
}

impl<'a> AstNode<'a> for DateTime<'a> {
    fn cast(node: SyntaxNodeRef<'a>) -> Option<Self> where Self: Sized { Self::cast(node) }
    fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }
}

impl<'a> From<DateTime<'a>> for SyntaxNodeRef<'a> {
    fn from(ast: DateTime<'a>) -> SyntaxNodeRef<'a> { ast.syntax() }
}

impl<'a> DateTime<'a> {
    pub fn cast(node: SyntaxNodeRef<'a>) -> Option<DateTime> {
        match node.symbol() {
            DATE_TIME => Some(DateTime(node)),
            _ => None,
        }
    }

    pub fn syntax(self) -> SyntaxNodeRef<'a> { self.0 }

    pub fn text(self) -> &'a str {
        self.syntax().leaf_text().unwrap()
    }
}
