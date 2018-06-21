use std::{
    borrow::Cow,
};

use ::{
    ast::{self, AstNode, AstChildren},
};

pub trait EntryOwner<'f>: AstNode<'f> {
    fn entries(self) -> AstChildren<'f, ast::Entry<'f>>;
}

pub trait KeyOwner<'f>: AstNode<'f> {
    fn keys(self) -> AstChildren<'f, ast::Key<'f>>;
}

pub trait TableHeaderOwner<'f>: AstNode<'f> {
    fn header(self) -> ast::TableHeader<'f>;
}

impl<'f> ast::Key<'f> {
    pub fn name(self) -> Cow<'f, str> {
        match self.kind() {
            ast::KeyKind::StringLit(lit) => lit.value(),
            ast::KeyKind::BareKey(key) => Cow::from(key.cst().text()),
        }
    }
}

impl<'f> ast::StringLit<'f> {
    pub fn value(self) -> Cow<'f, str> {
        //TODO: broken completely
        let text = self.cst().text();
        let len = text.len();
        Cow::from(&text[1..len - 1])
    }
}

impl<'f> ast::Bool<'f> {
    pub fn value(self) -> bool {
        self.cst().text() == "true"
    }
}

impl<'f> ast::Number<'f> {
    pub fn value(self) -> i64 {
        unimplemented!()
    }
}

impl<'f> ast::DateTime<'f> {
    // chrono?
    pub fn value(self) -> ::std::time::SystemTime {
        unimplemented!()
    }
}


impl<'f> EntryOwner<'f> for ast::Dict<'f> {
    fn entries(self) -> AstChildren<'f, ast::Entry<'f>> { self.entries() }
}

impl<'f> EntryOwner<'f> for ast::Table<'f> {
    fn entries(self) -> AstChildren<'f, ast::Entry<'f>> { self.entries() }
}

impl<'f> EntryOwner<'f> for ast::ArrayTable<'f> {
    fn entries(self) -> AstChildren<'f, ast::Entry<'f>> { self.entries() }
}

impl<'f> EntryOwner<'f> for ast::Doc<'f> {
    fn entries(self) -> AstChildren<'f, ast::Entry<'f>> { self.entries() }
}

impl<'f> KeyOwner<'f> for ast::TableHeader<'f> {
    fn keys(self) -> AstChildren<'f, ast::Key<'f>> { self.keys() }
}

impl<'f> KeyOwner<'f> for ast::Entry<'f> {
    fn keys(self) -> AstChildren<'f, ast::Key<'f>> { self.keys() }
}

impl<'f> TableHeaderOwner<'f> for ast::Table<'f> {
    fn header(self) -> ast::TableHeader<'f> { self.header() }
}

impl<'f> TableHeaderOwner<'f> for ast::ArrayTable<'f> {
    fn header(self) -> ast::TableHeader<'f> { self.header() }
}
