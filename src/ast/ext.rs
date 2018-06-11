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

pub trait KeyValueOwner<'p>: AstNode<'p> {
    fn entries(self) -> AstChildren<'p, KeyVal<'p>> {
        AstChildren::new(self.node().children())
    }
}

impl<'p> KeyValueOwner<'p> for Dict<'p> {}

impl<'p> KeyValueOwner<'p> for Table<'p> {}

impl<'p> KeyValueOwner<'p> for ArrayTable<'p> {}

impl<'p> KeyValueOwner<'p> for File<'p> {}

pub trait TableHeaderOwner<'p>: AstNode<'p> {
    fn header(self) -> TableHeader<'p> {
        AstChildren::new(self.node().children())
            .next()
            .expect("Table without header")
    }
}

impl<'p> TableHeaderOwner<'p> for Table<'p> {}

impl<'p> TableHeaderOwner<'p> for ArrayTable<'p> {}

impl<'p> Key<'p> {
    fn name(self) -> Cow<'p, str> {
        match self {
            Key::BareKey(bare) => Cow::from(bare.node().text()),
            Key::StringLit(lit) => lit.value(),
        }
    }
}

impl<'p> StringLit<'p> {
    fn value(self) -> Cow<'p, str> {
        //TODO: broken completely
        let text = self.node().text();
        let len = text.len();
        Cow::from(&text[1..len - 1])
    }
}

impl<'p> File<'p> {
    pub fn find_table_by_key(self, key: &str) -> Option<Table<'p>> {
        self.filter_tables(iter::once(key))
            .next()
    }

    pub fn find_table_by_keys(self, keys: &[&str]) -> Option<Table<'p>> {
        self.filter_tables(keys.iter().cloned())
            .next()
    }


    pub fn filter_tables<'a, K>(self, keys: K) -> impl Iterator<Item=Table<'p>>
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
