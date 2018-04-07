use ast::{AstNode, AstChildren};
use ast::{KeyVal, Dict, Table, ArrayTable, TableHeader, File};

pub trait KeyValueOwner<'p>: AstNode<'p> {
    fn entries(&self) -> AstChildren<'p, KeyVal<'p>> {
        AstChildren::new(self.node().children())
    }
}

impl<'p> KeyValueOwner<'p> for Dict<'p> {
}

impl<'p> KeyValueOwner<'p> for Table<'p> {
}

impl<'p> KeyValueOwner<'p> for ArrayTable<'p> {
}

impl<'p> KeyValueOwner<'p> for File<'p> {
}

pub trait TableHeaderOwner<'p>: AstNode<'p> {
    fn header(&self) ->  TableHeader<'p> {
        AstChildren::new(self.node().children())
            .next()
            .expect("Table without header")
    }
}

impl<'p> TableHeaderOwner<'p> for Table<'p> {
}

impl<'p> TableHeaderOwner<'p> for ArrayTable<'p> {
}

impl<'p> File<'p> {
    pub fn find_table<S: AsRef<str>>(&self, keys: &[S]) -> Option<Table<'p>> {
        self.tables()
            .find(|t| {
                let xs = keys.iter().map(|s| s.as_ref());
                let ys = t.header().keys().map(|key| key.node().text());
                xs.eq(ys)
            })
    }
}
