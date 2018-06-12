use std::{
    borrow::Cow,
};

use ::{
    ast::{
        AstNode, AstChildren,
        KeyVal, Dict, Table, ArrayTable, TableHeader, Doc, Key, StringLit,
    },
};

pub trait KeyValueOwner<'f>: AstNode<'f> {
    fn entries(self) -> AstChildren<'f, KeyVal<'f>> {
        AstChildren::new(self.node().children())
    }
}

impl<'f> KeyValueOwner<'f> for Dict<'f> {}

impl<'f> KeyValueOwner<'f> for Table<'f> {}

impl<'f> KeyValueOwner<'f> for ArrayTable<'f> {}

impl<'f> KeyValueOwner<'f> for Doc<'f> {}

pub trait TableHeaderOwner<'f>: AstNode<'f> {
    fn header(self) -> TableHeader<'f> {
        AstChildren::new(self.node().children())
            .next()
            .expect("Table without header")
    }
}

impl<'f> TableHeaderOwner<'f> for Table<'f> {}

impl<'f> TableHeaderOwner<'f> for ArrayTable<'f> {}

impl<'f> Key<'f> {
    pub fn name(self) -> Cow<'f, str> {
        match self {
            Key::BareKey(bare) => Cow::from(bare.node().text()),
            Key::StringLit(lit) => lit.value(),
        }
    }
}

impl<'f> StringLit<'f> {
    pub fn value(self) -> Cow<'f, str> {
        //TODO: broken completely
        let text = self.node().text();
        let len = text.len();
        Cow::from(&text[1..len - 1])
    }
}
