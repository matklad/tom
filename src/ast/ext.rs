use std::{
    iter,
    borrow::Cow,
};

use ::{
    ast::{
        AstNode, AstChildren,
        KeyVal, Dict, Table, ArrayTable, TableHeader, File, Key, StringLit,
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

impl<'f> KeyValueOwner<'f> for File<'f> {}

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

impl<'f> File<'f> {
    pub fn find_table_by_key(self, key: &str) -> Option<Table<'f>> {
        self.filter_tables(iter::once(key))
            .next()
    }

    pub fn find_table_by_keys(self, keys: &[&str]) -> Option<Table<'f>> {
        self.filter_tables(keys.iter().cloned())
            .next()
    }


    pub fn filter_tables<'a, K>(self, keys: K) -> impl Iterator<Item=Table<'f>>
        where K: Iterator<Item=&'a str> + Clone
    {
        self.tables()
            .filter(move |t| {
                let xs = keys.clone();
                let ys = t.header().keys().map(|key| key.name());
                xs.eq(ys)
            })
    }
}
