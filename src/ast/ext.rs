use std::{
    borrow::Cow,
};

use ::{
    ast::{
        AstNode, AstChildren,
        Entry, Dict, Table, ArrayTable, TableHeader, Doc, Key, KeyKind, StringLit,
    },
};

pub trait EntryOwner<'f>: AstNode<'f> {
    fn entries(self) -> AstChildren<'f, Entry<'f>>;
}

pub trait KeyOwner<'f>: AstNode<'f> {
    fn keys(self) -> AstChildren<'f, Key<'f>>;
}

pub trait TableHeaderOwner<'f>: AstNode<'f> {
    fn header(self) -> TableHeader<'f>;
}

impl<'f> Key<'f> {
    pub fn name(self) -> Cow<'f, str> {
        match self.kind() {
            KeyKind::StringLit(lit) => lit.value(),
            KeyKind::BareKey(key) => Cow::from(key.cst().text()),
        }
    }
}

impl<'f> StringLit<'f> {
    pub fn value(self) -> Cow<'f, str> {
        //TODO: broken completely
        let text = self.cst().text();
        let len = text.len();
        Cow::from(&text[1..len - 1])
    }
}

impl<'f> EntryOwner<'f> for Dict<'f> {
    fn entries(self) -> AstChildren<'f, Entry<'f>> { self.entries() }
}

impl<'f> EntryOwner<'f> for Table<'f> {
    fn entries(self) -> AstChildren<'f, Entry<'f>> { self.entries() }
}

impl<'f> EntryOwner<'f> for ArrayTable<'f> {
    fn entries(self) -> AstChildren<'f, Entry<'f>> { self.entries() }
}

impl<'f> EntryOwner<'f> for Doc<'f> {
    fn entries(self) -> AstChildren<'f, Entry<'f>> { self.entries() }
}

impl<'f> KeyOwner<'f> for TableHeader<'f> {
    fn keys(self) -> AstChildren<'f, Key<'f>> { self.keys() }
}

impl<'f> KeyOwner<'f> for Entry<'f> {
    fn keys(self) -> AstChildren<'f, Key<'f>> { self.keys() }
}

impl<'f> TableHeaderOwner<'f> for Table<'f> {
    fn header(self) -> TableHeader<'f> { self.header() }
}

impl<'f> TableHeaderOwner<'f> for ArrayTable<'f> {
    fn header(self) -> TableHeader<'f> { self.header() }
}
